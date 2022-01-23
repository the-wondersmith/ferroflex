// A structured representation of an index's definition in the header of a DataFlex table file

// Standard Library Imports
use std::fmt;
use std::iter::IntoIterator;

// Third-Party Imports
use prettytable::{Cell as PrettyCell, Row as PrettyRow, Table as PrettyTable};
use pyo3;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

// Crate-Level Imports
use crate::enums::{IndexCollation, IndexType};
use crate::structs::segment::FieldSegment;

// <editor-fold desc="// Index ...">

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[pyclass(dict, module = "ferroflex.structs")]
/// A structured representation of an index's
/// definition in the header of a DataFlex table file
pub struct Index {
    #[pyo3(get)]
    /// Denotes the index as a "batch" index
    pub r#type: IndexType,
    #[pyo3(get)]
    /// The total number of columns whose
    /// data make up the indexed rows
    pub field_count: u8,
    #[pyo3(get)]
    /// The index's field segments
    pub segments: Vec<Py<FieldSegment>>,
    #[pyo3(get)]
    /// Denotes the index's "type"
    pub collation: IndexCollation,
}

unsafe impl Send for Index {}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Python::with_gil(|py| {
            write!(
                f,
                "Index<type: {} | field_count: {} | segments: {} | collation: {}>",
                self.r#type,
                self.field_count,
                self.segments
                    .iter()
                    .map(|seg| Py::borrow(seg, py).to_string())
                    .collect::<Vec<String>>()
                    .join(", "),
                self.collation,
            )
        })
    }
}

impl IntoIterator for Index {
    type Item = Py<FieldSegment>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.segments.into_iter()
    }
}

impl Index {
    // <editor-fold desc="// 'Private' Methods ...">

    pub(crate) fn _as_pretty_table(&self) -> String {
        // Create the "outer" table element
        let (mut outer, mut left, mut right) =
            (PrettyTable::new(), PrettyTable::new(), PrettyTable::new());

        vec![
            ("type", (&self.r#type).to_string()),
            ("fields", (&self.field_count).to_string()),
            ("collation", (&self.collation).to_string()),
        ]
        .iter()
        .for_each(|(key, value)| {
            left.add_row(PrettyRow::from(vec![
                PrettyCell::new(key),
                PrettyCell::new(value),
            ]));
        });

        Python::with_gil(|py| {
            right.add_row(PrettyRow::from(self.segments.iter().map(|segment| {
                PrettyCell::new(Py::borrow(segment, py)._as_pretty_table().as_str())
            })))
        });

        outer.add_row(PrettyRow::from(vec![
            PrettyCell::new(left.to_string().as_str()),
            PrettyCell::new(right.to_string().as_str()),
        ]));

        outer.to_string()
    }

    // </editor-fold desc="// 'Private' Methods ...">

    // <editor-fold desc="// Public Methods ...">

    pub fn from_bytes(data: &[u8]) -> PyResult<Py<Index>> {
        let end: usize = match data.len() < 18 {
            true => 7,
            false => 17,
        };

        Python::with_gil(|py| {
            Py::new(py, {
                let idx = Index {
                    r#type: IndexType::from(data[0] >= 128),
                    field_count: match data[0] < 128 {
                        true => data[0],
                        false => data[0] - 128,
                    },
                    segments: FieldSegment::from_bytes(&data[1..end])?,
                    collation: match data[end] {
                        0 => IndexCollation::Default,
                        1 => IndexCollation::Ascending,
                        2 => IndexCollation::Uppercase,
                        _ => IndexCollation::Unknown,
                    },
                };

                if idx.field_count < 1 {
                    return Err(PyValueError::new_err(
                        "Indexes must involve at least one field!",
                    ));
                }

                idx
            })
        })
    }

    pub fn table_from_bytes(data: &[u8]) -> PyResult<Vec<Py<Index>>> {
        let chunk_size: usize = match data.len() % 18 != 0 {
            true => 8,
            false => 18,
        };

        Ok(data
            .chunks_exact(chunk_size)
            .map(|chunk| Index::from_bytes(chunk))
            .filter(PyResult::is_ok)
            .map(PyResult::unwrap)
            .collect::<Vec<Py<Index>>>())
    }

    // </editor-fold desc="// Public Methods ...">
}

#[pymethods]
impl Index {
    #[new]
    fn __new__(
        r#type: Option<String>,
        field_count: Option<u8>,
        segments: Option<Vec<Py<FieldSegment>>>,
        collation: Option<String>,
    ) -> PyResult<Self> {
        Ok(Self {
            r#type: match r#type {
                None => IndexType::Unknown,
                Some(val) => match val.to_lowercase().as_str() {
                    "batch" | "true" => IndexType::Batch,
                    "online" | "false" => IndexType::Online,
                    _ => IndexType::Unknown,
                },
            },
            field_count: field_count.unwrap_or_default(),
            segments: segments.unwrap_or_else(|| Vec::new()),
            collation: IndexCollation::from(collation.expect("Unknown `IndexCollation` type!")),
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

// </editor-fold desc="// Index ...">

// <editor-fold desc="// Tests ...">

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::Index;

    #[test]
    /// Test that the `Index` structure behaves as expected
    fn gets_indexes() {
        todo!()
    }
}

// </editor-fold desc="// Tests ...">
