// ferroflex's bindings to [GlueSQL](https://github.com/gluesql/gluesql)

pub(crate) mod alter;
pub(crate) mod index;
pub(crate) mod metadata;
pub(crate) mod store;
pub(crate) mod transaction;

// // Third-Party Imports
// use pyo3::types::PyModule;
// use pyo3::{PyResult, Python};

// // <editor-fold desc="// Component Registration ...">
//
// /// Register the Rust code to be "exported" to Python
// pub(crate) fn register_components(py: Python, ferroflex_module: &PyModule) -> PyResult<()> {
//     // Create the `sql` sub-module
//     let sql_module = PyModule::new(py, "ferroflex.sql")?;
//
//     // // Add the constructor function(s) to the module
//     // structures_module.add_function(pyo3::wrap_pyfunction!(connect, dbapi_module)?)?;
//     //
//     // // Add the class objects to the module
//     // structures_module.add_class::<Cursor>()?;
//
//     // Add the populated sub-module to the top-level `ferroflex` module
//     ferroflex_module.add("sql", sql_module)?;
//
//     // Return an OK
//     Ok(())
// }
//
// // </editor-fold desc="// Component Registration ...">
