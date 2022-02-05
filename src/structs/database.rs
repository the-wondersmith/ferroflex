// A structured representation of a collection of DataFlex table files

// Standard Library Imports
use std::borrow::Borrow;
use std::fmt;
use std::iter::IntoIterator;
use std::ops::Index as Indexable;
use std::path::PathBuf;

// Third-Party Imports
use gluesql::core::data::Schema;
use prettytable::{Cell as PrettyCell, Row as PrettyRow, Table as PrettyTable};
use pyo3::PyResult;
use serde::{Deserialize, Serialize};

// Crate-Level Imports
use crate::structs::{DataFlexTable, FileList};
use crate::utils::{path_from_string, string_from_path};
use crate::{iif, AttrIndexSliceOrItem};

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

impl DataFlexDB {
    // <editor-fold desc="// 'Private' Methods ...">

    pub(crate) fn _as_pretty_table(&self) -> String {
        let mut table = PrettyTable::new();

        self.filelist.iter().for_each(|entry| {
            table.add_row(PrettyRow::from(vec![
                PrettyCell::new(entry.file_number.to_string().as_str()),
                PrettyCell::new(if let Some(df) = &entry.dataflex_name {
                    df.as_str()
                } else {
                    &entry.root_name.as_str()
                }),
                PrettyCell::new(match &entry.description {
                    Some(desc) => desc.as_str(),
                    _ => "",
                }),
            ]));
        });

        table.to_string()
    }

    // </editor-fold desc="// 'Private' Methods ...">

    // <editor-fold desc="// Public Methods ...">

    pub fn len(&self) -> usize {
        self.filelist.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(self) -> std::vec::IntoIter<DataFlexTable> {
        self.into_iter()
    }

    pub fn get(
        &'fl self,
        table: AttrIndexSliceOrItem<DataFlexTable>,
    ) -> Option<&'fl DataFlexTable> {
        match table {
            AttrIndexSliceOrItem::Index(idx) => {
                let idx: isize = iif!(idx > 0, idx, self.tables.len() as isize + idx);
                if idx < 0 {
                    return None;
                }

                Some(&self.tables[idx as usize])
            }
            AttrIndexSliceOrItem::Name(name) => {
                self.tables.iter().filter(|tbl| *tbl == name).next()
            }
            _ => None,
        }
    }

    pub fn contains(&self, table: AttrIndexSliceOrItem<DataFlexTable>) -> bool {
        self.get(table).is_some()
    }

    pub fn from_path<P: AsRef<str>>(db_path: P) -> PyResult<DataFlexDB> {
        let filelist: FileList = FileList::from_path(db_path.borrow().as_ref())?;
        let db_path: PathBuf = path_from_string(db_path.as_ref(), Some(true));

        let tables: Vec<DataFlexTable> = filelist
            .borrow()
            .iter()
            .map(|file| db_path.join(format!("{}.dat", &file.root_name)))
            .filter(|path| path.exists() && path.is_file())
            .map(|path| DataFlexTable::from_path(string_from_path(path.as_path(), Some(false))))
            .filter(PyResult::is_ok)
            .map(PyResult::unwrap)
            .collect();

        Ok(DataFlexDB {
            db_path,
            filelist,
            tables,
        })
    }

    pub fn schema(&self) -> Vec<Schema> {
        (&self.tables)
            .iter()
            .map(DataFlexTable::schema)
            .collect::<Vec<Schema>>()
    }

    #[allow(unused_variables)]
    pub fn create_table(&self) -> PyResult<()> {
        todo!()
    }

    #[allow(unused_variables)]
    pub fn alter_table(&self) -> PyResult<()> {
        todo!()
    }

    #[allow(unused_variables)]
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
