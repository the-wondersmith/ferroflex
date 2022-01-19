// A structured representation of a single row of data in DataFlex table file

// Standard Library Imports
use std::fmt::{Display, Formatter, Result as FormatResult};
use std::ops::Deref;

// Third-Party Imports
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

// Crate-Level Imports
use crate::enums::Value;

// <editor-fold desc="// Row ...">

#[derive(Clone, Debug, Default, PartialOrd, PartialEq, Serialize, Deserialize, FromPyObject)]
pub struct Row(#[pyo3(transparent)] pub Vec<Value>);

unsafe impl Send for Row {}

impl Display for Row {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        write!(
            f,
            "Row<{}>",
            self.0
                .iter()
                .map(|value| value.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl Deref for Row {
    type Target = Vec<Value>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ToPyObject for Row {
    fn to_object(&self, py: Python) -> PyObject {
        self.0
            .iter()
            .map(|value| value.clone().into_py(py))
            .collect::<Vec<PyObject>>()
            .into_py(py)
    }
}

// </editor-fold desc="// Row ...">

// <editor-fold desc="// Tests ...">

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::Row;

    #[test]
    /// Test that the `Row` structure correctly handles table data
    fn gets_rows() {
        todo!()
    }
}

// </editor-fold desc="// Tests ...">
