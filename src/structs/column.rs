// A structured representation of a column's definition in the header of a DataFlex table file

// Standard Library Imports
use std::cmp::min;
use std::fmt;

// Third-Party Imports
use byteorder::{ByteOrder, LittleEndian};
use prettytable::{Cell as PrettyCell, Row as PrettyRow, Table as PrettyTable};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyType};
use serde::{Deserialize, Serialize};

// Crate-Level Imports
use crate::enums::DataType;
use crate::iif;

// <editor-fold desc="// Column ...">

#[derive(Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq, Serialize, Deserialize)]
#[pyclass(dict, module = "ferroflex.structs")]
/// A structured representation of a column's definition
/// in the header of a DataFlex table file
pub struct Column {
    #[pyo3(get)]
    /// The column's human-readable name
    pub name: String,
    #[pyo3(get)]
    /// The number of bytes between the
    /// first byte of a record and the first
    /// byte of the column's data
    pub offset: u64,
    #[pyo3(get)]
    /// The numerical "id" of the column's
    /// "primary" index (as defined in the
    /// header of the table to which the
    /// column belongs)
    pub main_index: Option<u64>,
    #[pyo3(get)]
    /// The number of digits to the
    /// right of the decimal (if the
    /// column represents a numerical
    /// data type)
    pub decimal_points: u64,
    #[pyo3(get)]
    /// The total number of bytes occupied
    /// by the column's data with respect
    /// to a single row in a given table
    pub length: u64,
    #[pyo3(get)]
    /// The "type" of data stored in /
    /// represented by the column
    pub data_type: DataType,
    #[pyo3(get)]
    /// The numerical "id" of the table
    /// holding the "remote" column to
    /// which the column is a foreign key
    /// (as defined in the `filelist.cfg`
    /// associated with the table to which
    /// the column belongs)
    pub related_file: Option<u64>,
    #[pyo3(get)]
    /// The numerical "id" of the "remote"
    /// column on another table to which
    /// the column is a foreign key (as
    /// defined in the header of the table
    /// to which the "remote" column belongs)
    pub related_field: Option<u64>,
}

unsafe impl Send for Column {}

impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Column<name: '{}' | type: {} | offset: {} | length: {}>",
            self.name, self.data_type, self.offset, self.length,
        )
    }
}

impl Column {
    // <editor-fold desc="// 'Private' Methods ...">

    pub(crate) fn _as_pretty_table(&self) -> String {
        let mut table = PrettyTable::new();

        vec![
            ("name", (&self.name).to_string()),
            ("type", (&self.data_type).to_string()),
            ("offset", (&self.offset).to_string()),
            ("length", (&self.length).to_string()),
            ("scale", (&self.decimal_points).to_string()),
            (
                "main_index",
                match &self.main_index {
                    None => "N/A".to_string(),
                    Some(idx) => idx.to_string(),
                },
            ),
            (
                "related_file",
                match &self.related_file {
                    None => "N/A".to_string(),
                    Some(file) => file.to_string(),
                },
            ),
            (
                "related_field",
                match &self.related_field {
                    None => "N/A".to_string(),
                    Some(field) => field.to_string(),
                },
            ),
        ]
        .iter()
        .for_each(|(key, value)| {
            table.add_row(PrettyRow::from(vec![
                PrettyCell::new(key),
                PrettyCell::new(value),
            ]));
        });

        table.to_string()
    }

    // </editor-fold desc="// 'Private' Methods ...">

    // <editor-fold desc="// Public Methods ...">

    pub fn from_bytes(data: &[u8], name: Option<&str>) -> PyResult<Py<Column>> {
        let decimal_points: u64 = iif!(data[4] == 1, data[2] & 0x0F, 0u8) as u64;

        Python::with_gil(|py| {
            Py::new(
                py,
                Column {
                    decimal_points,
                    length: data[3] as u64,
                    name: name.unwrap_or("").to_string(),
                    offset: LittleEndian::read_u16(&data[..2]) as u64,
                    data_type: match data[4] {
                        0 => DataType::Ascii,
                        1 => {
                            if decimal_points > 0 {
                                DataType::Float
                            } else {
                                DataType::Int
                            }
                        }
                        2 => DataType::Date,
                        // 3 => DataType::Overlap,
                        5 => DataType::Text,
                        _ => DataType::Binary,
                    },
                    main_index: {
                        let idx = data[2] >> 4 & 0x0F;

                        if idx > 0 {
                            Some(idx as u64)
                        } else {
                            None
                        }
                    },
                    related_file: if data[5] > 0 {
                        Some(data[5] as u64)
                    } else {
                        None
                    },
                    related_field: {
                        let field = LittleEndian::read_u16(&data[6..]);

                        if field > 0 {
                            Some(field as u64)
                        } else {
                            None
                        }
                    },
                },
            )
        })
    }

    pub fn table_from_bytes(data: &[u8], names: Option<Vec<String>>) -> PyResult<Vec<Py<Column>>> {
        let chunks = data.chunks_exact(8);

        if let Some(n) = names {
            return Ok(chunks
                .enumerate()
                .filter(|pair| pair.0 < n.len())
                .map(|pair| Column::from_bytes(pair.1, Some(&n[min(pair.0, n.len() - 1)])))
                .filter(PyResult::is_ok)
                .map(PyResult::unwrap)
                .collect::<Vec<Py<Column>>>());
        }

        Ok(chunks
            .map(|val| Column::from_bytes(val, None))
            .filter(PyResult::is_ok)
            .map(PyResult::unwrap)
            .collect::<Vec<Py<Column>>>())
    }

    // </editor-fold desc="// Public Methods ...">
}

#[pymethods]
impl Column {
    #[new]
    fn __new__(
        name: String,
        offset: u64,
        length: u64,
        data_type: &PyType,
        py_kwargs: Option<&PyDict>,
    ) -> PyResult<Self> {
        let (main_index, related_file, related_field, decimal_points) = match py_kwargs {
            Some(kwargs) => (
                match kwargs.get_item("main_index") {
                    Some(val) => match val.extract::<u64>() {
                        Ok(val) => Some(val),
                        Err(_) => None,
                    },
                    None => None,
                },
                match kwargs.get_item("related_file") {
                    Some(val) => match val.extract::<u64>() {
                        Ok(val) => Some(val),
                        Err(_) => None,
                    },
                    None => None,
                },
                match kwargs.get_item("related_field") {
                    Some(val) => match val.extract::<u64>() {
                        Ok(val) => Some(val),
                        Err(_) => None,
                    },
                    None => None,
                },
                match kwargs.get_item("decimal_points") {
                    Some(val) => val.extract::<u64>().unwrap_or(0u64),
                    None => 0u64,
                },
            ),
            None => (None, None, None, 0u64),
        };

        Ok(Self {
            name,
            offset,
            length,
            data_type: DataType::extract(data_type)?,
            main_index,
            related_file,
            related_field,
            decimal_points,
        })
    }

    fn __str__(slf: PyRef<Self>) -> PyResult<String> {
        Ok(format!("{}", *slf))
    }

    fn __repr__(slf: PyRef<Self>) -> PyResult<String> {
        Ok(format!("{}", *slf))
    }

    fn pretty(slf: PyRef<Self>) -> String {
        slf._as_pretty_table()
    }
}

// </editor-fold desc="// Column ...">

// <editor-fold desc="// Tests ...">

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::Column;

    #[test]
    /// Test that the `Column` structure correctly handles table data
    fn gets_columns() {
        todo!()
    }
}

// </editor-fold desc="// Tests ...">
