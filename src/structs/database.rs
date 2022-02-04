// A structured representation of a collection of DataFlex table files

// Standard Library Imports
use std::fmt;
use std::iter::IntoIterator;
use std::ops::Index as Indexable;
use std::path::PathBuf;

// Third-Party Imports
#[allow(unused_imports)]
use prettytable::{Cell as PrettyCell, Row as PrettyRow, Table as PrettyTable};
use pyo3::PyResult;
use serde::{Deserialize, Serialize};

// Crate-Level Imports
use crate::structs::{DataFlexTable, FileList};
use crate::AttrIndexSliceOrItem;

// <editor-fold desc="// DataFlexDB ...">

#[derive(Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq, Serialize, Deserialize)]
/// A structured representation of a collection of DataFlex table files
pub struct DataFlexDB {
    /// The db's on-disk path
    pub db_path: PathBuf,
    /// The db's filelist
    pub filelist: FileList,
    /// The db's filelist
    pub tables: Vec<DataFlexTable>,
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
    type Item = DataFlexTable;
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
        self.filelist.len()
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
