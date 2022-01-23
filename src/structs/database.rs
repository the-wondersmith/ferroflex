// A structured representation of a collection of DataFlex table files

// Standard Library Imports
use std::fmt;
use std::iter::IntoIterator;
use std::ops::Index as Indexable;
use std::path::PathBuf;

// Third-Party Imports
#[allow(unused_imports)]
use prettytable::{Cell as PrettyCell, Row as PrettyRow, Table as PrettyTable};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

// Crate-Level Imports
use crate::structs::{DataFlexTable, FileList};
use crate::{AttrIndexSliceOrItem, ValueOrSlice};

// <editor-fold desc="// DataFlexDB ...">

#[derive(Debug, Serialize, Deserialize)]
#[pyclass(dict, module = "ferroflex.structs")]
/// A structured representation of a collection of DataFlex table files
pub struct DataFlexDB {
    #[pyo3(get)]
    /// The db's on-disk path
    pub db_path: PathBuf,
    #[pyo3(get)]
    /// The db's filelist
    pub filelist: Py<FileList>,
    #[pyo3(get)]
    /// The db's filelist
    pub tables: Vec<Py<DataFlexTable>>,
}

unsafe impl Send for DataFlexDB {}

#[allow(unused_variables)]
impl<T: Into<i64>> Indexable<T> for DataFlexDB {
    type Output = PyResult<DataFlexTable>;

    fn index(&self, index: T) -> &'static Self::Output {
        todo!()
    }
}

impl IntoIterator for DataFlexDB {
    type Item = Py<DataFlexTable>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.tables.into_iter()
    }
}

#[allow(unused_variables)]
impl fmt::Display for DataFlexDB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!()
    }
}

#[allow(unused_variables)]
impl DataFlexDB {
    // <editor-fold desc="// 'Private' Methods ...">

    pub(crate) fn _as_pretty_table(&self) -> String {
        todo!()
    }

    // </editor-fold desc="// 'Private' Methods ...">

    // <editor-fold desc="// Public Methods ...">

    pub fn len(&self) -> usize {
        Python::with_gil(|py| Py::borrow(&self.filelist, py).len())
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> std::vec::IntoIter<DataFlexTable> {
        todo!()
    }

    pub fn get(
        &'fl self,
        table: AttrIndexSliceOrItem<DataFlexTable>,
    ) -> Option<&'fl DataFlexTable> {
        todo!()
    }

    pub fn contains(&self, table: AttrIndexSliceOrItem<DataFlexTable>) -> bool {
        todo!()
    }

    pub fn from_path<P: AsRef<str>>(db_path: P) -> PyResult<DataFlexDB> {
        todo!()
    }

    pub fn create_table(&self) -> PyResult<()> {
        todo!()
    }

    pub fn alter_table(&self) -> PyResult<()> {
        todo!()
    }

    pub fn drop_table(&self) -> PyResult<()> {
        todo!()
    }

    // </editor-fold desc="// Public Methods ...">
}

#[allow(unused_mut, unused_variables)]
#[pymethods]
impl DataFlexDB {
    #[new]
    fn __new__(filepath: String) -> PyResult<Self> {
        Self::from_path(AsRef::<str>::as_ref(&filepath))
    }

    fn __str__(slf: PyRef<Self>) -> PyResult<String> {
        Ok(format!("{}", *slf))
    }

    fn __repr__(slf: PyRef<Self>) -> PyResult<String> {
        Ok(format!("{}", *slf))
    }

    fn __len__(slf: PyRef<Self>) -> usize {
        slf.len() as usize
    }

    fn __getitem__(
        slf: PyRef<Self>,
        key: AttrIndexSliceOrItem<Option<isize>>,
    ) -> PyResult<ValueOrSlice<DataFlexTable>> {
        todo!()
    }

    fn __setitem__(
        mut slf: PyRefMut<Self>,
        index: isize,
        record: PyRef<DataFlexTable>,
    ) -> PyResult<()> {
        todo!()
    }

    fn __delitem__(mut slf: PyRefMut<Self>, index: isize) -> PyResult<()> {
        todo!()
    }

    fn __iter__(slf: PyRef<Self>) -> PyResult<PyObject> {
        todo!()
    }

    fn __contains__(slf: PyRef<Self>, value: PyObject) -> PyResult<bool> {
        todo!()
    }

    fn __reversed__(slf: PyRef<Self>) -> PyResult<Vec<PyObject>> {
        todo!()
    }

    fn pretty(slf: PyRef<Self>) -> String {
        slf._as_pretty_table()
    }

    #[getter(tables)]
    fn get_tables(slf: PyRef<Self>) -> PyResult<Vec<DataFlexTable>> {
        todo!()
    }
}

// <editor-fold desc="// DataFlexDB ...">

// <editor-fold desc="// Tests ...">

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::DataFlexDB;

    #[test]
    /// Test that the `DataFlexDB` structure correctly handles table data
    fn gets_dbs() {
        todo!()
    }
}

// </editor-fold desc="// Tests ...">
