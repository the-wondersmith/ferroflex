// A structured representation of a collection of DataFlex table files

use std::borrow::Borrow;
// Standard Library Imports
use std::collections::HashMap;
use std::fmt;
use std::iter::IntoIterator;
use std::ops::Index as Indexable;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

// Third-Party Imports
use caseless::compatibility_caseless_match_str as cl_eq;
use prettytable::Table as PrettyTable;
use pyo3::exceptions::PyFileNotFoundError;
// {Cell, Row as PrintableRow, };
use pyo3::iter::PyIterProtocol;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

// Crate-Level Imports
use crate::structs::{DataFlexTable, FileList, FileListEntry};
use crate::utils::{path_from_string, string_from_path};
use crate::{AttrIndexOrSlice, NameIndexOrItem, ValueOrSlice};

// <editor-fold desc="// DB Table Iterator ...">

#[derive(Clone, Debug)]
#[pyclass(dict, module = "ferroflex.structs")]
/// An intermediate structure used to iterate over the
/// individual tables in a DataFlex "database"
pub struct DBTableIterator {
    pub inner: std::vec::IntoIter<DataFlexTable>,
}

#[pyproto]
impl PyIterProtocol for DBTableIterator {
    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<Self>) -> Option<DataFlexTable> {
        slf.inner.next()
    }
}

// </editor-fold desc="// DB Table Iterator ...">

#[derive(Debug, Default, PartialEq, PartialOrd, Serialize, Deserialize)]
#[pyclass(dict, module = "ferroflex.structs")]
/// A structured representation of a collection of DataFlex table files
pub struct DataFlexDB {
    #[pyo3(get)]
    /// The db's on-disk path
    pub db_path: PathBuf,
    #[pyo3(get)]
    /// The db's filelist
    pub filelist: FileList,
    // #[pyo3(get)]
    /// The db's tables, as <TableNumber, Table>
    pub tables: Arc<RwLock<HashMap<usize, DataFlexTable>>>,
}

unsafe impl Send for DataFlexDB {}

impl<T: AsRef<str>> From<T> for DataFlexDB {
    fn from(db_path: T) -> PyResult<Self> {
        let db_path: PathBuf = path_from_string(db_path.as_ref(), Some(true));

        let db_path: PathBuf = if db_path.is_dir() {
            db_path.with_file_name("filelist.cfg")
        } else {
            db_path
        };

        if !cl_eq("filelist.cfg", db_path.borrow().file_name()?.to_str()?) {
            return Err(PyFileNotFoundError::new_err(""));
        }

        Ok(Self {
            filelist: FileList::from_path(string_from_path(db_path.borrow(), None))?,
            tables: Arc::new(RwLock::new(HashMap::new())),
            db_path,
        })
    }
}

#[allow(unused_variables)]
impl<T: Into<i64>> Indexable<T> for DataFlexDB {
    type Output = PyResult<DataFlexTable>;

    fn index(&self, index: T) -> &'static Self::Output {
        let index: i64 = index.into();

        todo!()
    }
}

#[allow(unused_variables)]
impl IntoIterator for DataFlexDB {
    type Item = DataFlexTable;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
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

    fn _field_values(&self) -> Vec<(String, String)> {
        todo!()
    }

    fn _as_pretty_table(&self) -> PrettyTable {
        todo!()
    }

    // </editor-fold desc="// 'Private' Methods ...">

    // <editor-fold desc="// Public Methods ...">

    pub fn len(&self) -> usize {
        self.filelist.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> std::vec::IntoIter<DataFlexTable> {
        todo!()
    }

    pub fn get(&'fl self, table: NameIndexOrItem<DataFlexTable>) -> Option<&'fl DataFlexTable> {
        let entry: &FileListEntry = match table {
            NameIndexOrItem::Item(tbl) => {
                return Some(tbl);
            }
            NameIndexOrItem::Index(idx) => match self.filelist.get(NameIndexOrItem::Index(idx)) {
                None => {
                    return None;
                }
                Some(tbl) => tbl,
            },
            NameIndexOrItem::Name(name) => match self.filelist.get(NameIndexOrItem::Name(name)) {
                None => {
                    return None;
                }
                Some(tbl) => tbl,
            },
        };

        {
            let mut tables = self.tables.try_write().unwrap();

            if tables.get(&entry.file_number).is_none() {
                let record = DataFlexTable::default();

                unimplemented!();

                tables.insert(entry.file_number, record);
            }
        }

        (*self.tables).read().unwrap().get(&entry.file_number)
    }

    pub fn contains(&self, table: NameIndexOrItem<DataFlexTable>) -> bool {
        match self.get(table) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn from_path<P: AsRef<str>>(db_path: P) -> PyResult<DataFlexDB> {
        todo!()
    }

    pub fn create_table(&self) -> PyResult<()> {
        todo!()
    }

    pub fn update_table(&self) -> PyResult<()> {
        todo!()
    }

    // </editor-fold desc="// Public Methods ...">
}

#[allow(unused_variables)]
#[pymethods]
impl DataFlexDB {
    #[new]
    fn __new__(filepath: String) -> PyResult<Self> {
        Self::from_path(AsRef::<str>::as_ref(&filepath))
    }

    fn __str__(slf: PyRefMut<Self>) -> PyResult<String> {
        Ok(format!("{}", *slf))
    }

    fn __repr__(slf: PyRefMut<Self>) -> PyResult<String> {
        Ok(format!("{}", *slf))
    }

    fn __len__(slf: PyRefMut<Self>) -> usize {
        slf.len() as usize
    }

    fn __getitem__(
        slf: PyRefMut<Self>,
        key: AttrIndexOrSlice,
    ) -> PyResult<ValueOrSlice<DataFlexTable>> {
        todo!()
    }

    fn __setitem__(
        slf: PyRefMut<Self>,
        index: isize,
        record: PyRef<DataFlexTable>,
    ) -> PyResult<()> {
        todo!()
    }

    fn __delitem__(slf: PyRefMut<Self>, index: isize) -> PyResult<()> {
        todo!()
    }

    fn __iter__(slf: PyRef<Self>) -> PyResult<Py<DBTableIterator>> {
        // Py::new(
        //     slf.py(),
        //     DBTableIterator {
        //         inner: slf
        //             .0
        //             .iter()
        //             .map(sql_value_to_py)
        //             .filter(PyResult::is_ok)
        //             .map(PyResult::unwrap)
        //             .collect::<Vec<PyObject>>()
        //             .into_iter(),
        //     },
        // )
        todo!()
    }

    fn __contains__(slf: PyRef<Self>, value: PyObject) -> PyResult<bool> {
        todo!()
    }

    fn __reversed__(slf: PyRef<Self>) -> PyResult<Vec<PyObject>> {
        todo!()
    }

    fn pretty(slf: PyRefMut<Self>) -> String {
        slf._as_pretty_table().to_string()
    }

    #[getter(tables)]
    fn get_tables(slf: PyRefMut<Self>) -> PyResult<Vec<DataFlexTable>> {
        todo!()
    }
}
