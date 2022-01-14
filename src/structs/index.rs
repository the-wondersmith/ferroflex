// A structured representation of an index's definition in the header of a DataFlex table file

// Standard Library Imports
use std::fmt;
use std::iter::IntoIterator;

// Third-Party Imports
use gluesql::core::data::SchemaIndex; // SchemaIndexOrd
use prettytable::{Cell, Row as PrintableRow, Table as PrettyTable};
use pyo3;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

// Crate-Level Imports
use crate::enums::{IndexCollation, IndexType};
use crate::structs::segment::FieldSegment;

// <editor-fold desc="// Index ...">

#[derive(Clone, Debug, Eq, Ord, PartialOrd, PartialEq, Serialize, Deserialize)]
#[pyclass(dict, module = "ferroflex.structs")]
/// A structured representation of an index's
/// definition in the header of a DataFlex table file
pub struct Index {
    // #[pyo3(get)]
    /// Denotes the index as a "batch" index
    pub r#type: IndexType,
    #[pyo3(get)]
    /// The total number of columns whose
    /// data make up the indexed rows
    pub field_count: u8,
    #[pyo3(get)]
    /// The index's field segments
    pub segments: Vec<FieldSegment>,
    // #[pyo3(get)]
    /// Denotes the index's "type"
    pub collation: IndexCollation,
}

unsafe impl Send for Index {}

impl Default for Index {
    fn default() -> Self {
        Index {
            r#type: IndexType::Unknown,
            field_count: 0u8,
            segments: vec![],
            collation: IndexCollation::Unknown,
        }
    }
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Index<type: {} | field_count: {} | segments: {} | collation: {}>",
            self.r#type,
            self.field_count,
            self.segments
                .iter()
                .map(|seg| seg.to_string())
                .collect::<Vec<String>>()
                .join(", "),
            self.collation,
        )
    }
}

impl IntoIterator for Index {
    type Item = FieldSegment;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.segments.into_iter()
    }
}

impl Index {
    // <editor-fold desc="// 'Private' Methods ...">

    fn _field_values(&self) -> Vec<(String, String)> {
        let mut table = PrettyTable::new();

        table.add_row(PrintableRow::new(
            self.segments
                .iter()
                .map(|seg| Cell::new(&seg.to_string()))
                .collect::<Vec<Cell>>(),
        ));

        vec![
            ("Type".to_string(), self.r#type.to_string()),
            ("Field Count".to_string(), self.field_count.to_string()),
            ("Collation".to_string(), self.collation.to_string()),
            ("Segments".to_string(), table.to_string()),
        ]
    }

    fn _as_pretty_table(&self) -> PrettyTable {
        let mut table = PrettyTable::new();

        self._field_values().iter().for_each(|pair| {
            table.add_row(PrintableRow::new(vec![
                Cell::new(&pair.0),
                Cell::new(&pair.1),
            ]));
        });

        table
    }

    // </editor-fold desc="// 'Private' Methods ...">

    // <editor-fold desc="// Public Methods ...">

    pub fn schema(&self) -> SchemaIndex {
        // SchemaIndex {
        //     name: String,
        //     expr: Expr,
        //     order: SchemaIndexOrd,
        // }
        todo!()
    }

    pub fn from_bytes(data: &[u8]) -> PyResult<Index> {
        let end: usize = match data.len() < 18 {
            true => 7,
            false => 17,
        };

        Ok(Index {
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
        })
    }

    pub fn table_from_bytes(data: &[u8]) -> PyResult<Vec<Index>> {
        let chunk_size: usize = match data.len() % 18 != 0 {
            true => 8,
            false => 18,
        };

        Ok(data
            .chunks_exact(chunk_size)
            .map(|chunk| Index::from_bytes(chunk).unwrap())
            .filter(|index| index.field_count > 0)
            .collect::<Vec<Index>>())
    }

    // </editor-fold desc="// Public Methods ...">
}

#[pymethods]
impl Index {
    #[new]
    fn __new__(
        r#type: Option<String>,
        field_count: Option<u8>,
        segments: Option<Vec<FieldSegment>>,
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
            segments: segments.unwrap_or_default(),
            collation: IndexCollation::from(collation.expect("Unknown `IndexCollation` type!")),
        })
    }

    fn __str__(slf: PyRefMut<Self>) -> PyResult<String> {
        Ok(format!("{}", *slf))
    }

    fn __repr__(slf: PyRefMut<Self>) -> PyResult<String> {
        Ok(format!("{}", *slf))
    }

    fn pretty(slf: PyRefMut<Self>) -> String {
        slf._as_pretty_table().to_string()
    }

    #[getter(r#type)]
    fn get_index_type(slf: PyRefMut<Self>) -> PyResult<String> {
        Ok(slf.r#type.to_string())
    }

    #[getter(collation)]
    fn get_collation(slf: PyRefMut<Self>) -> PyResult<String> {
        Ok(slf.collation.to_string())
    }
}

// </editor-fold desc="// Index ...">
