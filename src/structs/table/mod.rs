// A structured representation of a DataFlex table file

// Submodule Declarations
pub mod header;
pub mod row;

pub use header::Header;
pub use row::Row;

// Standard Library Imports
use std::fmt;
use std::iter::IntoIterator;
use std::ops::Index as Indexable;

// Third-Party Imports
use prettytable::Table as PrettyTable; // Cell, Row as PrintableRow,
use pyo3::exceptions::{PyIndexError, PyNotImplementedError};
use pyo3::prelude::*;
use pyo3::types::{PySliceIndices, PyTuple};
use serde::{Deserialize, Serialize};

// Crate-Level Imports
use crate::enums::{DataType, Value, Version};
use crate::exceptions::{InternalError, NotSupportedError};
use crate::utils::{
    bytes_from_file, date_from_bytes, float_from_bcd_bytes, int_from_bcd_bytes, string_from_bytes,
};
use crate::{iif, AttrIndexSliceOrItem, ValueOrSlice};

// <editor-fold desc="// DataFlexTable ...">

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, PartialOrd, Serialize)]
#[pyclass(dict, module = "ferroflex.structs")]
/// A structured representation of a DataFlex table file
pub struct DataFlexTable {
    #[pyo3(get)]
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
            self.header.file_root_name, self.header.record_count, self.header.version,
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

        let record_number: i64 = if record_number < 0i64 {
            self.header.record_count as i64 + record_number
        } else {
            record_number
        };

        if record_number < 0i64 || record_number > self.header.record_count as i64 {
            return Err(PyIndexError::new_err(""));
        }

        // TODO: Update this to behave properly for tables that have fill bytes
        if self.header.fill_bytes_per_block > 0 {
            panic!()
        }

        // Calculate the starting offset of the requested
        // record depending on the table version
        let start: u64 = match self.header.version {
            Version::V23B => 512i64 + (self.header.record_length as i64 * record_number),
            Version::V30 => 3072i64 + (self.header.record_length as i64 * record_number),
            Version::Unknown => {
                return Err(NotSupportedError::new_err("Unsupported table format!"));
            }
        } as u64;

        let end: u64 = start + self.header.record_length as u64;

        bytes_from_file(&self.header.filepath, Some(start), Some(end))
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
                        Ok(Some(val)) => Value::Date(val),
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

    pub fn schema(&self) -> PyResult<Option<!>> {
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

#[allow(unused_mut, unused_variables)]
#[pymethods]
impl DataFlexTable {
    // <editor-fold desc="// Magic Methods ...">

    #[new]
    fn __new__(filepath: String) -> PyResult<Self> {
        Self::from_path(AsRef::<str>::as_ref(&filepath))
    }

    fn __str__(slf: PyRef<Self>) -> String {
        format!("{}", *slf)
    }

    fn __repr__(slf: PyRef<Self>) -> String {
        format!("{}", *slf)
    }

    fn __len__(slf: PyRef<Self>) -> usize {
        slf.len() as usize
    }

    fn __getitem__(
        slf: PyRef<Self>,
        key: AttrIndexSliceOrItem<Row>,
    ) -> PyResult<ValueOrSlice<PyObject>> {
        Python::with_gil(|py| match key {
            // When an actual Row is supplied as the "needle"
            AttrIndexSliceOrItem::Item(_) => Err(PyNotImplementedError::new_err("")),
            // When an "attribute name" is supplied as the "needle"
            AttrIndexSliceOrItem::Name(_) => Err(PyNotImplementedError::new_err("")),
            // When a specific row number is supplied as the "needle"
            AttrIndexSliceOrItem::Index(idx) => Ok(ValueOrSlice::Value(Row::to_object(
                &(slf.nth_record(idx as i64)?),
                py,
            ))),
            // When a "range" of row numbers is supplied as the "needle"
            AttrIndexSliceOrItem::Slice(slc) => {
                let indexes: PySliceIndices = slc.indices(3)?;

                let (start, end) = (indexes.start as i64, indexes.stop as i64);

                let end: i64 = iif!(end > -1, end, slf.len() as i64 + end);
                let start: i64 = iif!(start > -1, start, slf.len() as i64 + start);

                if start < 0 || end < 0 {
                    return Err(PyIndexError::new_err(""));
                }

                let mut data = Vec::<PyObject>::new();

                for idx in start..end {
                    match slf.nth_record(idx) {
                        Ok(row) => data.push(Row::to_object(&row, py)),
                        Err(_) => {
                            return Err(InternalError::new_err(format!(
                                "Could not read data for row {} from '{}'!",
                                idx,
                                (slf.header.file_root_name).as_str()
                            )));
                        }
                    }
                }

                Ok(ValueOrSlice::Slice(data))
            }
        })
    }

    fn __setitem__(
        mut slf: PyRefMut<Self>,
        key: AttrIndexSliceOrItem<Row>,
        value: &PyAny,
    ) -> PyResult<()> {
        todo!()
    }

    fn __delitem__(mut slf: PyRefMut<Self>, index: AttrIndexSliceOrItem<Row>) -> PyResult<()> {
        todo!()
    }

    fn __iter__(slf: PyRef<Self>) -> () {
        todo!()
    }

    fn __reversed__(slf: PyRef<Self>) -> () {
        todo!()
    }

    fn __contains__(slf: PyRef<Self>, record: AttrIndexSliceOrItem<Row>) -> bool {
        todo!()
    }

    // </editor-fold desc="// Magic Methods ...">

    // <editor-fold desc="// Getter/Setter Methods ...">

    // </editor-fold desc="// Getter/Setter Methods ...">

    // <editor-fold desc="// Instance Methods ...">

    fn pretty(slf: PyRef<Self>) -> String {
        slf._as_pretty_table().to_string()
    }

    fn index(slf: PyRef<Self>, record: &PyAny) -> PyResult<i32> {
        todo!()
    }

    fn pop(mut slf: PyRefMut<Self>, index: i64) -> PyResult<()> {
        todo!()
    }

    fn insert(mut slf: PyRefMut<Self>, record: &PyAny) -> PyResult<()> {
        todo!()
    }

    fn append(mut slf: PyRefMut<Self>, record: &PyAny) -> PyResult<()> {
        todo!()
    }

    fn extend(mut slf: PyRefMut<Self>, records: &PyTuple) -> PyResult<()> {
        todo!()
    }

    fn remove(mut slf: PyRefMut<Self>, record: &PyAny) -> PyResult<()> {
        todo!()
    }

    // </editor-fold desc="// Instance Methods ...">
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
