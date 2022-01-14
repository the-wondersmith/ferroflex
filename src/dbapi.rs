// A DB API v2 compliant Python module
// see: [PEP 249](https://www.python.org/dev/peps/pep-0249)

// Third-Party Imports
use pyo3;
use pyo3::exceptions::{PyAttributeError, PyIndexError};
use pyo3::prelude::*;
use pyo3::types::PySliceIndices;

// Crate-Level Imports
use crate::{iif, AttrIndexOrSlice, ValueOrSlice};

// NOTE: At the time of this writing (2022-01-01), this crate is written with a prohibition
//       against unsafe Rust code. As such, none of the structs exported to Python as classes
//       implement `core::marker::Send` and are therefore marked "unsendable". This is also
//       reflected in the value of the `THREADSAFETY` constant below.
//
//       In order to "upgrade" the thread safety level of the module, rust's `Send` trait must
//       be implemented for (at a minimum) the `Connection` and `Cursor` structs. It *may* be
//       sufficient to simply add an "empty" impl block for each of them. At the time of this
//       writing it is currently unclear. For reference, an empty impl block for the `Cursor`
//       struct would simply be: unsafe impl Send for Cursor {}

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
    database: String,
    timeout: Option<u16>,
    isolation_level: Option<String>,
    uri: Option<bool>,
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

    Ok(Connection {
        db_path: Some(database),
        ..Connection::default()
    })
}

// </editor-fold desc="// Constructors ...">

// <editor-fold desc="// Globals ...">

/// String constant stating the supported DB API level.
/// Per PEP 249, only the strings "1.0" and "2.0" are allowed.
pub const APILEVEL: &str = "2.0";
/// Integer constant stating the level of thread safety the interface supports.
/// Possible values are:
///   - 0: Threads may not share the module
///   - 1: Threads may share the module, but not connections
///   - 2: Threads may share the module and connections
///   - 3: Threads may share the module, connections and cursors
pub const THREADSAFETY: usize = 1;
/// String constant stating the type of parameter marker formatting expected by the interface.
/// Possible values are:
///  - "qmark": Question mark style, e.g. ...WHERE name=?
///  - "numeric": Numeric, positional style, e.g. ...WHERE name=:1
///  - "named": Named style, e.g. ...WHERE name=:name
///  - "format": ANSI C printf format codes, e.g. ...WHERE name=%s
///  - "pyformat": Python extended format codes, e.g. ...WHERE name=%(name)s
pub const PARAMSTYLE: &str = "qmark";

// </editor-fold desc="// Globals ...">

// <editor-fold desc="// Objects ...">

// <editor-fold desc="// Connection ...">

#[derive(Clone, Debug, Default)]
#[pyclass(dict, module = "ferroflex.dbapi")]
/// A standard DB-API v2 Connection object.
pub struct Connection {
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
    #[pyo3(get)]
    /// The current default isolation level
    pub isolation_level: Option<String>,
    #[pyo3(get)]
    /// The path of the DataFlex "database" being
    /// connected to
    pub db_path: Option<String>,
}

unsafe impl Send for Connection {}

#[allow(unused_variables)]
#[pymethods]
impl Connection {
    #[new]
    fn new() -> Self {
        Connection::default()
    }

    #[pyo3(text_signature = "($self)")]
    /// Close the connection now (rather than `__del__()` is called).
    // The connection will be unusable from this point forward; an exception
    // will be raised if any operation is attempted with the connection. The
    // same applies to all cursor objects trying to use the connection. Note
    // that closing a connection without committing the changes first will
    // cause an implicit rollback to be performed.
    fn close(&mut self) -> PyResult<()> {
        todo!()
    }

    #[pyo3(text_signature = "($self)")]
    /// Commits the current transaction.
    /// If this method is not called, the results of any query
    /// executed since the last call to `commit()` will not visible
    /// from other connection.
    fn commit(&mut self) -> PyResult<()> {
        todo!()
    }

    #[pyo3(text_signature = "($self)")]
    /// Rolls back any changes to the database since the last
    /// call to `commit()`.
    fn rollback(&mut self) -> PyResult<()> {
        todo!()
    }

