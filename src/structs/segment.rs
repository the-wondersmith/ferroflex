// A structured representation of a field segment's definition in the header of a DataFlex table file

// Standard Library Imports
use std::fmt;

// Third-Party Imports
use prettytable::{Cell, Row as PrintableRow, Table as PrettyTable};
use pyo3;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

// <editor-fold desc="// Field Segment ...">

#[derive(Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq, Serialize, Deserialize)]
#[pyclass(dict, module = "ferroflex.structs")]
/// A structured representation of a field segment's
/// definition in the header of a DataFlex table file
pub struct FieldSegment {
    #[pyo3(get)]
    /// The column number (with respect to
    /// the column's parent table) to which
    /// the segment refers
    pub column: u8,
    #[pyo3(get)]
    /// The segment's position within its
    /// associated index
    pub segment: u8,
}

unsafe impl Send for FieldSegment {}

impl fmt::Display for FieldSegment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FieldSegment<column: {} | segment: {}>",
            self.column, self.segment
        )
    }
}

impl FieldSegment {
    // <editor-fold desc="// 'Private' Methods ...">

    fn _field_values(&self) -> Vec<(String, String)> {
        vec![
            ("Column Number".to_string(), self.column.to_string()),
            ("Field Segment".to_string(), self.segment.to_string()),
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

    pub fn from_bytes(data: &[u8]) -> PyResult<Vec<FieldSegment>> {
        Ok(data
            .iter()
            .enumerate()
            .map(|(i, col)| FieldSegment {
                column: *col,
                segment: i as u8,
            })
            .collect::<Vec<FieldSegment>>())
    }

    // </editor-fold desc="// Public Methods ...">
}

#[pymethods]
impl FieldSegment {
    #[new]
    fn __new__(column: Option<u8>, segment: Option<u8>) -> Self {
        Self {
            column: column.unwrap_or_default(),
            segment: segment.unwrap_or_default(),
        }
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
}

// </editor-fold desc="// Field Segment ...">
