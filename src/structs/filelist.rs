// A structured representation of DataFlex's `filelist.cfg` file

// Standard Library Imports
use std::fmt;
use std::iter::IntoIterator;

// Third-Party Imports
use caseless::compatibility_caseless_match_str as cl_eq;
use prettytable::{Cell as PrettyCell, Row as PrettyRow, Table as PrettyTable};
use pyo3;
use pyo3::exceptions::{PyFileNotFoundError, PyIndexError, PyKeyError};
use pyo3::prelude::*;
use pyo3::types::PySliceIndices;
use serde::{Deserialize, Serialize};

// Crate-Level Imports
use crate::utils::{bytes_from_file, string_from_bytes};
use crate::{iif, AttrIndexSliceOrItem, ValueOrSlice};

// <editor-fold desc="// FileListEntry ...">

#[derive(Debug, Clone, Eq, Ord, PartialOrd, PartialEq, Serialize, Deserialize)]
#[pyclass(dict, module = "ferroflex.structs")]
/// A single entry within a DataFlex `filelist.cfg` file
pub struct FileListEntry {
    #[pyo3(get)]
    /// A table's numeric index within
    /// the DataFlex "database"
    pub file_number: usize,
    #[pyo3(get)]
    /// A table's on-disk file name
    pub root_name: String,
    #[pyo3(get)]
    /// The name by which DataFlex references
    /// a given table
    pub dataflex_name: Option<String>,
    #[pyo3(get)]
    /// A table's human-readable description
    pub description: Option<String>,
}

unsafe impl Send for FileListEntry {}

impl Default for FileListEntry {
    fn default() -> Self {
        FileListEntry {
            file_number: 0usize,
            root_name: "filelist.cfg".to_string(),
            dataflex_name: None,
            description: None,
        }
    }
}

impl fmt::Display for FileListEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FileListEntry<number: {} | root_name: {} | df_name: {} | desc: {}>",
            self.file_number,
            self.root_name.as_str(),
            match &self.dataflex_name {
                None => "",
                Some(name) => name.as_str(),
            },
            match &self.description {
                None => "",
                Some(desc) => desc.as_str(),
            },
        )
    }
}

impl FileListEntry {
    // <editor-fold desc="// 'Private' Methods ...">

    pub(crate) fn _as_pretty_table(&self) -> String {
        let mut table = PrettyTable::new();

        vec![
            ("number", (&self.file_number).to_string()),
            ("root_name", (&self.root_name).to_string()),
            (
                "dataflex_name",
                match &self.dataflex_name {
                    None => "N/A".to_string(),
                    Some(name) => {
                        if !name.is_empty() {
                            name.to_string()
                        } else {
                            "N/A".to_string()
                        }
                    }
                },
            ),
            (
                "description",
                match &self.description {
                    None => "N/A".to_string(),
                    Some(desc) => {
                        if !desc.is_empty() {
                            desc.to_string()
                        } else {
                            "N/A".to_string()
                        }
                    }
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

    pub fn from_bytes<T: Into<usize>>(data: &[u8], idx: Option<T>) -> PyResult<FileListEntry> {
        Ok(FileListEntry {
            file_number: match idx {
                Some(value) => value.into(),
                None => 0usize,
            },
            root_name: string_from_bytes(&data[..40], Some(false))?,
            dataflex_name: match string_from_bytes(&data[41..73], Some(false)) {
                Ok(name) => Some(name),
                Err(_) => None,
            },
            description: match string_from_bytes(&data[73..], Some(false)) {
                Ok(desc) => Some(desc),
                Err(_) => None,
            },
        })
    }

    pub fn is(&self, table: AttrIndexSliceOrItem<FileListEntry>) -> bool {
        match table {
            AttrIndexSliceOrItem::Slice(_) => false,
            AttrIndexSliceOrItem::Item(entry) => *self == entry,
            AttrIndexSliceOrItem::Name(name) => {
                let root_name: &str = self.root_name.as_str();
                let df_name: &str = match &self.dataflex_name {
                    None => "",
                    Some(df) => df.as_str(),
                };

                cl_eq(name, root_name) || cl_eq(name, df_name)
            }
            AttrIndexSliceOrItem::Index(index) => {
                if index < 0 {
                    return false;
                }
                self.file_number as isize == index
            }
        }
    }

    // </editor-fold desc="// Public Methods ...">
}

#[pymethods]
impl FileListEntry {
    #[new]
    fn __new__(
        file_number: Option<usize>,
        root_name: Option<String>,
        dataflex_name: Option<String>,
        description: Option<String>,
    ) -> PyResult<Self> {
        Ok(Self {
            file_number: file_number.unwrap_or(0usize),
            root_name: root_name.unwrap_or_else(|| "".to_string()),
            dataflex_name,
            description,
        })
    }

    fn __str__(slf: PyRefMut<Self>) -> PyResult<String> {
        Ok(format!("{}", *slf))
    }

    fn __repr__(slf: PyRefMut<Self>) -> PyResult<String> {
        Ok(format!("{}", *slf))
    }

    fn pretty(slf: PyRefMut<Self>) -> String {
        slf._as_pretty_table()
    }
}

// </editor-fold desc="// FileListEntry ...">

// <editor-fold desc="// FileList ...">

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[pyclass(dict, module = "ferroflex.structs")]
/// A structured representation of DataFlex's `filelist.cfg` file
pub struct FileList {
    /// A table's numeric index within
    /// the DataFlex "database"
    pub files: Vec<FileListEntry>,
}

unsafe impl Send for FileList {}

impl fmt::Display for FileList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self._as_pretty_table())
    }
}

impl IntoIterator for FileList {
    type Item = FileListEntry;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.files.into_iter()
    }
}