    #[pyo3(text_signature = "($self)")]
    /// Create a new `Cursor` object using the connection.
    fn cursor(&mut self) -> PyResult<Cursor> {
        todo!()
    }

    #[pyo3(text_signature = "($self)")]
    /// Abort any queries that might be executing on the connection.
    /// The query will then abort and the caller will get an exception.
    fn interrupt(&mut self) -> PyResult<()> {
        todo!()
    }

    #[pyo3(text_signature = "($self)")]
    /// Prepare and execute a database operation (query or command).
    /// Parameters may be provided as sequence or mapping and will
    /// be bound to variables in the operation.
    fn execute(&mut self, sql: &str, parameters: Option<Vec<&PyAny>>) -> PyResult<()> {
        todo!()
    }

    #[pyo3(text_signature = "($self)")]
    /// Prepare a database operation (query or command) and then
    /// execute it against all parameter sets supplied by the
    /// `parameters` argument.
    fn executemany(&mut self, sql: &str, parameters: Option<Vec<Vec<&PyAny>>>) -> PyResult<()> {
        todo!()
    }

    #[pyo3(text_signature = "($self)")]
    /// Create a user-defined function that can be used from
    /// within SQL statements under the function name `name`.
    /// The `num_params` argument is the number of parameters
    /// the function accepts (if num_params is -1, the function
    /// may take any number of arguments), and `func` is a
    /// Python callable that is called as the SQL function.
    fn create_function(&mut self, name: &str, num_params: u8, func: &PyAny) -> PyResult<()> {
        todo!()
    }

    #[pyo3(text_signature = "($self)")]
    /// Create a user-defined aggregate function.
    /// The specified aggregate class must implement a `step`
    /// method, which accepts the number of parameters `num_params`
    /// (if num_params is -1, the function may take any number of
    /// arguments), and a `finalize` method which will return the
    /// final result of the aggregate.
    /// The `finalize` method can return any of the types supported
    /// by DataFlex: bytes, str, int, float, date, and None.
    fn create_aggregate(
        &mut self,
        name: &str,
        num_params: u8,
        aggregate_class: &PyAny,
    ) -> PyResult<()> {
        // Ok(())
        todo!()
    }
}

// </editor-fold desc="// Connection ...">

// <editor-fold desc="// Cursor Description ...">

#[derive(Clone, Debug)]
#[pyclass(dict, module = "ferroflex.dbapi")]
/// A standard DB-API v2 Cursor description.
pub struct CursorDescription {
    #[pyo3(get)]
    /// The column's human-readable name
    pub name: String,
    #[pyo3(get)]
    /// The column's data type
    pub type_code: usize,
    #[pyo3(get)]
    /// The "display" size of the column
    /// e.g. a DataFlex
    pub display_size: Option<usize>,
    #[pyo3(get)]
    /// The column's actual on-disk size
    pub internal_size: Option<usize>,
    #[pyo3(get)]
    /// The absolute maximum number of digits
    /// that can be stored in the column (if
    /// the column's datatype is numeric)
    pub precision: Option<usize>,
    #[pyo3(get)]
    /// The absolute maximum number of digits
    /// that can be present to the right of the
    /// decimal in the column's value (if
    /// the column's datatype is numeric)
    pub scale: Option<usize>,
    #[pyo3(get)]
    /// Indicates that the column's value
    /// can be `NULL`
    pub null_ok: Option<bool>,
}

unsafe impl Send for CursorDescription {}

