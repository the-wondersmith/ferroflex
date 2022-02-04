// A structured representation of DataFlex's `filelist.cfg` file

// Standard Library Imports
use std::fmt;
use std::iter::IntoIterator;
use std::path::Path;

// Third-Party Imports
use caseless::compatibility_caseless_match_str as cl_eq;
use prettytable::{Cell as PrettyCell, Row as PrettyRow, Table as PrettyTable};
use pyo3::exceptions::{PyFileNotFoundError, PyIndexError, PyValueError};
use pyo3::PyResult;
use serde::{Deserialize, Serialize};

// Crate-Level Imports
use crate::utils::{bytes_from_file, path_from_string, string_from_bytes};
use crate::{iif, AttrIndexSliceOrItem};

// <editor-fold desc="// FileListEntry ...">

#[derive(Clone, Debug, Eq, Ord, PartialOrd, PartialEq, Serialize, Deserialize)]
/// A single entry within a DataFlex `filelist.cfg` file
pub struct FileListEntry {
    /// A table's numeric index within
    /// the DataFlex "database"
    pub file_number: usize,
    /// A table's on-disk file name
    pub root_name: String,
    /// The name by which DataFlex references
    /// a given table
    pub dataflex_name: Option<String>,
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
        let file_number = match idx {
            Some(value) => value.into(),
            None => 0usize,
        };

        let root_name = string_from_bytes(&data[..40], Some(false))?;
        let dataflex_name = string_from_bytes(&data[41..73], Some(false))?;
        let description = string_from_bytes(&data[73..], Some(false))?;

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

        Ok(FileListEntry {
            file_number,
            root_name,
            dataflex_name: Some(dataflex_name),
            description: Some(description),
        })
    }

    pub fn is(&self, table: &AttrIndexSliceOrItem<FileListEntry>) -> bool {
        match table {
            AttrIndexSliceOrItem::Slice(_) => false,
            AttrIndexSliceOrItem::Item(entry) => entry.eq(self),
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

// </editor-fold desc="// FileListEntry ...">

// <editor-fold desc="// FileList ...">

#[derive(Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq, Serialize, Deserialize)]
/// A structured representation of DataFlex's `filelist.cfg` file
pub struct FileList {
    /// A table's numeric index within
    /// the DataFlex "database"
    pub files: Vec<FileListEntry>,
}

unsafe impl Send for FileList {}

impl fmt::Display for FileList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FileList<tables: {}>", self.files.len())
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
                .map(|(idx, chunk)| FileListEntry::from_bytes(chunk, Some(idx)))
                .filter(PyResult::is_ok)
                .map(PyResult::unwrap)
                .collect::<Vec<FileListEntry>>(),
        })
    }

    pub fn from_path<T: AsRef<str>>(filepath: T) -> PyResult<FileList> {
        let mut filepath = AsRef::<str>::as_ref(&filepath).to_string();

        if !filepath.ends_with("filelist.cfg") {
            if let Some(path) = Path::join(
                path_from_string(&filepath, Some(true)).as_path(),
                "filelist.cfg",
            )
            .to_str()
            {
                filepath = path.to_string();
            }
        }

        match bytes_from_file(&filepath, None::<u64>, None::<u64>) {
            Ok(data) => Ok(FileList::from_bytes(&data)?),
            Err(_) => Err(PyFileNotFoundError::new_err(format!(
                "Could not create a usable `FileList` from path '{}'",
                &filepath
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

        self.files.iter().filter(|entry| entry.is(&table)).next()
    }

    pub fn contains(&self, table: AttrIndexSliceOrItem<FileListEntry>) -> bool {
        match self.get(table) {
            Some(_) => true,
            None => false,
        }
    }

    // </editor-fold desc="// Public Methods ...">
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
