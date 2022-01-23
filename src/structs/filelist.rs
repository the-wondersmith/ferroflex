// A structured representation of DataFlex's `filelist.cfg` file

// Standard Library Imports
use std::fmt;
use std::iter::IntoIterator;

// Third-Party Imports
use caseless::compatibility_caseless_match_str as cl_eq;
use prettytable::{Cell as PrettyCell, Row as PrettyRow, Table as PrettyTable};
use pyo3;
use pyo3::exceptions::{PyFileNotFoundError, PyIndexError, PyKeyError, PyValueError};
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

    pub fn from_bytes<T: Into<usize>>(data: &[u8], idx: Option<T>) -> PyResult<Py<FileListEntry>> {
        let file_number = match idx {
            Some(value) => value.into(),
            None => 0usize,
        };

        let root_name = string_from_bytes(&data[..40], Some(false))?;
        let dataflex_name = string_from_bytes(&data[41..73], Some(false))?;
        let description = string_from_bytes(&data[73..], Some(false))?;

        let gil = Python::acquire_gil();
        let py = gil.python();

        if (file_number == 0usize && !cl_eq(&root_name, "filelist.cfg"))
            || (file_number > 0usize && cl_eq(&root_name, "filelist.cfg"))
        {
            return Err(PyIndexError::new_err(format!(
                "Invalid `file_number` + `root_name` pair: {} + {}",
                &file_number, &root_name,
            )));
        }

        if file_number > 0 && dataflex_name.is_empty() && description.is_empty() {
            return Err(PyValueError::new_err("Missing one or more field values!"));
        }

        Py::new(
            py,
            FileListEntry {
                file_number,
                root_name,
                dataflex_name: Some(dataflex_name),
                description: Some(description),
            },
        )
    }

    pub fn is(&self, table: &AttrIndexSliceOrItem<Py<FileListEntry>>) -> bool {
        match table {
            AttrIndexSliceOrItem::Slice(_) => false,
            AttrIndexSliceOrItem::Item(entry) => {
                let gil = Python::acquire_gil();
                let py = gil.python();

                let is_: bool = Py::borrow(&entry, py).eq(self);

                is_
            }
            AttrIndexSliceOrItem::Name(name) => {
                let root_name: &str = self.root_name.as_str();
                let df_name: &str = match &self.dataflex_name {
                    None => "",
                    Some(df) => df.as_str(),
                };

                cl_eq(name, root_name) || cl_eq(name, df_name)
            }
            AttrIndexSliceOrItem::Index(index) => {
                if *index < 0isize {
                    return false;
                }
                self.file_number as isize == *index
            }
        }
    }

    // </editor-fold desc="// Public Methods ...">
}

#[pymethods]
impl FileListEntry {
    #[new]
    fn __new__(
        file_number: usize,
        root_name: String,
        dataflex_name: Option<String>,
        description: Option<String>,
    ) -> PyResult<Self> {
        Ok(Self {
            file_number,
            root_name,
            dataflex_name,
            description,
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

// </editor-fold desc="// FileListEntry ...">

// <editor-fold desc="// EntryIterator ...">

#[pyclass(dict, module = "ferroflex.structs")]
/// An iterator over the entries in a DataFlex `filelist.cfg` file
struct EntryIterator {
    entries: Box<dyn Iterator<Item = Py<FileListEntry>>>,
}

unsafe impl Send for EntryIterator {}

#[pyproto]
impl pyo3::PyIterProtocol for EntryIterator {
    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }
    fn __next__(mut slf: PyRefMut<Self>) -> Option<Py<FileListEntry>> {
        slf.entries.next()
    }
}

// </editor-fold desc="// EntryIterator ...">

// <editor-fold desc="// FileList ...">

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[pyclass(dict, module = "ferroflex.structs")]
/// A structured representation of DataFlex's `filelist.cfg` file
pub struct FileList {
    /// A table's numeric index within
    /// the DataFlex "database"
    pub files: Vec<Py<FileListEntry>>,
}

unsafe impl Send for FileList {}

impl fmt::Display for FileList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FileList<tables: {}>", self.files.len())
    }
}

impl IntoIterator for FileList {
    type Item = Py<FileListEntry>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.files.into_iter()
    }
}

impl FileList {
    // <editor-fold desc="// 'Private' Methods ...">

