#![feature(
    generic_associated_types,
    associated_type_bounds,
    in_band_lifetimes,
    never_type
)]
#![allow(dead_code, unused_doc_comments)]
#![allow(clippy::needless_option_as_deref)]
// A Rust interface for DataFlex flat-file databases w/ DB-API v2 compliant Python bindings

extern crate core;

// Module Declarations
pub mod dbapi;
pub mod enums;
pub mod exceptions;
pub mod sql;
pub mod structs;
pub mod utils;

// Third-Party Imports
use pyo3::prelude::*;
use pyo3::types::PySlice;
use serde::{Deserialize, Serialize};

// <editor-fold desc="// Macros ...">

/// iif!(condition, result_when_true, result_when_false)
#[macro_export]
macro_rules! iif {
    ($c:expr, $t:expr, $f:expr) => {
        if $c {
            $t
        } else {
            $f
        }
    };
}

// </editor-fold desc="// Macros ...">

// <editor-fold desc="// Top-Level Python Module ...">

#[pymodule]
/// A DB-API v2 compliant interface for DataFlex flat-file databases implemented in Rust.
pub fn ferroflex(py: Python, module: &PyModule) -> PyResult<()> {
    // Register all the Rust-native components
    // that should be "exported" to Python
    dbapi::register_components(py, module)?;
    exceptions::register_components(py, module)?;
    utils::register_components(py, module)?;

    // // Call the `register` function from the other crate-level modules
    // structs::register_components(py, module)?;
    //
    // // Call the `register` function from the `sql` sub-module
    // sql::register_components(py, module)?;

    // Return an OK
    Ok(())
}

// </editor-fold desc="// Top-Level Python Module ...">

// <editor-fold desc="// Custom Types ...">

pub trait PyGetterValue<'a, T>
where
    T: Clone + PartialEq<T> + FromPyObject<'a>,
{
}

#[derive(Clone, Debug, PartialEq, FromPyObject)]
pub enum AttrIndexSliceOrItem<'value, ItemType>
where
    dyn PyGetterValue<'value, ItemType>:,
{
    Index(isize),
    Name(&'value str),
    Item(ItemType),
    Slice(&'value PySlice),
}

#[derive(Clone, Debug, PartialEq, FromPyObject, Serialize, Deserialize)]
pub enum ValueOrSlice<T> {
    #[pyo3(transparent)]
    Value(T),
    #[pyo3(transparent)]
    Slice(Vec<T>),
}

impl<RustObject> IntoPy<PyObject> for ValueOrSlice<RustObject>
where
    RustObject: IntoPy<PyObject>,
{
    fn into_py(self, py: Python) -> PyObject {
        match self {
            Self::Value(val) => IntoPy::into_py(val, py),
            Self::Slice(val) => IntoPy::into_py(val, py),
        }
    }
}

// </editor-fold desc="// Custom Types ...">
