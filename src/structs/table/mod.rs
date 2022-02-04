// A structured representation of a DataFlex table file

// Submodule Declarations
pub mod header;

pub use header::Header;

// Standard Library Imports
use std::fmt;
use std::iter::IntoIterator;
use std::ops::Index as Indexable;

// Third-Party Imports
use gluesql::core::data::{Row, Value};
use prettytable::Table as PrettyTable; // Cell, Row as PrintableRow,
use pyo3::exceptions::PyIndexError;
use pyo3::PyResult;
use serde::{Deserialize, Serialize};

// Crate-Level Imports
use crate::enums::{DataType, Version};
use crate::exceptions::NotSupportedError;
use crate::utils::{
    bytes_from_file, date_from_bytes, float_from_bcd_bytes, int_from_bcd_bytes, string_from_bytes,
};

// <editor-fold desc="// DataFlexTable ...">

#[derive(Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq, Serialize, Deserialize)]
/// A structured representation of a DataFlex table file
pub struct DataFlexTable {
    /// The table's header data
    pub header: Header,
}

unsafe impl Send for DataFlexTable {}

impl<T: Into<i64>> Indexable<T> for DataFlexTable {
    type Output = PyResult<Row>;

    #[allow(unused_variables)]
    fn index(&self, index: T) -> &'static Self::Output {
        todo!()
    }
}

impl IntoIterator for DataFlexTable {
    type Item = Row;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

impl fmt::Display for DataFlexTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "DataFlexTable<name: {} | record_count: {} | df_version: {}>",
            &self.header.file_root_name, &self.header.record_count, &self.header.version,
        )
    }
}

impl DataFlexTable {
    // <editor-fold desc="// 'Private' Methods ...">

    fn _as_pretty_table(&self) -> PrettyTable {
        todo!()
    }

    fn nth_record_bytes<I: Into<i64>>(&self, record_number: I) -> PyResult<Vec<u8>> {
        let record_number: i64 = record_number.into();

        let header = &self.header;

        let record_number: i64 = if record_number < 0i64 {
            header.record_count as i64 + record_number
        } else {
            record_number
        };

        if record_number < 0i64 || record_number > header.record_count as i64 {
            return Err(PyIndexError::new_err(""));
        }

        // TODO: Update this to behave properly for tables that have fill bytes
        if header.fill_bytes_per_block > 0 {
            panic!()
        }

        // Calculate the starting offset of the requested
        // record depending on the table version
        let start: u64 = match header.version {
            Version::V23B => 512i64 + (header.record_length as i64 * record_number),
            Version::V30 => 3072i64 + (header.record_length as i64 * record_number),
            Version::Unknown => {
                return Err(NotSupportedError::new_err("Unsupported table format!"));
            }
        } as u64;

        let end: u64 = start + header.record_length as u64;

        bytes_from_file(&header.filepath, Some(start), Some(end))
    }

    fn record_from_bytes<B: AsRef<[u8]>>(&self, record_data: B) -> PyResult<Row> {
        let record_data: &[u8] = record_data.as_ref();

        Ok(Row(self
            .header
            .columns
            .iter()
            .map(|col| {
                let start = (col.offset - 1) as usize;
                let end = (col.length as usize) + start;
                let data = &record_data[start..end];

                match col.data_type {
                    DataType::Ascii => Value::Str(string_from_bytes(data, Some(false)).unwrap()),
                    DataType::Int => Value::I64(int_from_bcd_bytes(data, Some(true)).unwrap()),
                    DataType::Float => {
                        Value::F64(float_from_bcd_bytes(data, Some(col.decimal_points)).unwrap())
                    }
                    DataType::Date => match date_from_bytes(data) {
                        Ok(Some(val)) => Value::Date(val.0),
                        _ => Value::Null,
                    },
                    // The first two bytes of TEXT and BINARY fields are actually
                    // a u16 integer denoting how much of the field's allotted
                    // length is actually "populated"
                    DataType::Text => Value::Str(string_from_bytes(data, Some(true)).unwrap()),
                    // `gluesql` doesn't currently support Binary / BLOB types
                    _ => Value::Null,
                    // data[2..][..LittleEndian::read_u16(&data[..2]) as usize].to_vec()
                }
            })
            .collect::<Vec<Value>>()))
    }

    // </editor-fold desc="// 'Private' Methods ...">

    // <editor-fold desc="// Public Methods ...">

    pub fn len(&self) -> u64 {
        self.header.record_count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> () {
        todo!()
    }

    pub fn schema(&self) -> PyResult<Option<()>> {
        todo!()
    }

    pub fn from_path<P: AsRef<str>>(table_path: P) -> PyResult<DataFlexTable> {
        Ok(DataFlexTable {
            header: Header::from_path(table_path.as_ref())?,
        })
    }

    pub fn nth_record<I: Into<i64>>(&self, record_number: I) -> PyResult<Row> {
        let record_number: i64 = record_number.into();

        let record_number: i64 = if record_number > -1i64 {
            record_number
        } else {
            self.header.record_count as i64 + record_number
        };

        if record_number < 0i64 {
            return Err(PyIndexError::new_err(""));
        }

        let record_number: u32 = match u32::try_from(record_number) {
            Ok(value) => value,
            Err(_) => return Err(PyIndexError::new_err("")),
        };

        let data = self.nth_record_bytes(record_number)?;

        self.record_from_bytes(&data)
    }

    #[allow(unused_variables)]
    pub fn append_record(&self, record: Row) -> PyResult<()> {
        todo!()
    }

    #[allow(unused_variables)]
    pub fn update_record<I: Into<i64>>(&self, record_number: I, record: Row) -> PyResult<()> {
        todo!()
    }

    // </editor-fold desc="// Public Methods ...">
}

// </editor-fold desc="// DataFlexTable ...">

// <editor-fold desc="// Tests ...">

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::DataFlexTable;

    #[test]
    /// Test that the `DataFlexTable` structure correctly gets rows
    fn gets_rows() {
        todo!()
    }
}

// </editor-fold desc="// Tests ...">
