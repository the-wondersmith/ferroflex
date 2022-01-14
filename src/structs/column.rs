// A structured representation of a column's definition in the header of a DataFlex table file

// Standard Library Imports
use std::cmp::min;
use std::fmt;

// Third-Party Imports
use byteorder::{ByteOrder, LittleEndian};
use gluesql::core::ast::ColumnDef; // {ColumnOption, ColumnOptionDef}
use gluesql::core::sqlparser::ast::DataType;
use prettytable::{Cell, Row as PrintableRow, Table as PrettyTable};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyType};
use serde::{Deserialize, Serialize};

// Crate-Level Imports
use crate::iif;

// <editor-fold desc="// Helpers ...">

fn pytype_to_sqltype(obj: Option<&PyType>) -> PyResult<DataType> {
    if let Some(pytype) = obj {
        let pytype: &str = pytype.name()?;

        return Ok(match pytype.to_lowercase().as_str() {
            "int" => DataType::Int(None),
            "str" => DataType::Text,
            "float" => DataType::Float(None),
            "date" => DataType::Date,
            "bytes" | "bytearray" => DataType::Varbinary(u64::MAX),
            _ => {
                return Err(PyValueError::new_err(""));
            }
        });
    }

    Err(PyValueError::new_err(""))
}

// </editor-fold desc="// Helpers ...">

// <editor-fold desc="// Column ...">

#[derive(Clone, Debug, Serialize, Deserialize)]
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
    pub offset: u32,
    #[pyo3(get)]
    /// The numerical "id" of the column's
    /// "primary" index (as defined in the
    /// header of the table to which the
    /// column belongs)
    pub main_index: Option<u8>,
    #[pyo3(get)]
    /// The number of digits to the
    /// right of the decimal (if the
    /// column represents a numerical
    /// data type)
    pub decimal_points: u8,
    #[pyo3(get)]
    /// The total number of bytes occupied
    /// by the column's data with respect
    /// to a single row in a given table
    pub length: u32,
    // #[pyo3(get)]
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
    pub related_file: Option<u8>,
    #[pyo3(get)]
    /// The numerical "id" of the "remote"
    /// column on another table to which
    /// the column is a foreign key (as
    /// defined in the header of the table
    /// to which the "remote" column belongs)
    pub related_field: Option<u8>,
}

unsafe impl Send for Column {}

impl Default for Column {
    fn default() -> Self {
        Column {
            name: "".to_string(),
            offset: 0,
            main_index: None,
            decimal_points: 0,
            length: 0,
            data_type: DataType::Binary(0u64),
            related_file: None,
            related_field: None,
        }
    }
}

impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Column<name: '{}' | type: {:?} | offset: {} | length: {}>",
            self.name, self.data_type, self.offset, self.length,
        )
    }
}

impl Column {
    // <editor-fold desc="// 'Private' Methods ...">

    fn _field_values(&self) -> Vec<(String, String)> {
        vec![
            ("offset".to_string(), format!("{}", self.offset)),
            (
                "main_index".to_string(),
                match &self.main_index {
                    None => "None".to_string(),
                    Some(val) => val.to_string(),
                },
            ),
            (
                "decimal_points".to_string(),
                format!("{}", self.decimal_points),
            ),
            ("length".to_string(), format!("{}", self.length)),
            ("data_type".to_string(), format!("{:?}", self.data_type)),
            (
                "related_file".to_string(),
                match &self.related_file {
                    None => "None".to_string(),
                    Some(val) => val.to_string(),
                },
            ),
            (
                "related_field".to_string(),
                match &self.related_field {
                    None => "None".to_string(),
                    Some(val) => val.to_string(),
                },
            ),
        ]
    }

    fn _as_pretty_table(&self) -> PrettyTable {
        let (mut outer, mut inner) = (PrettyTable::new(), PrettyTable::new());

        self._field_values().iter().for_each(|pair| {
            inner.add_row(PrintableRow::new(vec![
                Cell::new(&pair.0),
                Cell::new(&pair.1),
            ]));
        });

        outer.add_row(PrintableRow::new(vec![Cell::new(&self.name)]));
        outer.add_row(PrintableRow::new(vec![Cell::new(&inner.to_string())]));

        outer
    }

    // </editor-fold desc="// 'Private' Methods ...">

    // <editor-fold desc="// Public Methods ...">

    pub fn definition(&self) -> PyResult<ColumnDef> {
        // ColumnOptionDef {
        //     name: Option<String>,
        //     option: ColumnOption,
        // }
        //
        // ColumnOption {
        //     /// `NULL`
        //     Null,
        //     /// `NOT NULL`
        //     NotNull,
        //     /// `DEFAULT <restricted-expr>`
        //     Default(Expr),
        //     /// `{ PRIMARY KEY | UNIQUE }`
        //     Unique { is_primary: bool },
        // }

        Ok(ColumnDef {
            name: self.name.clone(),
            data_type: match self.data_type {
                DataType::Int(_) => gluesql::core::ast::DataType::Int,
                DataType::Float(_) => gluesql::core::ast::DataType::Float,
                DataType::Char(_) | DataType::Text => gluesql::core::ast::DataType::Text,
                _ => gluesql::core::ast::DataType::List,
            },
            options: Vec::new(), // TODO: Implement `ColumnOptionDef` support
        })
    }

