// INSERT DOCSTRING HERE

// Third-Party Imports
use gluesql::prelude::*;
use pyo3::class::iter::IterNextOutput;
use pyo3::exceptions::{PyAttributeError, PyIndexError, PyStopIteration};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PySliceIndices, PyTuple, PyType};

// Crate-Level Imports
use crate::exceptions::ProgrammingError;
use crate::{iif, AttrIndexSliceOrItem, ValueOrSlice};

// Sub-Module Level Imports
use super::{Connection, FerricRow}; // FerricValue

// <editor-fold desc="// Cursor Description ...">

#[derive(Clone, Debug, Default)]
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

#[pymethods]
impl CursorDescription {
    /// Get a `CursorDescription` field by numerical index,
    /// literal name, or slice.
    fn __getitem__(
        &self,
        py: Python,
        value: AttrIndexSliceOrItem<isize>,
    ) -> PyResult<ValueOrSlice<PyObject>> {
        match value {
            AttrIndexSliceOrItem::Item(idx) | AttrIndexSliceOrItem::Index(idx) => {
                Ok(ValueOrSlice::Value(match idx {
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
                }))
            }
            AttrIndexSliceOrItem::Name(name) => Ok(ValueOrSlice::Value(match name {
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
            AttrIndexSliceOrItem::Slice(slc) => {
                let indexes: PySliceIndices = slc.indices(3)?;

                let (start, end) = (indexes.start, indexes.stop);

                let end: isize = iif!(end > -1, end, 6isize + end);
                let start: isize = iif!(start > -1, start, 6isize + start);

                if start < 0 || end < 0 {
                    return Err(PyIndexError::new_err(""));
                }

                let mut items: Vec<PyObject> = Vec::new();

                for idx in start..end {
                    match self
                        .__getitem__(py, AttrIndexSliceOrItem::Index(idx))
                        .unwrap()
                    {
                        ValueOrSlice::Value(value) => items.push(value),
                        ValueOrSlice::Slice(values) => {
                            values.iter().for_each(|value| items.push(value.clone()))
                        }
                    };
                }

                Ok(ValueOrSlice::Slice(items))
            }
        }
    }
}

// </editor-fold desc="// Cursor Description ...">

// <editor-fold desc="// Cursor ...">

#[derive(Clone, Debug)]
#[pyclass(dict, module = "ferroflex.dbapi")]
/// A standard DB-API v2 Cursor object.
pub struct Cursor {
    //
    // Python-Accessible Attributes
    //
    #[pyo3(get)]
    /// A read-only reference to the `Connection` object on
    /// which the cursor was created.
    pub connection: Py<Connection>,
    #[pyo3(get)]
    /// Specifies the number of rows that the last .execute*()
    /// produced (for DQL statements like SELECT) or affected
    /// (for DML statements like UPDATE or INSERT).
    pub rowcount: Option<isize>,
    #[pyo3(get)]
    /// Specifies the current 0-based index of the cursor in the
    /// result set or `None` if the index cannot be determined.
    ///
    /// The index can be seen as index of the cursor in a sequence
    /// (the result set). The next fetch operation will fetch the
    /// row indexed by `rownumber` in that sequence.
    pub rownumber: Option<usize>,
    #[pyo3(get)]
    /// Provides the `rowid` of the last modified row (if the
    /// most recent operation did in fact modify a row).
    pub lastrowid: Option<usize>,
    #[pyo3(get, set)]
    /// Specifies the number of rows to fetch at a time with
    /// `.fetchmany()`. Defaults to 1 (i.e. fetch a single
    /// row at a time).
    pub arraysize: usize,
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
    #[pyo3(get)]
    /// This is a Python list object to which the interface
    /// appends tuples (exception class, exception value) for
    /// all messages which the interfaces receives from the
    /// underlying database for this cursor.
    ///
    /// The list is cleared by all standard cursor methods calls
    /// (prior to executing the call) except for the .fetch*()
    /// calls automatically to avoid excessive memory usage and
    /// can also be cleared by executing del cursor.messages[:].
    ///
    /// All error and warning messages generated by the database
    /// are placed into this list, so checking the list allows
    /// the user to verify correct operation of the method calls.
    ///
    /// The aim of this attribute is to eliminate the need for a
    /// Warning exception which often causes problems (some warnings
    /// really only have informational character).
    pub messages: Option<Vec<(Py<PyType>, PyObject)>>,
    //
    // Rust-only "Internal" Attributes
    //
    /// The results of the most recent query executed by the cursor
    pub(crate) results: Option<Vec<FerricRow>>,
    /// INSERT DOCSTRING HERE
    pub(crate) input_size: usize,
    /// INSERT DOCSTRING HERE
    pub(crate) output_size: usize,
}

unsafe impl Send for Cursor {}

#[allow(unused_variables)]
#[pymethods]
impl Cursor {
    // <editor-fold desc="// Magic methods ...">

    pub fn __iter__(slf: PyRefMut<Self>) -> PyResult<PyRefMut<Self>> {
        if slf.results.is_none() {
            return Err(PyStopIteration::new_err(""));
        }

        Ok(slf)
    }

    pub fn __next__(mut slf: PyRefMut<Self>) -> IterNextOutput<FerricRow, Option<u8>> {
        let row = match &mut slf.results {
            None => {
                slf.rownumber = None;
                return IterNextOutput::Return(None);
            }
            Some(results) => match results.pop() {
                None => {
                    slf.results = None;
                    slf.rownumber = None;
                    return IterNextOutput::Return(None);
                }
                Some(row) => {
                    slf.rownumber = Some(slf.rownumber.unwrap_or(0) + 1);
                    row
                }
            },
        };

        IterNextOutput::Yield(row)
    }

    // </editor-fold desc="// Magic methods ...">

    // <editor-fold desc="// Properties ...">

    // </editor-fold desc="// Properties ...">

    // <editor-fold desc="// Required methods ...">

    #[pyo3(text_signature = "(self) -> None")]
    /// Close the cursor now (rather than `__del__()` is called).
    ///
    /// The cursor will be unusable from this point forward; an
    /// exception will be raised if any operation is attempted
    /// with the cursor.
    pub fn close(slf: PyRefMut<Self>) -> PyResult<()> {
        let conn = Py::borrow_mut(&slf.connection, PyRefMut::py(&slf));
        Connection::close(conn)
    }

    #[pyo3(text_signature = "(self) -> None")]
    /// Commits the current transaction.
    /// If this method is not called, the results of any query
    /// executed since the last call to `commit()` will not visible
    /// from other connections.
    pub fn commit(slf: PyRefMut<Self>) -> PyResult<()> {
        let conn = Py::borrow_mut(&slf.connection, PyRefMut::py(&slf));
        Connection::commit(conn)
    }

    #[pyo3(text_signature = "(self) -> None")]
    /// Rolls back any changes to the database since the last
    /// call to `commit()`.
    pub fn rollback(slf: PyRefMut<Self>) -> PyResult<()> {
        let conn = Py::borrow_mut(&slf.connection, PyRefMut::py(&slf));
        Connection::rollback(conn)
    }

    #[pyo3(text_signature = "(self, \
        sql: str, \
        parameters: Optional[Union[Sequence[Any], Mapping[str, Any]]] \
        ) -> Optional[Sequence[CursorDescription]]")]
    /// Execute the supplied SQL statement with any supplied values
    /// bound to the statement using placeholders.
    ///
    /// Method will only execute a single SQL statement. Attempting
    /// to execute more than one statement with it, it will raise a
    /// Warning. Use `executescript` if you want to execute multiple
    /// SQL statements with one call.
    pub fn execute(
        mut slf: PyRefMut<Self>,
        py: Python,
        sql: &str,
        parameters: Option<Vec<&PyAny>>,
    ) -> PyResult<String> {
        // Prepare and execute a database operation (query or command).
        //
        // Parameters may be provided as sequence or mapping and will be
        // bound to variables in the operation. Variables are specified in
        // a database-specific notation (see the module's paramstyle attribute
        // for details).
        //
        // A reference to the operation will be retained by the cursor. If
        // the same operation object is passed in again, then the cursor can
        // optimize its behavior. This is most effective for algorithms where
        // the same operation is used, but different parameters are bound to
        // it (many times).
        //
        // For maximum efficiency when reusing an operation, it is best to use
        // the .setinputsizes() method to specify the parameter types and sizes
        // ahead of time. It is legal for a parameter to not match the predefined
        // information; the implementation should compensate, possibly with a loss
        // of efficiency.
        //
        // The parameters may also be specified as list of tuples to e.g. insert
        // multiple rows in a single operation, but this kind of usage is deprecated:
        // .executemany() should be used instead.

        let conn = slf.connection.clone_ref(py);
        let sql_engine = &mut Py::borrow_mut(&conn, py).sql_engine;

        let result: Payload = sql_engine.execute(sql).unwrap();

        let ret_val: String = format!("{:?}", &result);

        match result {
            Payload::Select { labels, rows } => {
                slf.results = Some(
                    rows.into_iter()
                        .map(FerricRow::from)
                        .collect::<Vec<FerricRow>>(),
                );
                slf.rowcount = Some(0);
            }
            // TODO: Implement `results` population
            //       for remaining payload types
            // Payload::Insert(rows) => {}
            // Payload::Delete(rows) => {}
            // Payload::Update(rows) => {}
            // Payload::Create => {}
            // Payload::DropTable => {}
            // Payload::AlterTable => {}
            // Payload::CreateIndex => {}
            // Payload::DropIndex => {}
            // Payload::StartTransaction => {}
            // Payload::Commit => {}
            // Payload::Rollback => {}
            // Payload::ShowVariable(var) => {}
            _ => (),
        };

        Ok(ret_val)
    }

    #[pyo3(text_signature = "(self, \
        sql: str, \
        parameters: Optional[Union[Sequence[Any], Mapping[str, Any]]] \
        ) -> Optional[Sequence[Sequence[CursorDescription]]]")]
    /// Execute a parameterized SQL query against all parameter
    /// sequences or mappings found in the `parameters` argument.
    pub fn executemany(
        slf: PyRefMut<Self>,
        py: Python,
        sql: &str,
        parameters: Option<Vec<Vec<&PyAny>>>,
    ) -> PyResult<()> {
        // Prepare a database operation (query or command) and then execute it
        // against all parameter sequences or mappings found in the sequence
        // seq_of_parameters.
        //
        // Modules are free to implement this method using multiple calls to the
        // .execute() method or by using array operations to have the database
        // process the sequence as a whole in one call.
        //
        // Use of this method for an operation which produces one or more result
        // sets constitutes undefined behavior, and the implementation is permitted
        // (but not required) to raise an exception when it detects that a result
        // set has been created by an invocation of the operation.
        //
        // The same comments as for .execute() also apply accordingly to this method.

        todo!()
    }

    #[pyo3(text_signature = "(self, sql_script: str) -> None")]
    /// A nonstandard convenience method for executing multiple SQL
    /// statements at once. It issues a COMMIT statement first, then
    /// executes the SQL script it gets as a parameter. This method
    /// disregards isolation_level; any transaction control must be
    /// added to sql_script.
    pub fn executescript(slf: PyRefMut<Self>, py: Python, sql_script: &str) -> PyResult<()> {
        todo!()
    }

    #[pyo3(text_signature = "(self) -> Sequence[Optional[Any]]")]
    /// Fetch the next row of a query result set, returning a single
    /// sequence, or `None` when no more data is available.
    ///
    /// An exception is raised if the previous call to `execute` did
    /// not produce any result set or no call was issued yet.
    pub fn fetchone(slf: PyRefMut<Self>) -> PyResult<Option<FerricRow>> {
        let row = match Cursor::__next__(slf) {
            IterNextOutput::Yield(value) => Some(value),
            IterNextOutput::Return(_) => None,
        };

        Ok(row)
    }

    #[pyo3(text_signature = "(self, size: Optional[int] = None) -> Sequence[Optional[Any]]")]
    /// INSERT DOCSTRING HERE
    pub fn fetchmany(
        mut slf: PyRefMut<Self>,
        size: Option<usize>,
    ) -> PyResult<Option<Vec<FerricRow>>> {
        // Fetch the next set of rows of a query result, returning a sequence
        // of sequences (e.g. a list of tuples). An empty sequence is returned
        // when no more rows are available.
        //
        // The number of rows to fetch per call is specified by the parameter.
        // If it is not given, the cursor's arraysize determines the number of
        // rows to be fetched. The method should try to fetch as many rows as
        // indicated by the size parameter. If this is not possible due to the
        // specified number of rows not being available, fewer rows may be returned.
        //
        // An Error (or subclass) exception is raised if the previous call to
        // `execute*` did not produce any result set or no call was issued yet.
        //
        // NOTE: There are performance considerations involved with the size parameter.
        // For optimal performance, it is usually best to use the .arraysize attribute.
        // If the size parameter is used, then it is best for it to retain the same value
        // from one `fetchmany` call to the next.
        Ok(match &mut slf.results {
            None => {
                return Err(ProgrammingError::new_err("No "));
            }
            Some(results) => Some(results.drain(0..size.unwrap_or(results.len())).collect()),
        })
    }

    #[pyo3(text_signature = "(self) -> Sequence[Sequence[Optional[Any]]]")]
    /// INSERT DOCSTRING HERE
    pub fn fetchall(mut slf: PyRefMut<Self>) -> PyResult<Vec<FerricRow>> {
        // Fetch all (remaining) rows of a query result, returning them as a
        // sequence of sequences (e.g. a list of tuples).
        //
        // Note that the cursor's arraysize attribute can affect the
        // performance of this operation. An exception is raised if the
        // previous call to .execute*() did not produce any result set or
        // no call was issued yet.
        Ok(match &mut slf.results {
            None => {
                return Err(ProgrammingError::new_err("No "));
            }
            Some(results) => results.drain(0..results.len()).collect(),
        })
    }

    #[pyo3(text_signature = "(self) -> None")]
    /// Skip to the next available set in the cursor's `results`
    /// buffer, discarding any remaining rows from the current set.
    ///
    /// If there are no more sets, the method returns None. Otherwise,
    /// it returns a true value and subsequent calls to the .fetch*()
    /// methods will return rows from the next result set.
    ///
    /// An Error (or subclass) exception is raised if the previous call
    /// to .execute*() did not produce any result set or no call was
    /// issued yet.
    pub fn nextset(mut slf: PyRefMut<Self>) -> PyResult<Option<bool>> {
        match &mut slf.results {
            None => Err(ProgrammingError::new_err("")),
            Some(results) => match results.pop() {
                Some(row) => Ok(Some(true)),
                None => Ok(None),
            },
        }
    }

    #[pyo3(text_signature = "(self, size: int) -> None")]
    /// INSERT DOCSTRING HERE
    pub fn setinputsizes(mut slf: PyRefMut<Self>, size: usize) -> PyResult<()> {
        // This can be used before a call to `execute*` to predefine memory
        // areas for the operation's parameters.
        //
        // `sizes` is specified as a sequence — one item for each input parameter.
        // The item should be a Type object that corresponds to the input that will
        // be used, or it should be an integer specifying the maximum length of a
        // string parameter. If the item is None, then no predefined memory area
        // will be reserved for that column (this is useful to avoid predefined areas
        // for large inputs).
        //
        // This method would be used before the `execute` method is invoked.
        //
        // Implementations are free to have this method do nothing and users are
        // free to not use it.
        Ok(slf.input_size = size)
    }

    #[pyo3(text_signature = "(self, size: int, column: Optional[str] = None) -> None")]
    /// INSERT DOCSTRING HERE
    pub fn setoutputsize(slf: PyRefMut<Self>, value: usize) -> PyResult<()> {
        // Set a column buffer size for fetches of large columns (e.g. LONGs,
        // BLOBs, etc.).
        //
        // The column is specified as an index into the result sequence. Not
        // specifying the column will set the default size for all large
        // columns in the cursor.
        //
        // This method would be used before the .execute*() method is invoked.
        //
        // Implementations are free to have this method do nothing and users
        // are free to not use it.
        todo!()
    }

    // </editor-fold desc="// Required methods ...">

    // # <editor-fold desc="// 'Optional' methods ...">

    #[pyo3(text_signature = "(self) -> Sequence[Optional[Any]]")]
    /// Return the next row from the currently executing SQL
    /// statement using the same semantics as `fetchone`.
    ///
    /// A `StopIteration` exception is raised when the result
    /// set is exhausted.
    pub fn next(slf: PyRefMut<Self>) -> PyResult<FerricRow> {
        match Cursor::__next__(slf) {
            IterNextOutput::Yield(value) => Ok(value),
            IterNextOutput::Return(_) => Err(PyStopIteration::new_err("")),
        }
    }

    #[pyo3(text_signature = "(self, \
        procname: str, \
        *args: Any, \
        **kwargs: Any) -> Sequence[Optional[Any]]")]
    /// INSERT DOCSTRING HERE
    pub fn callproc(
        slf: PyRefMut<Self>,
        py: Python,
        procname: &str,
        py_args: &PyTuple,
        py_kwargs: Option<&PyDict>,
    ) -> PyResult<()> {
        // Call a stored database procedure with the given name.
        //
        // The sequence of parameters must contain one entry for each
        // argument that the procedure expects. The result of the call is
        // returned as modified copy of the input sequence. Input
        // parameters are left untouched, output and input/output
        // parameters replaced with possibly new values. The procedure may
        // also provide a result set as output. This must then be made
        // available through the standard .fetch*() methods.

        todo!()
    }

    #[pyo3(text_signature = "(self, \
    distance: int, \
    mode: Literal['relative', 'absolute']) -> None")]
    /// INSERT DOCSTRING HERE
    pub fn scroll(slf: PyRefMut<Self>, distance: isize, mode: &str) -> PyResult<()> {
        // Scroll the cursor in the result set to a new position according to
        // mode.
        //
        // If mode is relative (default), value is taken as offset to the
        // current position in the result set, if set to absolute, value
        // states an absolute target position.
        //
        // An IndexError should be raised in case a scroll operation would
        // leave the result set. In this case, the cursor position is left
        // undefined (ideal would be to not move the cursor at all).
        //
        // Note: This method should use native scrollable cursors, if available,
        // or revert to an emulation for forward-only scrollable cursors. The
        // method may raise NotSupportedError to signal that a specific operation
        // is not supported by the database (e.g. backward scrolling).
        todo!()
    }

    // </editor-fold desc="// 'Optional' methods ...">
}

// </editor-fold desc="// Cursor ...">
