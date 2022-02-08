// INSERT DOCSTRING HERE

// Standard Library Imports
use std::borrow::Borrow;

// Third-Party Imports
use gluesql::prelude::*;
use pyo3::prelude::*;
use pyo3::types::{PyFunction, PyTuple};

// Crate-Level Imports
use crate::structs::DataFlexDB;

// Sub-Module Level Imports
use super::Cursor;

// <editor-fold desc="// Connection ...">

// #[derive(Clone, Debug, Default)]
#[pyclass(dict, module = "ferroflex.dbapi")]
/// A standard DB-API v2 Connection object.
pub struct Connection {
    //
    // Python-Accessible Attributes
    //
    #[pyo3(get)]
    /// Indicates if the connection is open or not
    pub closed: bool,
    #[pyo3(get)]
    /// The total number of database rows that have
    /// been modified, inserted, or deleted since the
    /// database connection was opened.
    pub total_changes: usize,
    #[pyo3(get)]
    /// Indicates the whether or not the connection
    /// currently has uncommitted changes (i.e. an
    /// active transaction).
    pub in_transaction: bool,
    #[pyo3(get, set)]
    /// The current default isolation level
    pub isolation_level: usize,
    #[pyo3(get)]
    /// INSERT DOCSTRING HERE
    pub row_factory: Option<PyObject>,
    #[pyo3(get)]
    /// INSERT DOCSTRING HERE
    pub text_factory: Option<PyObject>,
    //
    // Rust-only "Internal" Attributes
    //
    /// The connection's GlueSQL engine
    pub(crate) sql_engine: Glue<usize, DataFlexDB>,
}

unsafe impl Send for Connection {}

impl Default for Connection {
    fn default() -> Self {
        Self {
            closed: false,
            total_changes: 0,
            in_transaction: false,
            isolation_level: 0,
            row_factory: None,
            text_factory: None,
            sql_engine: Glue::new(DataFlexDB::default()),
        }
    }
}

impl Connection {
    pub fn new<P: AsRef<str>>(db_path: P) -> PyResult<Self> {
        Ok(Self {
            sql_engine: Glue::new(DataFlexDB::from_path(db_path)?),
            ..Self::default()
        })
    }
}

#[pymethods]
impl Connection {
    // <editor-fold desc="// Ferroflex-Specific Methods ...">

    #[pyo3(text_signature = "(self) -> str")]
    /// The path of the DataFlex "database" being
    /// connected to
    pub fn db_path(slf: PyRef<Self>) -> PyResult<String> {
        Ok(slf
            .sql_engine
            .borrow()
            .storage
            .as_ref()
            .unwrap()
            .borrow()
            .db_path
            .to_str()
            .unwrap()
            .to_string())
    }

    // </editor-fold desc="// Ferroflex-Specific Methods ...">

    // <editor-fold desc="// DB API Required Methods ...">

    #[allow(unused_variables)]
    #[pyo3(text_signature = "(self) -> None")]
    /// Close the connection now (rather than `__del__()` is called).
    /// The connection will be unusable from this point forward; an exception
    /// will be raised if any operation is attempted with the connection. The
    /// same applies to all cursor objects trying to use the connection. Note
    /// that closing a connection without committing the changes first will
    /// cause an implicit rollback to be performed.
    pub fn close(slf: PyRefMut<Self>) -> PyResult<()> {
        todo!()
    }

    #[allow(unused_variables)]
    #[pyo3(text_signature = "(self) -> None")]
    /// Commits the current transaction.
    /// If this method is not called, the results of any query
    /// executed since the last call to `commit()` will not visible
    /// from other connections.
    pub fn commit(slf: PyRefMut<Self>) -> PyResult<()> {
        todo!()
    }

    #[allow(unused_variables)]
    #[pyo3(text_signature = "(self) -> None")]
    /// Rolls back any changes to the database since the last
    /// call to `commit()`.
    pub fn rollback(slf: PyRefMut<Self>) -> PyResult<()> {
        todo!()
    }

    #[allow(unused_variables)]
    #[pyo3(text_signature = "(self) -> Cursor")]
    /// Create a new `Cursor` object using the connection.
    pub fn cursor(slf: PyRefMut<Self>) -> PyResult<Py<Cursor>> {
        todo!()
    }

    // </editor-fold desc="// DB API Required Methods ...">

    // <editor-fold desc="// SQLite Emulation Methods ...">