impl FileList {
    // <editor-fold desc="// 'Private' Methods ...">

    pub(crate) fn _as_pretty_table(&self) -> String {
        let mut table = PrettyTable::new();

        self.files.iter().for_each(|entry| {
            table.add_row(PrettyRow::from(vec![PrettyCell::new(
                entry._as_pretty_table().as_str(),
            )]));
        });

        table.to_string()
    }

    // </editor-fold desc="// 'Private' Methods ...">

    // <editor-fold desc="// Public Methods ...">

    pub fn from_bytes(data: &[u8]) -> PyResult<FileList> {
        Ok(FileList {
            files: data
                .chunks_exact(128)
                .enumerate()
                .map(|(idx, chunk)| FileListEntry::from_bytes(chunk, Some(idx)).unwrap_or_default())
                .filter(|entry| {
                    if cl_eq(entry.root_name.as_str(), "filelist.cfg") && entry.file_number == 0 {
                        return true;
                    }

                    let df_name: bool = match entry.dataflex_name.as_ref() {
                        None => false,
                        Some(val) => !val.is_empty(),
                    };

                    let desc: bool = match entry.description.as_ref() {
                        None => false,
                        Some(val) => !val.is_empty(),
                    };

                    let name: bool = !cl_eq(entry.root_name.as_str(), "filelist.cfg")
                        && !entry.root_name.is_empty()
                        && entry.file_number >= 1;

                    name && (df_name || desc)
                })
                .collect::<Vec<FileListEntry>>(),
        })
    }

    pub fn from_path<T: AsRef<str>>(filepath: T) -> PyResult<FileList> {
        let filepath: &str = AsRef::<str>::as_ref(&filepath);

        match bytes_from_file(filepath, None::<u64>, None::<u64>) {
            Ok(data) => Ok(FileList::from_bytes(&data)?),
            Err(_) => Err(PyFileNotFoundError::new_err(format!(
                "Could not create a usable `FileList` from path '{}'",
                filepath
            ))),
        }
    }

    pub fn len(&self) -> usize {
        self.files.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> std::slice::Iter<FileListEntry> {
        self.files.iter()
    }

    pub fn get(
        &'fl self,
        table: AttrIndexSliceOrItem<FileListEntry>,
    ) -> Option<&'fl FileListEntry> {
        let table = if let AttrIndexSliceOrItem::Index(index) = table {
            let index = iif!(index > -1, index, self.files.len() as isize + index);

            if index < 0 {
                return None;
            }

            AttrIndexSliceOrItem::Index(index)
        } else {
            table
        };

        self.files
            .iter()
            .filter(|entry| entry.is(table.clone()))
            .next()
    }