impl CursorDescription {
    fn _get_item(&self, value: AttrIndexOrSlice) -> PyResult<ValueOrSlice<PyObject>> {
        Python::with_gil(|py| match value {
            AttrIndexOrSlice::Attr(name) => Ok(ValueOrSlice::Value(match name {
                "name" => self.name.to_object(py),
                "type_code" => self.type_code.to_object(py),
                "display_size" => self.display_size.to_object(py),
                "internal_size" => self.internal_size.to_object(py),
                "precision" => self.precision.to_object(py),
                "scale" => self.scale.to_object(py),
                "null_ok" => self.null_ok.to_object(py),
                _ => {
                    return Err(PyAttributeError::new_err(""));
                }
            })),
            AttrIndexOrSlice::Index(idx) => Ok(ValueOrSlice::Value(match idx {
                0 | -7 => self.name.to_object(py),
                1 | -6 => self.type_code.to_object(py),
                2 | -5 => self.display_size.to_object(py),
                3 | -4 => self.internal_size.to_object(py),
                4 | -3 => self.precision.to_object(py),
                5 | -2 => self.scale.to_object(py),
                6 | -1 => self.null_ok.to_object(py),
                _ => {
                    return Err(PyIndexError::new_err(""));
                }
            })),
            AttrIndexOrSlice::Slice(slc) => {
                let indexes: PySliceIndices = slc.indices(3)?;

                let (start, end) = (indexes.start, indexes.stop);

                let end: isize = iif!(end > -1, end, 6isize + end);
                let start: isize = iif!(start > -1, start, 6isize + start);

                if start < 0 || end < 0 {
                    return Err(PyIndexError::new_err(""));
                }

                let mut items: Vec<PyObject> = Vec::new();

                for idx in start..end {
                    match self._get_item(AttrIndexOrSlice::Index(idx)).unwrap() {
                        ValueOrSlice::Value(value) => items.push(value),
                        ValueOrSlice::Slice(values) => {
                            values.iter().for_each(|value| items.push(value.clone()))
                        }
                    };
                }

                Ok(ValueOrSlice::Slice(items))
            }
        })
    }
}

#[pymethods]
impl CursorDescription {
    /// Get a `CursorDescription` field by numerical index
    fn __getitem__(
        slf: PyRefMut<Self>,
        value: AttrIndexOrSlice,
    ) -> PyResult<ValueOrSlice<PyObject>> {
        slf._get_item(value)
    }
}

// </editor-fold desc="// Cursor Description ...">

// <editor-fold desc="// Cursor ...">

#[derive(Clone, Debug)]
#[pyclass(dict, module = "ferroflex.dbapi")]
/// A standard DB-API v2 Cursor object.
pub struct Cursor {
    #[pyo3(get)]
    /// Specifies the number of rows that the last .execute*()
    /// produced (for DQL statements like SELECT) or affected
    /// (for DML statements like UPDATE or INSERT).
    pub rowcount: Option<isize>,
    #[pyo3(get)]
    /// Provides the `rowid` of the last modified row (if the
    /// most recent operation did in fact modify a row).
    pub lastrowid: Option<isize>,
    #[pyo3(get, set)]
    /// Specifies the number of rows to fetch at a time with
    /// `.fetchmany()`. Defaults to 1 (i.e. fetch a single
    /// row at a time).
    pub arraysize: Option<usize>,
    #[pyo3(get)]
    /// A sequence of `CursorDescription` sequences.
    /// Each of these sequences contains information describing
    /// exactly one result column. The first two items of each
    /// description (`name` and `type_code`) will always be
    /// populated. The other five are optional and are set to
    /// `None` if no meaningful values can be provided. This
    /// attribute will be None for operations that do not
    /// return rows or if the cursor has not had an operation
    /// invoked via the .execute*() method yet.
    pub description: Option<Vec<CursorDescription>>,
    // #[pyo3(get)]
    // /// A reference to the Connection object on which the
    // /// cursor was created.
    // pub connection: &'conn Connection,
}

unsafe impl Send for Cursor {}

#[allow(unused_variables)]
#[pymethods]
impl Cursor {
    // DB API Required Methods
    // .callproc( procname [, parameters ] )
    // .close()
    // .execute(operation [, parameters])
    // .executemany( operation, seq_of_parameters )
    // .fetchone()
    // .fetchmany([size=cursor.arraysize])
    // .fetchall()
    // .nextset()
    // .setinputsizes(sizes)
    // .setoutputsize(size [, column])

    // OptionalDB API Extensions
    // Cursor.rownumber -> int
    // Cursor.connection -> Connection
    // Cursor.scroll(value [, mode='relative|absolute']) -> None (?)
    // Cursor.messages -> List[Tuple[Type[Exception], Exception]]
    // Connection.messages -> List[Tuple[Type[Exception], Exception]]
    // Cursor.next() -> ResultRow
    // Cursor.__iter__() -> Iterable[ResultRow]
    // Cursor.lastrowid -> Optional[int]

