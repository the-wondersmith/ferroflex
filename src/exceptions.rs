// Python-compatible error classes.

// Third-Party Imports
use pyo3::create_exception;
use pyo3::prelude::*;

// <editor-fold desc="// Component Registration ...">

/// Register the Rust code to be "exported" to Python
pub(crate) fn register_components(py: Python, ferroflex_module: &PyModule) -> PyResult<()> {
    // Create the `exceptions` sub-module
    let exceptions_module = PyModule::new(py, "ferroflex.exceptions")?;

    // Add the module-specific base exception class
    exceptions_module.add("FerricError", py.get_type::<FerricError>())?;

    // Add the file/table level exceptions
    exceptions_module.add("BCDDecodingError", py.get_type::<BCDDecodingError>())?;
    exceptions_module.add("DateDecodingError", py.get_type::<DateDecodingError>())?;
    exceptions_module.add(
        "TextFieldDecodingError",
        py.get_type::<TextFieldDecodingError>(),
    )?;

    // Add the DB API required exceptions
    exceptions_module.add("DataError", py.get_type::<DataError>())?;
    exceptions_module.add("DatabaseError", py.get_type::<DatabaseError>())?;
    exceptions_module.add("InternalError", py.get_type::<InternalError>())?;
    exceptions_module.add("IntegrityError", py.get_type::<IntegrityError>())?;
    exceptions_module.add("InterfaceError", py.get_type::<InterfaceError>())?;
    exceptions_module.add("OperationalError", py.get_type::<OperationalError>())?;
    exceptions_module.add("ProgrammingError", py.get_type::<ProgrammingError>())?;
    exceptions_module.add("NotSupportedError", py.get_type::<NotSupportedError>())?;

    // Add the populated sub-module to the top-level `ferroflex` module
    ferroflex_module.add("exceptions", exceptions_module)?;

    // Return an OK
    Ok(())
}

// </editor-fold desc="// Component Registration ...">

// <editor-fold desc="// Exceptions ...">

/// Base class for all `ferroflex` Python exceptions.
/// It is usable in Python code to catch any error raised
/// by the module itself with a single `except` statement.
create_exception!(ferroflex, FerricError, pyo3::exceptions::PyException);

// <editor-fold desc="// File/Table Level Exceptions ...">

/// Raised if the contents of any numeric-type column cannot
/// be properly read/decoded from a DataFlex table file.
create_exception!(ferroflex, BCDDecodingError, FerricError);

/// Raise if the contents of a DATE column cannot be properly
/// read/decoded from a DataFlex table file.
create_exception!(ferroflex, DateDecodingError, FerricError);

/// Raised if the contents of a TEXT column cannot be properly
/// read/decoded from a DataFlex table file.
create_exception!(ferroflex, TextFieldDecodingError, FerricError);

// </editor-fold desc="// File/Table Level Exceptions ...">

// <editor-fold desc="// DB API Required Exceptions ...">

// Top-level exceptions

/// Raised for errors related to the `ferroflex` module rather
/// than the DataFlex "database" itself.
create_exception!(ferroflex, InterfaceError, FerricError);

/// Raised for errors related to the DataFlex "database" itself.
create_exception!(ferroflex, DatabaseError, FerricError);

// Required subclasses

/// Raised for errors that are due to problems with the processed
/// data like division by zero, numeric value out of range, etc.
create_exception!(ferroflex, DataError, DatabaseError);

/// Raised when the database encounters an internal error, e.g.
/// the cursor is not valid anymore, the transaction is out of
/// sync, etc.
create_exception!(ferroflex, InternalError, DatabaseError);

/// Raised when the relational integrity of the database is affected,
/// e.g. a foreign key check fails.
create_exception!(ferroflex, IntegrityError, DatabaseError);

/// Raised for errors that are related to the database's operation
/// and not necessarily under the control of the programmer, e.g.
/// an unexpected disconnect occurs, the data source name is not
/// found, a transaction could not be processed, a memory allocation
/// error occurred during processing, etc.
create_exception!(ferroflex, OperationalError, DatabaseError);

/// Raised for programming errors, e.g. table not found or already
/// exists, syntax error in the SQL statement, wrong number of
/// parameters specified, etc.
create_exception!(ferroflex, ProgrammingError, DatabaseError);

/// Raised in case a method or database API was used which is not
/// supported by the database, e.g. requesting a .rollback() on a
/// connection that does not support transaction or has transactions
/// turned off.
create_exception!(ferroflex, NotSupportedError, DatabaseError);

// </editor-fold desc="// DB API Required Exceptions ...">

// </editor-fold desc="// Exceptions ...">
