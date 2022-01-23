// A structured representation of a single row of data in DataFlex table file

// Standard Library Imports
use itertools::Itertools;
use std::fmt::{Display, Formatter, Result as FormatResult};
use std::ops::Deref;

// Third-Party Imports
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

// Crate-Level Imports
use crate::enums::Value;

// <editor-fold desc="// Row ...">

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[pyclass(dict, module = "ferroflex.structs")]
/// A structured representation of the data
/// in a single row of a DataFlex table file
pub struct Row {
    pub data: Vec<Value>,
}

unsafe impl Send for Row {}

impl Display for Row {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        write!(
            f,
            "Row<{}>",
            self.data.iter().map(|value| value.to_string()).join(", ")
        )
    }
}

impl Deref for Row {
    type Target = Vec<Value>;

    fn deref(&self) -> &Self::Target {
        &self.data
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