    // <editor-fold desc="// Magic methods ...">

    //     def __init__(self) -> None:
    //         pass
    //
    //     def __iter__(self) -> Any:
    //         return NotImplemented

    // #[args(args = "*")]
    // #[pyo3(text_signature = "($self, *args: Optional[Any]) -> None")]
    // /// Call the `Cursor` class directly
    // fn __call__(&self, args: &PyTuple) -> PyResult<i32> {
    //     println!("Cursor class has been called");
    //     Ok(0)
    // }

    // </editor-fold desc="// Magic methods ...">

    // <editor-fold desc="// Required Properties ...">
    //
    // @property
    // def description(self) -> Sequence[Any]:
    //     """This read-only attribute is a sequence of 7-item sequences.
    //
    //     Each of these sequences contains information describing one result column:
    //
    //     - name
    //     - type_code
    //     - display_size
    //     - internal_size
    //     - precision
    //     - scale
    //     - null_ok
    //
    //     The first two items (name and type_code) are mandatory, the other five are optional
    //     and are set to None if no meaningful values can be provided. This attribute will
    //     be None for operations that do not return rows or if the cursor has not had an
    //     operation invoked via the .execute() method yet.
    //     """
    //     return NotImplemented
    //
    // @property
    // def rowcount(self) -> int:
    //     """This read-only attribute specifies the number of rows that the last
    //     .execute() produced (for DQL statements like SELECT) or affected (for
    //     DML statements like UPDATE or INSERT).
    //
    //     The attribute is -1 in case no .execute() has been performed on
    //     the cursor or the rowcount of the last operation is cannot be
    //     determined by the interface.
    //     """
    //     return NotImplemented

    // # </editor-fold desc="// Required Properties ...">

    // # <editor-fold desc="// 'Optional' properties ...">

    // # </editor-fold desc="// 'Optional' properties ...">

    // <editor-fold desc="// Required methods ...">