    pub(crate) fn _as_pretty_table(&self) -> String {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let mut table = PrettyTable::new();

        self.files.iter().for_each(|entry| {
            table.add_row(PrettyRow::from(vec![PrettyCell::new(
                Py::borrow(entry, py)._as_pretty_table().as_str(),
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
                .map(|(idx, chunk)| FileListEntry::from_bytes(chunk, Some(idx)))
                .filter(PyResult::is_ok)
                .map(PyResult::unwrap)
                .collect::<Vec<Py<FileListEntry>>>(),
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

    pub fn iter(&self) -> std::slice::Iter<Py<FileListEntry>> {
        self.files.iter()
    }

    pub fn get(
        &'fl self,
        table: AttrIndexSliceOrItem<Py<FileListEntry>>,
    ) -> Option<&'fl Py<FileListEntry>> {
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
            .filter(|entry| Python::with_gil(|py| Py::borrow(entry, py).is(&table)))
            .next()
    }

    pub fn contains(&self, table: AttrIndexSliceOrItem<Py<FileListEntry>>) -> bool {
        match self.get(table) {
            Some(_) => true,
            None => false,
        }
    }

    // </editor-fold desc="// Public Methods ...">
}

#[pymethods]
impl FileList {
    #[new]
    fn __new__(filepath: Option<String>, files: Option<Vec<Py<FileListEntry>>>) -> PyResult<Self> {
        let mut file_list = match filepath {
            None => Self::default(),
            Some(path) => Self::from_path(path)?,
        };

        if let Some(fls) = files {
            file_list.files.extend(fls);
        }

        Ok(file_list)
    }

    fn __str__(slf: PyRef<Self>) -> PyResult<String> {
        Ok(format!("{}", *slf))
    }

    fn __repr__(slf: PyRef<Self>) -> PyResult<String> {
        Ok(format!("{}", *slf))
    }

    fn __len__(slf: PyRef<Self>) -> usize {
        slf.len()
    }

    fn __getitem__(
        slf: PyRef<Self>,
        key: AttrIndexSliceOrItem<PyRef<FileListEntry>>,
    ) -> PyResult<ValueOrSlice<Py<FileListEntry>>> {
        match key {
            AttrIndexSliceOrItem::Item(_) => Err(PyKeyError::new_err("")),
            AttrIndexSliceOrItem::Index(idx) => {
                let idx: isize = iif!(idx > -1, idx, slf.files.len() as isize + idx);

                if idx < 0 {
                    return Err(PyIndexError::new_err(""));
                }

                let idx: usize = idx as usize;

                let rec: Option<&Py<FileListEntry>> = slf.files.get(idx);

                match rec {
                    None => Err(PyIndexError::new_err("")),
                    Some(record) => {
                        Python::with_gil(|py| Ok(ValueOrSlice::Value(Py::clone_ref(record, py))))
                    }
                }
            }
            AttrIndexSliceOrItem::Name(name) => Python::with_gil(|py| {
                for table in slf.files.iter() {
                    if Py::borrow(table, py).is(&AttrIndexSliceOrItem::Name(name)) {
                        return Ok(ValueOrSlice::Value(Py::clone_ref(table, py)));
                    }
                }
                Err(PyKeyError::new_err(""))
            }),
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

                Ok(ValueOrSlice::Slice(slf.files[start..end].to_vec()))
            }
        }
    }

    fn __setitem__(
        mut slf: PyRefMut<Self>,
        index: isize,
        record: Py<FileListEntry>,
    ) -> PyResult<()> {
        let index: isize = iif!(index > -1, index, slf.files.len() as isize + index);

        if index < 0 {
            return Err(PyIndexError::new_err(""));
        }

        match slf.files.get_mut(index as usize) {
            None => slf.files.push(record),
            Some(entry) => {
                std::mem::drop(std::mem::replace(entry, record));
            }
        }

        Ok(())
    }

    fn __delitem__(mut slf: PyRefMut<Self>, index: isize) -> PyResult<()> {
        let index: isize = iif!(index > -1, index, slf.files.len() as isize + index);

        if index < 0 || index >= slf.files.len() as isize {
            return Err(PyIndexError::new_err(""));
        }

        slf.files.remove(index as usize);

        Ok(())
    }

    fn __iter__(slf: PyRef<Self>) -> EntryIterator {
        EntryIterator {
            entries: Box::new(slf.files.clone().into_iter()),
        }
    }

    fn __contains__(slf: PyRef<Self>, table: AttrIndexSliceOrItem<Py<FileListEntry>>) -> bool {
        slf.contains(table)
    }

    fn __reversed__(slf: PyRef<Self>) -> EntryIterator {
        EntryIterator {
            entries: Box::new(slf.files.clone().into_iter().rev()),
        }
    }

    fn pretty(slf: PyRef<Self>) -> String {
        slf._as_pretty_table()
    }

    fn append(slf: PyRefMut<Self>, record: Py<FileListEntry>) -> PyResult<()> {
        let idx = (slf.len() + 1) as isize;

        FileList::__setitem__(slf, idx, record)
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