    pub fn from_bytes(data: &[u8], name: Option<&str>) -> PyResult<Column> {
        let decimal_points: u8 = iif!(data[4] == 1, data[2] & 0x0F, 0u8) as u8;

        Ok(Column {
            name: name.unwrap_or("").to_string(),
            offset: LittleEndian::read_u16(&data[..2]) as u32,
            main_index: Some((data[2] >> 4 & 0x0F) as u8),
            decimal_points,
            length: data[3] as u32,
            data_type: match data[4] {
                0 => DataType::Char(Some(data[4] as u64)),
                1 => {
                    if decimal_points > 0 {
                        DataType::Float(Some((&decimal_points * 2) as u64))
                    } else {
                        DataType::Int(None)
                    }
                }
                2 => DataType::Date,
                5 => DataType::Text,
                // 3 => DataType::Overlap,
                _ => DataType::Binary(0u64), // Should be treated as a stand-in for Binary (for now)
            },
            related_file: Some(data[5] as u8),
            related_field: Some(LittleEndian::read_u16(&data[6..]) as u8),
        })
    }

    pub fn table_from_bytes(data: &[u8], names: Option<Vec<String>>) -> PyResult<Vec<Column>> {
        let chunks = data.chunks_exact(8);

        if let Some(n) = names {
            return Ok(chunks
                .enumerate()
                .filter(|pair| pair.0 < n.len())
                .map(|pair| Column::from_bytes(pair.1, Some(&n[min(pair.0, n.len() - 1)])).unwrap())
                .collect::<Vec<Column>>());
        }

        Ok(chunks
            .map(|val| Column::from_bytes(val, None).unwrap())
            .collect::<Vec<Column>>())
    }

    // </editor-fold desc="// Public Methods ...">
}

#[pymethods]
impl Column {
    #[new]
    #[args(
        name = "None",
        offset = "None",
        length = "None",
        data_type = "None",
        py_kwargs = "**"
    )]
    fn __new__(
        name: Option<String>,
        offset: Option<u32>,
        length: Option<u32>,
        data_type: Option<&PyType>,
        py_kwargs: Option<&PyDict>,
    ) -> PyResult<Self> {
        let (main_index, related_file, related_field, decimal_points) = match py_kwargs {
            Some(kwargs) => (
                match kwargs.get_item("main_index") {
                    Some(val) => match val.extract::<u8>() {
                        Ok(val) => Some(val),
                        Err(_) => None,
                    },
                    None => None,
                },
                match kwargs.get_item("related_file") {
                    Some(val) => match val.extract::<u8>() {
                        Ok(val) => Some(val),
                        Err(_) => None,
                    },
                    None => None,
                },
                match kwargs.get_item("related_field") {
                    Some(val) => match val.extract::<u8>() {
                        Ok(val) => Some(val),
                        Err(_) => None,
                    },
                    None => None,
                },
                match kwargs.get_item("decimal_points") {
                    Some(val) => val.extract::<u8>().unwrap_or(0u8),
                    None => 0u8,
                },
            ),
            None => (None, None, None, 0u8),
        };

        Ok(Self {
            name: name.unwrap_or_default(),
            offset: offset.unwrap_or_default(),
            length: length.unwrap_or_default(),
            data_type: pytype_to_sqltype(data_type)?,
            main_index,
            related_file,
            related_field,
            decimal_points,
        })
    }

    fn __str__(slf: PyRefMut<Self>) -> PyResult<String> {
        Ok(format!("{}", *slf))
    }

    fn __repr__(slf: PyRefMut<Self>) -> PyResult<String> {
        Ok(format!("{}", *slf))
    }

    fn pretty(slf: PyRefMut<Self>) -> String {
        slf._as_pretty_table().to_string()
    }

    #[getter(data_type)]
    fn get_data_type(slf: PyRefMut<Self>) -> PyResult<String> {
        Ok(match slf.data_type {
            DataType::Char(length) => format!("CHAR({})", length.unwrap_or(u64::MAX)), // pyo3::types::PyString
            DataType::Int(_) => "INTEGER".to_string(), // pyo3::types::PyInt
            DataType::Float(length) => format!("DECIMAL({})", length.unwrap_or(0u64)), // pyo3::types::PyFloat
            DataType::Date => "DATE".to_string(), // pyo3::types::PyDate
            DataType::Text => "TEXT".to_string(),
            _ => "UNKNOWN".to_string(),
        })
    }
}

// </editor-fold desc="// Column ...">