    // def close(self) -> Any:
    //     """Close the cursor now (rather than whenever __del__ is called).
    //
    //     The cursor will be unusable from this point forward; an
    //     exception will be raised if any operation is attempted with the
    //     cursor.
    //     """
    //     pass
    //
    // def execute(self, query: str, parameters: Sequence[Any]) -> Any:
    //     """Prepare and execute a database operation (query or command).
    //
    //     Parameters may be provided as sequence or mapping and will be bound to variables in the
    //     operation. Variables are specified in a database-specific notation (see the module's paramstyle
    //     attribute for details).
    //
    //     A reference to the operation will be retained by the cursor. If the same operation object
    //     is passed in again, then the cursor can optimize its behavior. This is most effective for
    //     algorithms where the same operation is used, but different parameters are bound to it (many times).
    //
    //     For maximum efficiency when reusing an operation, it is best to use the .setinputsizes()
    //     method to specify the parameter types and sizes ahead of time. It is legal for a parameter
    //     to not match the predefined information; the implementation should compensate, possibly
    //     with a loss of efficiency.
    //
    //     The parameters may also be specified as list of tuples to e.g. insert multiple rows in a
    //     single operation, but this kind of usage is deprecated: .executemany() should be used instead.
    //
    //     Return values are not defined.
    //     """
    //     pass
    //
    // def executemany(self, query: str, parameters: Sequence[Sequence[Any]]) -> Any:
    //     """Prepare a database operation (query or command) and then execute it
    //     against all parameter sequences or mappings found in the sequence
    //     seq_of_parameters.
    //
    //     Modules are free to implement this method using multiple calls to the .execute() method or by
    //     using array operations to have the database process the sequence as a whole in one call.
    //
    //     Use of this method for an operation which produces one or more result sets constitutes undefined
    //     behavior, and the implementation is permitted (but not required) to raise an exception when it
    //     detects that a result set has been created by an invocation of the operation.
    //
    //     The same comments as for .execute() also apply accordingly to this method.
    //
    //     Return values are not defined.
    //     """
    //     pass
    //
    // def fetchone(self) -> Any:
    //     """Fetch the next row of a query result set, returning a single
    //     sequence, or None when no more data is available.
    //
    //     An exception is raised if the previous call to .execute() did
    //     not produce any result set or no call was issued yet.
    //     """
    //     pass
    //
    // def fetchmany(self, size: Optional[int] = None) -> Any:
    //     """Fetch the next set of rows of a query result, returning a sequence
    //     of sequences (e.g. a list of tuples). An empty sequence is returned
    //     when no more rows are available.
    //
    //     The number of rows to fetch per call is specified by the parameter. If it is not given, the cursor's
    //     arraysize determines the number of rows to be fetched. The method should try to fetch as many rows
    //     as indicated by the size parameter. If this is not possible due to the specified number of rows not
    //     being available, fewer rows may be returned.
    //
    //     An Error (or subclass) exception is raised if the previous call to .execute*() did not produce any
    //     result set or no call was issued yet.
    //
    //     Note there are performance considerations involved with the size parameter. For optimal performance,
    //     it is usually best to use the .arraysize attribute. If the size parameter is used, then it is best
    //     for it to retain the same value from one .fetchmany() call to the next.
    //     """
    //     pass
    //
    // def fetchall(self) -> Any:
    //     """Fetch all (remaining) rows of a query result, returning them as a
    //     sequence of sequences (e.g. a list of tuples).
    //
    //     Note that the cursor's arraysize attribute can affect the
    //     performance of this operation. An exception is raised if the
    //     previous call to .execute*() did not produce any result set or
    //     no call was issued yet.
    //     """
    //     pass
    //
    // def nextset(self) -> Any:
    //     """This method will make the cursor skip to the next available set,
    //     discarding any remaining rows from the current set.
    //
    //     If there are no more sets, the method returns None. Otherwise, it returns a true value and
    //     subsequent calls to the .fetch*() methods will return rows from the next result set.
    //
    //     An Error (or subclass) exception is raised if the previous call to .execute*() did not
    //     produce any result set or no call was issued yet.
    //     """
    //     pass
    //
    // @property
    // def arraysize(self) -> int:
    //     """This read/write attribute specifies the number of rows to fetch at a
    //     time with .fetchmany().
    //
    //     It defaults to 1 meaning to fetch a single row at a time.
    //     Implementations must observe this value with respect to the
    //     .fetchmany() method, but are free to interact with the database
    //     a single row at a time. It may also be used in the
    //     implementation of .executemany().
    //     """
    //     return NotImplemented
    //
    // @arraysize.setter
    // def arraysize(self, value: int) -> None:
    //     """"""
    //     raise NotImplementedError
    //
    // def setinputsizes(self, sizes: int) -> Any:
    //     """This can be used before a call to .execute*() to predefine memory
    //     areas for the operation's parameters.
    //
    //     `sizes` is specified as a sequence — one item for each input parameter. The item should be a Type Object
    //     that corresponds to the input that will be used, or it should be an integer specifying the maximum length
    //     of a string parameter. If the item is None, then no predefined memory area will be reserved for that column
    //     (this is useful to avoid predefined areas for large inputs).
    //
    //     This method would be used before the .execute() method is invoked.
    //
    //     Implementations are free to have this method do nothing and users are free to not use it.
    //     """
    //     pass
    //
    // def setoutputsize(self, size: int, column: Optional[Any] = None) -> Any:
    //     """Set a column buffer size for fetches of large columns (e.g. LONGs,
    //     BLOBs, etc.).
    //
    //     The column is specified as an index into the result sequence. Not specifying the column will set the
    //     default size for all large columns in the cursor.
    //
    //     This method would be used before the .execute*() method is invoked.
    //
    //     Implementations are free to have this method do nothing and users are free to not use it.
    //     """
    //     pass
    //
    // </editor-fold desc="// Required methods ...">

