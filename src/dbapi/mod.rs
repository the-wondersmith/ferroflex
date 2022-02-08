// A DB API v2 compliant Python module
// see: [PEP 249](https://www.python.org/dev/peps/pep-0249)

// Submodule Declarations
pub(crate) mod connection;
pub(crate) mod cursor;
pub(crate) mod globals;
pub(crate) mod types;
pub(crate) mod wrappers;

// Third-Party Imports
use pyo3::prelude::*;

// Sub-Module "Exports"
pub use connection::Connection;
pub use cursor::{Cursor, CursorDescription};
pub use globals::*;
pub use types::*;
pub use wrappers::{FerricRow, FerricValue};

// <editor-fold desc="// Component Registration ...">

/// Register the Rust code to be "exported" to Python
pub(crate) fn register_components(py: Python, ferroflex_module: &PyModule) -> PyResult<()> {
    // Create the `dbapi` sub-module
    let dbapi_module = PyModule::new(py, "ferroflex.dbapi")?;

    // Add all the constants as top-level module attributes
    dbapi_module.add("apilevel", APILEVEL)?;
    dbapi_module.add("paramstyle", PARAMSTYLE)?;
    dbapi_module.add("threadsafety", THREADSAFETY)?;

    // Add the constructor function(s) to the module
    dbapi_module.add_function(pyo3::wrap_pyfunction!(connect, dbapi_module)?)?;

    // Add the class objects to the module
    dbapi_module.add_class::<Cursor>()?;
    dbapi_module.add_class::<Connection>()?;
    dbapi_module.add_class::<CursorDescription>()?;

    // Add the populated sub-module to the top-level `ferroflex` module
    ferroflex_module.add("dbapi", dbapi_module)?;

    // Return an OK
    Ok(())
}

// </editor-fold desc="// Component Registration ...">

// <editor-fold desc="// Constructors ...">

#[allow(unused_variables)]
#[pyfunction]
#[pyo3(text_signature = "(\
    database: str, \
    /, \
    uri: bool = False, \
    timeout: Optional[int] = 0, \
    isolation_level: Optional[str] = None\
) -> ferroflex.dbapi.Connection \
")]
/// Constructor for creating a "connection" to a DataFlex "database" directory
fn connect(
    database: &str,
    uri: Option<bool>,
    timeout: Option<u16>,
    isolation_level: Option<usize>,
) -> PyResult<Connection> {
    // database - path to either the `filelist.cfg` file of the target "database"
    //            -OR-
    //            a directory containing dataflex `.dat` and `.tag` files
    // timeout - (not currently implemented) how to wait if a read-lock can't
    //            be acquired on the target file(s) before raising an error
    // isolation_level - (not currently implemented) the transaction isolation
    //                    style the connection should emulate / use
    // uri - (not currently implemented) indicates that the string supplied as
    //       `database` should be interpreted as a URI allowing the user to
    //        specify additional options.

    Connection::new(database)
}

// </editor-fold desc="// Constructors ...">

// <editor-fold desc="// Tests ...">

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::{Connection, Cursor, CursorDescription};

    #[test]
    /// Test that the `Connection` structure behaves as expected
    fn connects() {
        todo!()
    }

    #[test]
    /// Test that the `Cursor` structure behaves as expected
    fn creates_cursors() {
        todo!()
    }

    #[test]
    /// Test that the `CursorDescription` structure behaves as expected
    fn cursors_are_descriptive() {
        todo!()
    }
}

// </editor-fold desc="// Tests ...">