    #[allow(unused_variables)]
    #[pyo3(text_signature = "(self, \
        sql: str, \
        parameters: Optional[Union[Sequence[Any], Mapping[str, Any]]] \
        ) -> Optional[Sequence[CursorDescription]]")]
    /// A nonstandard shortcut that creates a cursor object by first
    /// calling the connection's `cursor` method, then calling the
    /// resulting cursor’s `execute` method with the parameters
    /// given before returning the newly created cursor.
    ///
    /// Parameters may be provided as a sequence or mapping and will
    /// be bound to variables in the operation.
    pub fn execute(
        slf: PyRefMut<Self>,
        py: Python,
        sql: &str,
        parameters: Option<Vec<&PyAny>>,
    ) -> PyResult<Py<Cursor>> {
        let cursor = Py::new(
            py,
            Cursor {
                connection: slf.into(),
                rowcount: None,
                rownumber: None,
                lastrowid: None,
                arraysize: 0,
                description: None,
                messages: None,
                results: None,
                input_size: 1,
                output_size: 1,
            },
        )?;

        let _ = Cursor::execute(cursor.borrow_mut(py), py, sql, parameters);

        Ok(cursor)
    }

    #[allow(unused_variables)]
    #[pyo3(text_signature = "(self, \
        sql: str, \
        parameters: Optional[Union[Sequence[Any], Mapping[str, Any]]] \
        ) -> Optional[Sequence[Sequence[CursorDescription]]]")]
    /// A nonstandard shortcut that creates a cursor object by first
    /// calling the connection's `cursor` method, then calling the
    /// resulting cursor’s `executemany` method with the parameters
    /// given before returning the newly created cursor.
    ///
    /// Parameters may be provided as a sequence or mapping and will
    /// be bound to variables in the operation.
    pub fn executemany(
        slf: PyRefMut<Self>,
        sql: &str,
        parameters: Option<Vec<Vec<&PyAny>>>,
    ) -> PyResult<()> {
        todo!()
    }

    #[allow(unused_variables)]
    #[pyo3(text_signature = "(self, sql_script: str) -> None")]
    /// A nonstandard shortcut that creates a cursor object by first
    /// calling the connection's `cursor` method, then calling the
    /// resulting cursor’s `executescript` method with the given
    /// `sql_script` before returning the newly created cursor.
    pub fn executescript(slf: PyRefMut<Self>, sql_script: &str) -> PyResult<()> {
        todo!()
    }

    #[allow(unused_variables)]
    #[pyo3(text_signature = "(self, \
        target: Connection, \
        *, \
        pages: int = -1, \
        progress: Optional[Callable[[int, int, int], None]] = None, \
        name: str = 'main', \
        sleep: float = 0.25\
        ) -> Connection")]
    /// This method makes a backup of a SQLite database even while
    /// it’s being accessed by other clients, or concurrently by
    /// the same connection. The copy will be written into the
    /// mandatory argument `target`, that must be another Connection
    /// instance.
    ///
    /// By default, or when pages is either 0 or a negative integer,
    /// the entire database is copied in a single step; otherwise the
    /// method performs a loop copying up to pages pages at a time.
    ///
    /// If progress is specified, it must either be None or a callable
    /// object that will be executed at each iteration with three integer
    /// arguments, respectively the status of the last iteration, the
    /// remaining number of pages still to be copied and the total number
    /// of pages.
    ///
    /// The name argument specifies the database name that will be copied:
    /// it must be a string containing either "main", the default, to
    /// indicate the main database, "temp" to indicate the temporary
    /// database or the name specified after the AS keyword in an
    /// ATTACH DATABASE statement for an attached database.
    ///
    /// The sleep argument specifies the number of seconds to sleep by
    /// between successive attempts to backup remaining pages, can be
    /// specified either as an integer or a floating point value.
    pub fn backup(
        slf: PyRefMut<Self>,
        target: &Connection,
        py_args: &PyTuple,
        pages: Option<i64>,
        progress: Option<&PyFunction>,
        name: Option<&str>,
        sleep: Option<f64>,
    ) -> PyResult<()> {
        todo!()
    }

    #[allow(unused_variables)]
    #[pyo3(text_signature = "(self) -> None")]
    /// Abort any queries that might be executing on the connection.
    /// The query will then abort and the caller will get an exception.
    pub fn interrupt(slf: PyRefMut<Self>) -> PyResult<()> {
        todo!()
    }

    #[allow(unused_variables)]
    #[pyo3(text_signature = "(self, \
        name: str, \
        num_params: int, \
        aggregate_class: Type[Any]) -> None")]
    /// Create a user-defined aggregate function.
    ///
    /// The specified aggregate class must implement a `step`
    /// method, which accepts the number of parameters `num_params`
    /// (if num_params is -1, the function may take any number of
    /// arguments), and a `finalize` method which will return the
    /// final result of the aggregate.
    ///
    /// The `finalize` method can return any of the types supported
    /// by DataFlex: bytes, str, int, float, date, and None.
    pub fn create_aggregate(
        slf: PyRefMut<Self>,
        name: &str,
        num_params: u8,
        aggregate_class: &PyAny,
    ) -> PyResult<()> {
        // Ok(())
        todo!()
    }

    #[allow(unused_variables)]
    #[pyo3(text_signature = "(self, \
        name: str, \
        callable: Callable[[str, str], int]) -> None")]
    /// Creates a collation with the specified name and callable.
    ///
    /// The callable will be passed two string arguments. It should
    /// return -1 if the first is ordered lower than the second, 0 if
    /// they are ordered equal and 1 if the first is ordered higher than
    /// the second. Note that this controls sorting (ORDER BY in SQL)
    /// so your comparisons don’t affect other SQL operations.
    ///
    /// NOTE: the callable will get its parameters as Python bytestrings,
    /// which will normally be encoded in UTF-8.
    pub fn create_collation(
        slf: PyRefMut<Self>,
        name: &str,
        callable: &PyFunction,
    ) -> PyResult<()> {
        todo!()
    }

    #[allow(unused_variables)]
    #[pyo3(text_signature = "(self, \
        name: str, \
        num_params: int, \
        func: Callable[[Optional[Any], ...], Optional[Any]]) -> None")]
    /// Create a user-defined function that can be used from
    /// within SQL statements under the function name `name`.
    ///
    /// The `num_params` argument is the number of parameters
    /// the function accepts (if num_params is -1, the function
    /// may take any number of arguments), and `func` is a
    /// Python callable that is called as the SQL function.
    pub fn create_function(
        slf: PyRefMut<Self>,
        name: &str,
        num_params: u8,
        func: &PyFunction,
    ) -> PyResult<()> {
        todo!()
    }

    // </editor-fold desc="// SQLite Emulation Methods ...">
}

// </editor-fold desc="// Connection ...">