    // # <editor-fold desc="// 'Optional' methods ...">
    //
    // def callproc(self, procname: str, *args: Any, **kwargs: Any) -> Any:
    //     """Call a stored database procedure with the given name.
    //
    //     The sequence of parameters must contain one entry for each
    //     argument that the procedure expects. The result of the call is
    //     returned as modified copy of the input sequence. Input
    //     parameters are left untouched, output and input/output
    //     parameters replaced with possibly new values. The procedure may
    //     also provide a result set as output. This must then be made
    //     available through the standard .fetch*() methods.
    //     """
    //     pass
    //
    // def rownumber(self) -> int:
    //     """This read-only attribute should provide the current 0-based index of
    //     the cursor in the result set or None if the index cannot be determined.
    //
    //     The index can be seen as index of the cursor in a sequence (the result set). The next fetch
    //     operation will fetch the row indexed by .rownumber in that sequence.
    //
    //     Warning Message: "DB-API extension cursor.rownumber used".
    //     """
    //     pass
    //
    // def connection(self) -> ConnectionType:
    //     """This read-only attribute return a reference to the Connection object
    //     on which the cursor was created.
    //
    //     The attribute simplifies writing polymorph code in multi-connection environments.
    //
    //     Warning Message: "DB-API extension cursor.connection used".
    //     """
    //     pass
    //
    // def scroll(self, distance: int, mode: str = "relative") -> Any:
    //     """Scroll the cursor in the result set to a new position according to
    //     mode.
    //
    //     If mode is relative (default), value is taken as offset to the current position in the result set, if set
    //     to absolute, value states an absolute target position.
    //
    //     An IndexError should be raised in case a scroll operation would leave the result set. In this case, the
    //     cursor position is left undefined (ideal would be to not move the cursor at all).
    //
    //     Note: This method should use native scrollable cursors, if available, or revert to an emulation for
    //     forward-only scrollable cursors. The method may raise NotSupportedError to signal that a specific operation
    //     is not supported by the database (e.g. backward scrolling).
    //
    //     Warning Message: "DB-API extension cursor.scroll() used".
    //     """
    //     pass
    //
    // def messages(self) -> Any:
    //     """This is a Python list object to which the interface appends tuples
    //     (exception class, exception value) for all messages which the
    //     interfaces receives from the underlying database for this cursor.
    //
    //     The list is cleared by all standard cursor methods calls (prior to executing the call) except for
    //     the .fetch() calls automatically to avoid excessive memory usage and can also be cleared by executing
    //     del cursor.messages[:].
    //
    //     All error and warning messages generated by the database are placed into this list, so checking the
    //     list allows the user to verify correct operation of the method calls.
    //
    //     The aim of this attribute is to eliminate the need for a Warning exception which often causes problems
    //     (some warnings really only have informational character).
    //
    //     Warning Message: "DB-API extension cursor.messages used".
    //     """
    //     pass
    //
    // def next(self) -> Any:
    //     """Return the next row from the currently executing SQL statement using
    //     the same semantics as .fetchone().
    //
    //     A StopIteration exception is raised when the result set is exhausted for Python versions 2.2
    //     and later. Previous versions don't have the StopIteration exception and so the method should
    //     raise an IndexError instead.
    //
    //     Warning Message: "DB-API extension cursor.next() used".
    //     """
    //     pass
    //
    // def lastrowid(self) -> Any:
    //     """This read-only attribute provides the rowid of the last modified
    //     row.
    //
    //     (most databases return a rowid only when a single INSERT operation is performed). If the operation
    //     does not set a rowid or if the database does not support row ids, this attribute should be set to None.
    //
    //     The semantics of .lastrowid are undefined in case the last executed statement modified more than one row,
    //     e.g. when using INSERT with .executemany().
    //
    //     Warning Message: "DB-API extension cursor.lastrowid used".
    //     """
    //     pass
    //
    // </editor-fold desc="// 'Optional' methods ...">
}

// </editor-fold desc="// Cursor ...">

// <editor-fold desc="// Type Objects ...">

// Required Type Objects
// STRING type
// BINARY type
// NUMBER type
// DATETIME type
// ROWID type
// SQL NULL values are represented by the Python None singleton on input and output.

// </editor-fold desc="// Type Objects ...">

// </editor-fold desc="// Objects ...">
