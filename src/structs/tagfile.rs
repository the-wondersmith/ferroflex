// A structured representation of a DataFlex `.tag` file

// Standard Library Imports
use std::fmt;
use std::fmt::Formatter;
use std::iter::IntoIterator;
use std::ops::Index as Indexable;
use std::path::Path;
use std::slice::SliceIndex;

// Third-Party Imports

use prettytable::Table as PrettyTable; // Cell, Row as PrintableRow,
use pyo3::exceptions::{PyIndexError, PyKeyError};
use pyo3::prelude::*;
use pyo3::types::PySliceIndices;
use serde::{Deserialize, Serialize};

// Crate-Level Imports
use crate::utils::{bytes_from_file, string_from_path};
use crate::{iif, AttrIndexSliceOrItem, ValueOrSlice};

// <editor-fold desc="// Custom Types ...">

pub type TagCollection = Vec<String>;

// </editor-fold desc="// Custom Types ...">

// <editor-fold desc="// Tag File ...">

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[pyclass(dict, module = "ferroflex.structs")]
/// A structured representation of a field segment's
/// definition in the header of a DataFlex table file
pub struct TagFile {
    #[pyo3(get)]
    /// The file's absolute on-disk path
    pub filepath: String,
    #[pyo3(get)]
    /// The column names contained by the file
    pub tags: TagCollection,
}

unsafe impl Send for TagFile {}

impl fmt::Display for TagFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TagFile<'{}' | [{}]>",
            self.filepath.as_str(),
            self.tags.join(", ")
        )
    }
}

impl<I: SliceIndex<[String]>> Indexable<I> for TagFile {
    type Output = <I as SliceIndex<[String]>>::Output;

    fn index(&self, index: I) -> &Self::Output {
        self.tags.index(index)
    }
}

impl IntoIterator for TagFile {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.tags.into_iter()
    }
}

impl TagFile {
    // <editor-fold desc="// 'Private' Methods ...">

    fn _as_pretty_table(&self) -> PrettyTable {
        todo!()
    }

    // </editor-fold desc="// 'Private' Methods ...">

    // <editor-fold desc="// 'Public' Methods ...">

    pub fn find_tags_for_table(table_path: &Path) -> PyResult<Option<String>> {
        let tag_path = table_path.with_extension("tag");

        if !tag_path.exists() || !tag_path.is_file() {
            return Ok(None);
        }

        // TODO: Implement limited recursive ancestor search for MIA tag files

        Ok(Some(string_from_path(
            &tag_path,
            Some(!tag_path.is_absolute()),
        )))
    }

    pub fn from_filepath<T: AsRef<str>>(filepath: T) -> PyResult<TagFile> {
        // The column names in a tag file are delimited by ASCII \r\n
        // which means we can format them *and* get them into a usable
        // vector at the same time by reading the whole tag file into a
        // unicode-encoded string, then splitting at ASCII whitespace
        // characters, and finally collecting into a Vec<String>
        Ok(TagFile {
            filepath: filepath.as_ref().to_string(),
            tags: std::str::from_utf8(
                bytes_from_file(filepath.as_ref(), None::<u64>, None::<u64>)?.as_slice(),
            )?
            .split_ascii_whitespace()
            .filter(|entry| !entry.is_empty())
            .map(|entry| entry.to_string())
            .collect::<TagCollection>(),
        })
    }

    pub fn generate_column_names<T: AsRef<str>>(
        column_count: u8,
        known_columns: Option<Vec<T>>,
    ) -> PyResult<Vec<String>> {
        let mut known_columns: Vec<String> = known_columns
            .unwrap_or_default()
            .iter()
            .map(|tag| tag.as_ref().to_string())
            .collect::<Vec<String>>();

        known_columns.extend(
            (known_columns.len()..(column_count as usize)).map(|val| format!("Column{}", val + 1)),
        );

        // There shouldn't ever be more than 255 columns in a table
        known_columns.truncate(255);

        Ok(known_columns)
    }

    // </editor-fold desc="// 'Public' Methods ...">
}

#[pymethods]
impl TagFile {
    #[new]
    fn __new__(filepath: Option<String>, tags: Option<TagCollection>) -> PyResult<Self> {
        let mut tag_file: TagFile = match filepath {
            None => Self::default(),
            Some(path) => Self::from_filepath(&path)?,
        };

        if let Some(mut t) = tags {
            tag_file.tags.append(t.as_mut());
        }

        Ok(tag_file)
    }

    fn __str__(slf: PyRefMut<Self>) -> PyResult<String> {
        Ok(format!("{}", *slf))
    }

    fn __repr__(slf: PyRefMut<Self>) -> PyResult<String> {
        Ok(format!("{}", *slf))
    }

    fn __getitem__(
        slf: PyRefMut<Self>,
        key: AttrIndexSliceOrItem<&'value str>,
    ) -> PyResult<ValueOrSlice<String>> {
        match key {
            AttrIndexSliceOrItem::Index(idx) => {
                let idx: isize = iif!(idx > -1, idx, slf.tags.len() as isize + idx);

                if idx < 0 {
                    return Err(PyIndexError::new_err(""));
                }

                match slf.tags.get(idx as usize) {
                    None => Err(PyIndexError::new_err("")),
                    Some(tag) => Ok(ValueOrSlice::Value(tag.to_string())),
                }
            }
            AttrIndexSliceOrItem::Slice(slc) => {
                let indexes: PySliceIndices = slc.indices(3)?;

                let (start, end) = (indexes.start, indexes.stop);

                let end: isize = iif!(end > -1, end, slf.tags.len() as isize + end);
                let start: isize = iif!(start > -1, start, slf.tags.len() as isize + start);

                if start < 0 || end < 0 {
                    return Err(PyIndexError::new_err(""));
                }

                let (start, end) = (start as usize, end as usize);

                match slf.tags.get(start..end) {
                    None => Err(PyIndexError::new_err("")),
                    Some(tags) => Ok(ValueOrSlice::Slice(tags.to_vec())),
                }
            }
            AttrIndexSliceOrItem::Item(attr) | AttrIndexSliceOrItem::Name(attr) => {
                match attr.to_lowercase().as_str() {
                    "tags" => Ok(ValueOrSlice::Slice(slf.tags.to_vec())),
                    "path" | "filepath" => Ok(ValueOrSlice::Value(slf.filepath.to_string())),
                    _ => Err(PyKeyError::new_err("")),
                }
            }
        }
    }

    fn pretty(slf: PyRefMut<Self>) -> String {
        slf._as_pretty_table().to_string()
    }
}

// </editor-fold desc="// Tag File ...">

// <editor-fold desc="// Tests ...">

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::{TagCollection, TagFile};

    #[test]
    /// Test that the `TagFile` structure behaves as expected
    fn gets_tag_files() {
        todo!()
    }
}

// </editor-fold desc="// Tests ...">
