#![allow(unused_imports)]
// Integration tests for the `ferroflex` DB API interface

// Third-Party Imports
use pyo3::prelude::*;
use pyo3::types::PyCFunction;

// Crate-Level Imports
use ferroflex::*;

// Module "Inclusions"
mod common;

#[pyfunction]
fn it_works(value: Option<&str>) -> PyResult<String> {
    Ok(match value {
        None => String::new(),
        Some(val) => val.to_string(),
    })
}

#[test]
fn test_optional_bool() -> PyResult<()> {
    // Acquire the Python GIL
    let gil = Python::acquire_gil();
    let py = gil.python();

    // Transform the annotated function above into a `PyCFunction`
    let f: &PyCFunction = wrap_pyfunction!(it_works)(py)?;

    py_assert!(py, f, "f() == ''");
    py_assert!(py, f, "f('it works!') == 'it works!'");

    Ok(())
}