    pub fn contains(&self, table: AttrIndexSliceOrItem<FileListEntry>) -> bool {
        match self.get(table) {
            Some(_) => true,
            None => false,
        }
    }

    // </editor-fold desc="// Public Methods ...">
}

// TODO: Add impl for PyIterator trait to FileList object

#[pymethods]
impl FileList {
    #[new]
    fn __new__(filepath: Option<String>, files: Option<Vec<FileListEntry>>) -> PyResult<Self> {
        let mut file_list = match filepath {
            None => Self::default(),
            Some(path) => Self::from_path(path)?,
        };

        if let Some(fls) = files {
            file_list.files.extend(fls);
        }

        Ok(file_list)
    }

    fn __str__(slf: PyRefMut<Self>) -> PyResult<String> {
        Ok(format!("{}", *slf))
    }

    fn __repr__(slf: PyRefMut<Self>) -> PyResult<String> {
        Ok(format!("{}", *slf))
    }

    fn __len__(slf: PyRefMut<Self>) -> usize {
        slf.len()
    }

    fn __getitem__(
        slf: PyRefMut<Self>,
        key: AttrIndexSliceOrItem<FileListEntry>,
    ) -> PyResult<ValueOrSlice<FileListEntry>> {
        match key {
            #[allow(unused_variables)]
            AttrIndexSliceOrItem::Item(item) => {
                todo!()
            }
            AttrIndexSliceOrItem::Index(idx) => {
                let idx: isize = iif!(idx > -1, idx, slf.files.len() as isize + idx);

                if idx < 0 {
                    return Err(PyIndexError::new_err(""));
                }

                let idx: usize = idx as usize;

                let rec: Option<&FileListEntry> = slf.files.get(idx);

                if rec.is_none() {
                    return Err(PyIndexError::new_err(""));
                }

                Ok(ValueOrSlice::Value(rec.unwrap().clone()))
            }
            AttrIndexSliceOrItem::Name(name) => {
                for table in slf.files.iter() {
                    if table.is(AttrIndexSliceOrItem::Name(name)) {
                        return Ok(ValueOrSlice::Value(table.clone()));
                    }
                }
                Err(PyKeyError::new_err(""))
            }
            AttrIndexSliceOrItem::Slice(slc) => {
                let indexes: PySliceIndices = slc.indices(3)?;

                let (start, end) = (indexes.start, indexes.stop);

                let end: isize = iif!(end > -1, end, slf.files.len() as isize + end);
                let start: isize = iif!(start > -1, start, slf.files.len() as isize + start);

                if start < 0 || end < 0 {
                    return Err(PyIndexError::new_err(""));
                }

                let end: usize = end as usize;
                let start: usize = start as usize;

                Ok(ValueOrSlice::Slice((&slf.files)[start..end].to_vec()))
            }
        }
    }

    fn __setitem__(
        _slf: PyRefMut<Self>,
        _index: isize,
        _record: PyRef<FileListEntry>,
    ) -> PyResult<()> {
        todo!()
    }

    fn __delitem__(_slf: PyRefMut<Self>, _index: isize) -> PyResult<()> {
        todo!()
    }

    #[allow(unused_variables)]
    fn __iter__(slf: PyRef<Self>) -> PyResult<()> {
        todo!()
    }

    fn __contains__(slf: PyRefMut<Self>, table: AttrIndexSliceOrItem<FileListEntry>) -> bool {
        slf.contains(table)
    }

    fn __reversed__(_slf: PyRef<Self>) -> PyResult<Vec<FileListEntry>> {
        todo!()
    }

    fn pretty(slf: PyRefMut<Self>) -> String {
        slf._as_pretty_table()
    }
}

// </editor-fold desc="// FileList ...">

// <editor-fold desc="// Tests ...">

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::{FileList, FileListEntry};

    #[test]
    /// Test that the `FileList` structure behaves as expected
    fn gets_file_lists() {
        todo!()
    }
}

// </editor-fold desc="// Tests ...">
