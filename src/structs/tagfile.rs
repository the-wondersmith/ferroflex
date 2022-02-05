// A structured representation of a DataFlex `.tag` file

// Standard Library Imports
use std::fmt;
use std::fmt::Formatter;
use std::iter::IntoIterator;
use std::ops::Index as Indexable;
use std::path::Path;
use std::slice::SliceIndex;

// Third-Party Imports
use prettytable::{Cell as PrettyCell, Row as PrettyRow, Table as PrettyTable};
use pyo3::PyResult;
use serde::{Deserialize, Serialize};

// Crate-Level Imports
use crate::utils::{bytes_from_file, string_from_path};

// <editor-fold desc="// Custom Types ...">

pub type TagCollection = Vec<String>;

// </editor-fold desc="// Custom Types ...">

// <editor-fold desc="// Tag File ...">

#[derive(Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq, Serialize, Deserialize)]
/// A structured representation of a field segment's
/// definition in the header of a DataFlex table file
pub struct TagFile {
    /// The file's absolute on-disk path
    pub filepath: String,
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

    pub(crate) fn _as_pretty_table(&self) -> String {
        PrettyTable::from_iter([
            PrettyRow::from(vec![
                PrettyCell::new("filepath"),
                PrettyCell::new(&self.filepath),
            ]),
            PrettyRow::from(vec![
                PrettyCell::new("tags"),
                PrettyCell::new(format!(" {} ", self.tags.join(" ¦ ")).as_str()),
            ]),
        ])
        .to_string()
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
