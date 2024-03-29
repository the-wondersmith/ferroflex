// A structured representation of a field segment's definition in the header of a DataFlex table file

// Standard Library Imports
use std::fmt;

// Third-Party Imports
use prettytable::{Cell as PrettyCell, Row as PrettyRow, Table as PrettyTable};
use pyo3::PyResult;
use serde::{Deserialize, Serialize};

// <editor-fold desc="// Field Segment ...">

#[derive(Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq, Serialize, Deserialize)]
/// A structured representation of a field segment's
/// definition in the header of a DataFlex table file
pub struct FieldSegment {
    /// The column number (with respect to
    /// the column's parent table) to which
    /// the segment refers
    pub column: u8,
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

    pub(crate) fn _as_pretty_table(&self) -> String {
        PrettyTable::from_iter([
            PrettyRow::new(vec![
                PrettyCell::new("column_number"),
                PrettyCell::new(&self.column.to_string().as_str()),
            ]),
            PrettyRow::new(vec![
                PrettyCell::new("field_segment"),
                PrettyCell::new(&self.segment.to_string().as_str()),
            ]),
        ])
        .to_string()
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

// </editor-fold desc="// Field Segment ...">

// <editor-fold desc="// Tests ...">

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::FieldSegment;

    #[test]
    /// Test that the `FieldSegment` structure behaves as expected
    fn gets_segments() {
        todo!()
    }
}

// </editor-fold desc="// Tests ...">
