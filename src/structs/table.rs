// A structured representation of a DataFlex table file

// Standard Library Imports
use std::cmp::{max, min};
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use std::iter::IntoIterator;
use std::ops::Index as Indexable;
use std::sync::{Arc, RwLock};
use std::vec::IntoIter;

// Third-Party Imports
use byteorder::{ByteOrder, LittleEndian};
use caseless::compatibility_caseless_match_str as cl_eq;
use chrono::NaiveDate;
use gluesql::core::ast::ColumnDef;
use gluesql::core::data::{Schema, SchemaIndex, Value};
use gluesql::core::sqlparser::ast::DataType;
use prettytable::{Cell, Row as PrintableRow, Table as PrettyTable};
use pyo3::exceptions::{PyIndexError, PyKeyError, PyTypeError, PyValueError};
use pyo3::iter::PyIterProtocol;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PySliceIndices, PyTuple};
use serde::{Deserialize, Serialize};

// Crate-Level Imports
use crate::enums::{CompressionType, Version};
use crate::exceptions::{InternalError, NotSupportedError};
use crate::structs::{Column, Index, TagFile};
use crate::utils::{
    bytes_from_file, date_from_bytes, float_from_bcd_bytes, int_from_bcd_bytes, path_from_string,
    string_from_bytes,
};
use crate::{iif, AttrIndexOrSlice, ValueOrSlice};

// <editor-fold desc="// Helpers ...">

pub(crate) fn sql_value_to_string(value: &Value) -> String {
    match value {
        Value::Date(val) => val.format("%Y-%m-%d").to_string(),
        Value::F64(val) => val.to_string(),
        Value::I64(val) => val.to_string(),
        Value::Str(val) => val.to_string(),
        Value::Null => "NULL".to_string(),
        _ => "UNKNOWN".to_string(),
    }
}

pub(crate) fn sql_value_to_py(val: &Value) -> PyResult<PyObject> {
    Python::with_gil(|py| {
        Ok(match val {
            Value::Str(val) => val.into_py(py),
            Value::I64(val) => val.into_py(py),
            Value::F64(val) => val.into_py(py),
            Value::Date(val) => pyo3_chrono::NaiveDate::from(*val).into_py(py),
            Value::Null => py.None(),
            _ => {
                return Err(PyValueError::new_err(""));
            }
        })
    })
}

pub(crate) fn py_to_sql_value(obj: &PyAny) -> PyResult<Value> {
    // Python::with_gil(|py| {
    //     let obj: &PyAny = obj.as_ref(py);
    // })

    let type_name: &str = obj.get_type().name()?;

    Ok(match type_name.to_lowercase().as_str() {
        "str" => Value::Str(obj.extract::<String>()?),
        "int" => Value::I64(obj.extract::<i64>()?),
        "float" => Value::F64(obj.extract::<f64>()?),
        "date" => Value::Date(NaiveDate::from(obj.extract::<pyo3_chrono::NaiveDate>()?)),
        "none" | "nonetype" => Value::Null,
        _ => {
            return Err(PyTypeError::new_err(""));
        }
    })
}

// </editor-fold desc="// Helpers ...">

// <editor-fold desc="// Value Iterator ...">

#[derive(Clone, Debug)]
#[pyclass(dict, module = "ferroflex.structs")]
/// An intermediate structure used to iterate over the
/// values in a single row from a DataFlex table file
pub struct RowValueIterator {
    pub inner: std::vec::IntoIter<PyObject>,
}

#[pyproto]
impl PyIterProtocol for RowValueIterator {
    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<Self>) -> Option<PyObject> {
        slf.inner.next()
    }
}

// </editor-fold desc="// Value Iterator ...">

// <editor-fold desc="// Row ...">

#[derive(Clone, Debug, Eq, PartialOrd, PartialEq, Serialize, Deserialize)]
#[pyclass(dict, module = "ferroflex.structs")]
/// A structured representation of a single entry
/// in a DataFlex table file
pub struct Row(Vec<Value>);

unsafe impl Send for Row {}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Row<{}>",
            self.0
                .iter()
                .map(|val| match val {
                    Value::Date(v) => format!("'{}'", v),
                    Value::Str(v) => format!("'{}'", v),
                    Value::F64(v) => v.to_string(),
                    Value::I64(v) => v.to_string(),
                    Value::Null => "NULL".to_string(),
                    _ => format!("{:?}", val),
                })
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl IntoIterator for Row {
    type Item = Value;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl ToPyObject for Row {
    fn to_object(&self, py: Python) -> PyObject {
        self.0
            .iter()
            .map(sql_value_to_py)
            .filter(|entry| entry.is_ok())
            .map(|value| value.unwrap())
            .collect::<Vec<PyObject>>()
            .into_py(py)
    }
}

impl Row {
    // <editor-fold desc="// 'Private' Methods ...">

    fn _as_pretty_table(&self) -> PrettyTable {
        // Create a pretty-printable row by stringify-ing
        // the row's individual `Value` objects
        let mut table = PrettyTable::new();

        table.add_row(PrintableRow::new(
            self.0
                .iter()
                .map(|value| Cell::new(sql_value_to_string(value).as_str()))
                .collect::<Vec<Cell>>(),
        ));

        table
    }

    // </editor-fold desc="// 'Private' Methods ...">

    // <editor-fold desc="// 'Public' Methods ...">

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> std::slice::Iter<Value> {
        self.0.iter()
    }

    // </editor-fold desc="// 'Public' Methods ...">
}

#[allow(unused_variables)]
#[pymethods]
impl Row {
    #[new]
    #[args(args = "*", kwargs = "**")]
    fn __new__(py_args: &PyTuple, py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        let mut values: Vec<Value> = py_args
            .iter()
            .map(py_to_sql_value)
            .filter(|value| value.is_ok())
            .map(|value| value.unwrap())
            .collect();

        if let Some(kwargs) = py_kwargs {
            kwargs.values().iter().for_each(|value| {
                if let Ok(v) = py_to_sql_value(value) {
                    values.push(v)
                }
            })
        }

        Ok(Self(values))
    }

    fn __str__(slf: PyRefMut<Self>) -> PyResult<String> {
        Ok(format!("{}", *slf))
    }

    fn __repr__(slf: PyRefMut<Self>) -> PyResult<String> {
        Ok(format!("{}", *slf))
    }

    fn __len__(slf: PyRefMut<Self>) -> usize {
        slf.len()
    }

    fn __getitem__(slf: PyRefMut<Self>, key: AttrIndexOrSlice) -> PyResult<ValueOrSlice<PyObject>> {
        match key {
            AttrIndexOrSlice::Index(idx) => {
                let idx = iif!(idx > -1, idx, slf.0.len() as isize + idx);

                if idx < 0 {
                    return Err(PyIndexError::new_err(""));
                }

                if let Some(col) = slf.0.get(idx as usize) {
                    return Ok(ValueOrSlice::Value(sql_value_to_py(col)?));
                }

                Err(PyIndexError::new_err(""))
            }
            AttrIndexOrSlice::Slice(slc) => {
                let slc: (isize, isize) = slc.extract()?;
                let (start, end) = slc;

                let end: isize = iif!(end > -1, end, slf.0.len() as isize + end);
                let start: isize = iif!(start > -1, start, slf.0.len() as isize + start);

                if start < 0 || end < 0 {
                    return Err(PyIndexError::new_err(""));
                }

                let end: usize = end as usize;
                let start: usize = start as usize;

                Ok(ValueOrSlice::Slice(
                    (&slf.0)[start..end]
                        .iter()
                        .map(|col| sql_value_to_py(col).unwrap())
                        .collect::<Vec<PyObject>>(),
                ))
            }
            AttrIndexOrSlice::Attr(_) => Err(PyKeyError::new_err("")),
        }
    }

    fn __setitem__(_slf: PyRefMut<Self>, _index: isize, _record: PyRef<Row>) -> PyResult<()> {
        todo!()
    }

    fn __delitem__(_slf: PyRefMut<Self>, _index: isize) -> PyResult<()> {
        todo!()
    }

    fn __iter__(slf: PyRef<Self>) -> PyResult<Py<RowValueIterator>> {
        Py::new(
            slf.py(),
            RowValueIterator {
                inner: slf
                    .0
                    .iter()
                    .map(sql_value_to_py)
                    .filter(PyResult::is_ok)
                    .map(PyResult::unwrap)
                    .collect::<Vec<PyObject>>()
                    .into_iter(),
            },
        )
    }

    fn __contains__(_slf: PyRef<Self>, value: PyObject) -> PyResult<bool> {
        todo!()
    }

    fn __reversed__(_slf: PyRef<Self>) -> PyResult<Vec<PyObject>> {
        todo!()
    }

    fn pretty(slf: PyRefMut<Self>) -> String {
        slf._as_pretty_table().to_string()
    }
}

// </editor-fold desc="// Row ...">

// <editor-fold desc="// Header ...">

#[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Serialize, Deserialize)]
#[pyclass(dict, module = "ferroflex.structs")]
/// A structured representation of the header portion
/// of a DataFlex table file
pub struct Header {
    // Common Attributes
    #[pyo3(get)]
    /// The total number of columns in the table
    pub field_count: u8,
    #[pyo3(get)]
    /// The total number of records currently
    /// present in the table
    pub record_count: u32,
    #[pyo3(get)]
    /// The total length (in bytes) of the data
    /// that makes up one "row" in the table
    pub record_length: u32,
    #[pyo3(get)]
    /// The absolute maximum number of records
    /// that the table should be allowed to store
    pub max_record_count: u32,
    #[pyo3(get)]
    /// The absolute maximum number of records
    /// that the table has *ever* stored
    pub highest_record_count: u32,
    #[pyo3(get)]
    /// Indicates that the bytes occupied by
    /// records deleted from the table should
    /// be overwritten with null bytes instead
    /// of being "snipped" from the table
    pub reuse_deleted_space: bool,
    #[pyo3(get)]
    /// Indicates that the table is configured
    /// for simultaneous access by multiple users
    pub multiuser_reread_active: bool,
    // Embedded Structures
    #[pyo3(get)]
    /// The header's "index table"
    pub indexes: Vec<Index>,
    #[pyo3(get)]
    /// The name of the on-disk table file to
    /// which the header belongs
    pub file_root_name: String,
    #[pyo3(get)]
    /// The header's "column" table
    pub columns: Vec<Column>,
    // Computed Attributes
    /// The total number of records present
    /// within a given "block" of bytes
    pub records_per_block: u8,
    #[pyo3(get)]
    /// The total number of "filler" bytes
    /// that should be expected at the end
    /// of every "block" of records
    pub fill_bytes_per_block: u8,
    // #[pyo3(get)]
    /// The version of DataFlex in use when
    /// the table was initially created
    pub version: Version,
    #[pyo3(get)]
    /// The absolute path of the table's
    /// on-disk file
    pub filepath: String,
    // DataFlex 3.0+ Attributes
    // #[pyo3(get)]
    /// Denotes the type of compression used
    /// to shrink the table's on-disk size
    _compression_type: Option<CompressionType>,
    // #[pyo3(get)]
    /// Indicates that the table is currently
    /// locked for reading or writing
    _file_locking1: Option<bool>,
    // #[pyo3(get)]
    /// Indicates that the table is currently
    /// locked for reading or writing
    _file_locking2: Option<bool>,
    // #[pyo3(get)]
    /// (unverified) Denotes the offset at which
    /// the table's first "available" record can
    /// be found
    _first_available_record: Option<u32>,
    // #[pyo3(get)]
    /// Indicates that integrity verification
    /// is currently enabled for the table's
    /// header section
    _header_integrity_enabled: Option<bool>,
    // #[pyo3(get)]
    /// Indicates that new records should
    /// be written to the nulled-over space
    /// previously occupied by any records
    /// that have been deleted from the table
    /// instead of being appended to the "end"
    /// of the table's on-disk data
    _reuse_deleted_records: Option<bool>,
    // Unused / Undocumented Attributes
    // _always_one: (u8, u8),
    // _always_zero: u8,
    // _checksums: [[u8; 4]; 4],
}

unsafe impl Send for Header {}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Header<table_name: {} | df_version: {}>",
            self.file_root_name, self.version
        )
    }
}

impl Header {
    // <editor-fold desc="// 'Private' Methods ...">

    fn _get_header_bytes_from(table_path: &str) -> PyResult<Vec<u8>> {
        // Try to read the first ~3kb of the table (`bytes_from_file` will return
        // as many bytes as it can read if the table is smaller than that)
        let header_bytes = bytes_from_file(table_path, None, Some(3072u64))?;

        // At this point the opened file handle should have been dropped, so all
        // we have to do is check the "version bytes" and return the header data
        // accordingly
        match header_bytes[0x1C..0x1E] {
            [0x1E, 0x1E] => Ok(header_bytes),
            [0x00, 0x00] => Ok(header_bytes[0..min(header_bytes.len(), 512)].to_vec()),
            _ => Err(NotSupportedError::new_err("Unsupported table format!")),
        }
    }

    fn _field_values(&self) -> Vec<(String, String)> {
        vec![
            ("Table Name".to_string(), self.file_root_name.to_string()),
            ("DF Version".to_string(), format!("{}", self.version)),
            ("Field Count".to_string(), format!("{}", self.field_count)),
            ("Record Count".to_string(), format!("{}", self.record_count)),
            (
                "Record Length".to_string(),
                format!("{}", self.record_length),
            ),
            (
                "Max Record Count".to_string(),
                format!("{}", self.max_record_count),
            ),
            (
                "Highest Record Count".to_string(),
                format!("{}", self.highest_record_count),
            ),
            (
                "Reuse Deleted Space".to_string(),
                format!("{}", self.reuse_deleted_space),
            ),
            (
                "Multi-User Reread".to_string(),
                format!("{}", self.multiuser_reread_active),
            ),
            (
                "Records / \"Block\"".to_string(),
                format!("{}", self.records_per_block),
            ),
            (
                "Fill-Bytes / \"Block\"".to_string(),
                format!("{}", self.fill_bytes_per_block),
            ),
            // ("Indexes".to_string(), format!("{}", self.indexes)),
            // ("Columns".to_string(), format!("{}", self.columns)),
        ]
    }

    fn _as_pretty_table(&self) -> PrettyTable {
        // Create a mutable pretty-printing table
        let mut table = PrettyTable::new();

        // Append a new row to the table for each
        // key-value pair of the header's fields
        self._field_values().iter().for_each(|pair| {
            table.add_row(PrintableRow::new(vec![
                Cell::new(&pair.0),
                Cell::new(&pair.1),
            ]));
        });

        if !self.indexes.is_empty() {
            // Append a row to represent the header's index table
            table.add_row(PrintableRow::new(vec![
                Cell::new("Indexes"),
                Cell::new(
                    &(vec![self
                        .indexes
                        .iter()
                        .map(|idx| Cell::new(&format!("{}", idx)))
                        .collect::<PrintableRow>()]
                    .into_iter()
                    .collect::<PrettyTable>())
                    .to_string(),
                ),
            ]));
        }

        // Append a row to represent the header's column table
        table.add_row(PrintableRow::new(vec![
            Cell::new("Columns"),
            Cell::new(
                &(PrettyTable::from_iter(vec![PrintableRow::from_iter(
                    self.columns
                        .iter()
                        .map(|col| Cell::new(&format!("{}", col)))
                        .collect::<Vec<Cell>>(),
                )]))
                .to_string(),
            ),
        ]));

        // Yield the assembled pretty-formatted
        // table as a string
        table
    }

    fn _ensure_column_sizes(mut self) -> Self {
        let column_count = self.columns.len();

        if column_count == 1 {
            self.columns[0].length = self.record_length;
            return self;
        }

        let offset_pairs: Vec<(u32, u32)> = self
            .columns
            .iter()
            .enumerate()
            .map(|pair| {
                if pair.0 == column_count - 1 {
                    (pair.1.offset, self.columns[pair.0 - 1].offset)
                } else {
                    (pair.1.offset, self.columns[pair.0 + 1].offset)
                }
            })
            .collect();

        for (pos, col) in self.columns.iter_mut().enumerate() {
            if pos == column_count - 1 {
                col.length = offset_pairs[pos].0 - offset_pairs[pos].1;
            } else {
                col.length = offset_pairs[pos].1 - offset_pairs[pos].0;
            }
        }

        self
    }

    // </editor-fold desc="// 'Private' Methods ...">

    // <editor-fold desc="// 'Public' Methods ...">

    pub fn from_bytes(header_data: &[u8], column_names: Vec<String>) -> PyResult<Header> {
        // Ensure that we've been given at least as many bytes
        // as we actually need, then build the header accordingly

        let field_count: u8 = match header_data.len() {
            512usize => header_data[0x59],
            3072usize => header_data[0xA5],
            _ => 0u8,
        };

        let column_names: Vec<String> =
            TagFile::generate_column_names(field_count, Some(column_names))?;

        return match header_data.len() {
            512usize => {
                Ok(Header {
                    // Common Attributes
                    field_count: header_data[0x59],
                    record_count: LittleEndian::read_u16(&header_data[0x08..0x0C]) as u32,
                    record_length: LittleEndian::read_u16(&header_data[0x4E..0x50]) as u32,
                    max_record_count: LittleEndian::read_u16(&header_data[0x0C..0x10]) as u32,
                    highest_record_count: LittleEndian::read_u16(&header_data[..0x03]) as u32,
                    reuse_deleted_space: header_data[0x58] == 0,
                    multiuser_reread_active: !matches!(header_data[0x5C], 0),
                    // Embedded Structures
                    indexes: Index::table_from_bytes(header_data[0x64..0xB4].into())?,
                    file_root_name: string_from_bytes(
                        &header_data[0xB4..0xBD].to_vec(),
                        Some(false),
                    )?,
                    columns: Column::table_from_bytes(
                        header_data[0xC4..0x1FD].into(),
                        Some(column_names),
                    )?,
                    // Computed Attributes
                    records_per_block: max(
                        512 / LittleEndian::read_u16(&header_data[0x4E..0x50]),
                        1,
                    ) as u8,
                    fill_bytes_per_block: (512
                        % min(512, LittleEndian::read_u16(&header_data[0x4E..0x50])))
                        as u8,
                    version: Version::V23B,
                    ..Header::default()
                })
            }
            3072usize => {
                Ok(Header {
                    // Common Attributes
                    field_count: header_data[0xA5],
                    record_count: LittleEndian::read_u16(&header_data[0x08..0x0C]) as u32,
                    record_length: LittleEndian::read_u16(&header_data[0x9A..0x9C]) as u32,
                    max_record_count: LittleEndian::read_u16(&header_data[0x0C..0x10]) as u32,
                    highest_record_count: LittleEndian::read_u16(&header_data[..0x03]) as u32,
                    reuse_deleted_space: header_data[0x4A] == 0,
                    multiuser_reread_active: false,
                    // Embedded Structures
                    indexes: Index::table_from_bytes(header_data[0xB0..0x1D0].into())?,
                    file_root_name: string_from_bytes(
                        &header_data[0x2D0..0x2E0].to_vec(),
                        Some(false),
                    )?,
                    columns: Column::table_from_bytes(
                        header_data[0x2E0..0xAD8].into(),
                        Some(column_names),
                    )?,
                    // Computed Attributes
                    records_per_block: LittleEndian::read_u16(&header_data[0x98..0x9A]) as u8,
                    fill_bytes_per_block: (512
                        % min(512u16, LittleEndian::read_u16(&header_data[0x9A..0x9C])))
                        as u8,
                    version: Version::V30,
                    // V3 Attributes
                    _compression_type: match header_data[0x1F] {
                        0 => Some(CompressionType::None),
                        1 => Some(CompressionType::Fast),
                        2 => Some(CompressionType::Standard),
                        _ => None,
                    },
                    _file_locking1: Some(!matches!(header_data[0x41], 0)),
                    _file_locking2: Some(header_data[0xA8] == 1),
                    _first_available_record: Some(
                        LittleEndian::read_u16(&header_data[0x20..0x24]) as u32
                    ),
                    _header_integrity_enabled: Some(
                        header_data[0x10..0x14].iter().sum::<u8>() == 0u8,
                    ),
                    _reuse_deleted_records: Some(header_data[0xA4] == 1),
                    ..Header::default()
                }
                ._ensure_column_sizes())
            }
            _ => Err(InternalError::new_err(format!(
                "Expected either 512 or 3072 bytes but actually got {}",
                header_data.len()
            ))),
        };
    }

    pub fn from_path(filepath: &str) -> PyResult<Header> {
        // 1 - Ensure the provided path is actually a table
        //     - If it's not, return Header::default()
        // 2 - Try to find the table's tag file
        //     - Read the column names if it's found
        // 3 - Call `get_header_bytes` with the validated table path
        // 4 - Form up like Voltron

        let table_path = path_from_string(filepath, Some(true));

        let (column_names, header_data) = if table_path.is_file()
            && table_path.exists()
            && cl_eq(
                table_path
                    .extension()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default(),
                "dat",
            ) {
            (
                match TagFile::find_tags_for_table(&table_path) {
                    Ok(Some(val)) => TagFile::from_filepath(&val)?.tags,
                    _ => Vec::new(),
                },
                Header::_get_header_bytes_from(filepath)?,
            )
        } else {
            (Vec::new(), Vec::new())
        };

        if header_data.is_empty() {
            return Err(InternalError::new_err(format!(
                "Couldn't create a valid `Header` from '{}'",
                filepath
            )));
        }

        let mut header = Header::from_bytes(&header_data, column_names)?;

        header.filepath = String::from(filepath);

        Ok(header)
    }

    // </editor-fold desc="// 'Public' Methods ...">
}

#[pymethods]
impl Header {
    #[new]
    fn __new__(filepath: String) -> PyResult<Self> {
        Self::from_path(filepath.as_ref())
    }

    fn __str__(slf: PyRefMut<Self>) -> PyResult<String> {
        Ok(format!("{}", *slf))
    }

    fn __repr__(slf: PyRefMut<Self>) -> PyResult<String> {
        Ok(format!("{}", *slf))
    }

    fn pretty(slf: PyRefMut<Self>) -> String {
        slf._as_pretty_table().to_string()
    }
}

// </editor-fold desc="// Header ...">

// <editor-fold desc="// Table Iterator ...">

#[derive(Clone, Debug)]
#[pyclass(dict, module = "ferroflex.structs")]
/// An intermediate structure used to iterate
/// over the rows in a DataFlex table file
pub struct TableRowIterator {
    pub inner: std::vec::IntoIter<Row>,
}

#[pyproto]
impl PyIterProtocol for TableRowIterator {
    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<Self>) -> Option<Row> {
        slf.inner.next()
    }
}

// </editor-fold desc="// Table Iterator ...">

// <editor-fold desc="// DataFlexTable ...">

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, PartialOrd, Serialize)]
#[pyclass(dict, module = "ferroflex.structs")]
/// A structured representation of a DataFlex table file
pub struct DataFlexTable {
    #[pyo3(get)]
    /// The table's header data
    pub header: Header,
    // #[pyo3(get)]
    /// The table's rows, as <RowNumber, Row>
    pub rows: Arc<RwLock<HashMap<u32, Row>>>,
}

unsafe impl Send for DataFlexTable {}

impl<T: AsRef<str>> From<T> for DataFlexTable {
    fn from(table_path: T) -> Self {
        DataFlexTable::from_path(table_path.as_ref()).unwrap()
    }
}

impl<T: Into<i64>> Indexable<T> for DataFlexTable {
    type Output = PyResult<Row>;

    #[allow(unused_variables)]
    fn index(&self, index: T) -> &'static Self::Output {
        let index: i64 = index.into();

        todo!()
    }
}

impl IntoIterator for DataFlexTable {
    type Item = Row;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        (*self.rows)
            .read()
            .unwrap()
            .iter()
            .map(|(_, value)| value.clone())
            .collect::<Vec<Row>>()
            .into_iter()
    }
}

impl fmt::Display for DataFlexTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "DataFlexTable<name: {} | record_count: {} | df_version: {}>",
            self.header.file_root_name, self.header.record_count, self.header.version,
        )
    }
}

impl DataFlexTable {
    // <editor-fold desc="// 'Private' Methods ...">

    fn _field_values(&self) -> Vec<(String, String)> {
        vec![
            ("File Path".to_string(), self.header.filepath.to_string()),
            (
                "Header".to_string(),
                self.header._as_pretty_table().to_string(),
            ),
        ]
    }

    fn _as_pretty_table(&self) -> PrettyTable {
        // Create a mutable pretty-printable table
        let mut table = PrettyTable::new();

        // Add the filepath and header rows
        for (key, value) in self._field_values().iter() {
            table.add_row(PrintableRow::new(vec![Cell::new(key), Cell::new(value)]));
        }

        // Add a blank row to the table
        table.add_row(PrintableRow::new(vec![Cell::new("")]));

        // Create another mutable pretty-printable table
        let mut printable = PrettyTable::new();

        // Add the "header" row as one cell per column
        printable.add_row(PrintableRow::new(
            self.header
                .columns
                .iter()
                .map(|col| Cell::new(&col.name))
                .collect::<Vec<Cell>>(),
        ));

        // // Append a new row for each record in the table
        // self._rows.iter().for_each(|row| {
        //     printable.add_row(PrintableRow::new(
        //         row.iter()
        //             .map(|entry| Cell::new(&format!("{:?}", entry)))
        //             .collect::<Vec<Cell>>(),
        //     ));
        // });

        // Append the created table to the "main" table
        table.add_row(PrintableRow::new(vec![Cell::new(&printable.to_string())]));

        table
    }

    fn _column_definitions(&self) -> PyResult<Vec<ColumnDef>> {
        Ok(self
            .header
            .columns
            .iter()
            .map(Column::definition)
            .filter(PyResult::is_ok)
            .map(|def| def.unwrap())
            .collect::<Vec<ColumnDef>>())
    }

    fn _index_definitions(&self) -> PyResult<Vec<SchemaIndex>> {
        // TODO: Actually implement `Index` support
        Ok(Vec::new())
    }

    fn nth_record_bytes<T: Into<i64>>(&self, record_number: T) -> PyResult<Vec<u8>> {
        let record_number: i64 = record_number.into();

        let record_number: i64 = if record_number < 0i64 {
            self.header.record_count as i64 + record_number
        } else {
            record_number
        };

        if record_number < 0i64 || record_number > self.header.record_count as i64 {
            return Err(PyIndexError::new_err(""));
        }

        // TODO: Update this to behave properly for tables that have fill bytes
        if self.header.fill_bytes_per_block > 0 {
            panic!()
        }

        // Calculate the starting offset of the requested
        // record depending on the table version
        let start: u64 = match self.header.version {
            Version::V23B => 512i64 + (self.header.record_length as i64 * record_number),
            Version::V30 => 3072i64 + (self.header.record_length as i64 * record_number),
            Version::Unknown => {
                return Err(NotSupportedError::new_err("Unsupported table format!"));
            }
        } as u64;

        let end: u64 = start + self.header.record_length as u64;

        bytes_from_file(&self.header.filepath, Some(start), Some(end))
    }

    fn record_from_bytes<T: AsRef<[u8]>>(&self, record_data: T) -> PyResult<Row> {
        let record_data: &[u8] = record_data.as_ref();

        Ok(Row(self
            .header
            .columns
            .iter()
            .map(|col| {
                let start = (col.offset - 1) as usize;
                let end = (col.length as usize) + start;
                let data = &record_data[start..end];

                match col.data_type {
                    DataType::Char(_) => Value::Str(string_from_bytes(data, Some(false)).unwrap()),
                    DataType::Int(_) => Value::I64(int_from_bcd_bytes(data, Some(true)).unwrap()),
                    DataType::Float(_) => {
                        Value::F64(float_from_bcd_bytes(data, Some(col.decimal_points)).unwrap())
                    }
                    DataType::Date => match date_from_bytes(data) {
                        Ok(Some(val)) => Value::Date(NaiveDate::from(val)),
                        _ => Value::Null,
                    },
                    // The first two bytes of TEXT and BINARY fields are actually
                    // a u16 integer denoting how much of the field's allotted
                    // length is actually "populated"
                    DataType::Text => Value::Str(string_from_bytes(data, Some(true)).unwrap()),
                    // `gluesql` doesn't currently support Binary / BLOB types
                    _ => Value::Null,
                    // data[2..][..LittleEndian::read_u16(&data[..2]) as usize].to_vec()
                }
            })
            .collect::<Vec<Value>>()))
    }

    // </editor-fold desc="// 'Private' Methods ...">

    // <editor-fold desc="// Public Methods ...">

    pub fn len(&self) -> u32 {
        self.header.record_count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> TableRowIterator {
        TableRowIterator {
            inner: self.clone().into_iter(),
        }
    }

    pub fn schema(&self) -> PyResult<Schema> {
        Ok(Schema {
            table_name: self.header.file_root_name.to_string(),
            column_defs: self._column_definitions()?,
            indexes: self._index_definitions()?,
        })
    }

    pub fn from_path<T: AsRef<str>>(table_path: T) -> PyResult<DataFlexTable> {
        Ok(DataFlexTable {
            header: Header::from_path(table_path.as_ref())?,
            rows: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    pub fn nth_record<T: Into<i64>>(&self, record_number: T) -> PyResult<Row> {
        let record_number: i64 = record_number.into();

        let record_number: i64 = if record_number > -1i64 {
            record_number
        } else {
            self.header.record_count as i64 + record_number
        };

        if record_number < 0i64 {
            return Err(PyIndexError::new_err(""));
        }

        let record_number: u32 = match u32::try_from(record_number) {
            Ok(value) => value,
            Err(_) => return Err(PyIndexError::new_err("")),
        };

        {
            let mut rows = self.rows.try_write().unwrap();

            if rows.get(&record_number).is_none() {
                let data = match self.nth_record_bytes(record_number) {
                    Ok(bytes) => bytes,
                    Err(error) => {
                        return Err(error);
                    }
                };

                let record = match self.record_from_bytes(&data) {
                    Ok(row) => row,
                    Err(error) => {
                        return Err(error);
                    }
                };

                rows.insert(record_number, record);
            }
        }

        match (*self.rows).read().unwrap().get(&record_number) {
            None => Err(PyIndexError::new_err("")),
            Some(row) => Ok((*row).clone()),
        }
    }

    #[allow(unused_variables)]
    pub fn append_record(&self, record: Row) -> PyResult<()> {
        todo!()
    }

    #[allow(unused_variables)]
    pub fn update_record(&self, record_number: i32, record: Row) -> PyResult<()> {
        todo!()
    }

    // </editor-fold desc="// Public Methods ...">
}

#[allow(unused_variables)]
#[pymethods]
impl DataFlexTable {
    // <editor-fold desc="// Magic Methods ...">

    #[new]
    fn __new__(filepath: String) -> PyResult<Self> {
        Self::from_path(AsRef::<str>::as_ref(&filepath))
    }

    fn __str__(slf: PyRefMut<Self>) -> PyResult<String> {
        Ok(format!("{}", *slf))
    }

    fn __repr__(slf: PyRefMut<Self>) -> PyResult<String> {
        Ok(format!("{}", *slf))
    }

    fn __len__(slf: PyRefMut<Self>) -> usize {
        slf.len() as usize
    }

    fn __getitem__(slf: PyRefMut<Self>, key: AttrIndexOrSlice) -> PyResult<ValueOrSlice<Row>> {
        match key {
            AttrIndexOrSlice::Index(idx) => Ok(ValueOrSlice::Value(slf.nth_record(idx as i64)?)),
            AttrIndexOrSlice::Attr(_) => Err(PyKeyError::new_err("")),
            AttrIndexOrSlice::Slice(slc) => {
                let indexes: PySliceIndices = slc.indices(3)?;

                let (start, end) = (indexes.start as i64, indexes.stop as i64);

                let end: i64 = iif!(end > -1, end, slf.len() as i64 + end);
                let start: i64 = iif!(start > -1, start, slf.len() as i64 + start);

                if start < 0 || end < 0 {
                    return Err(PyIndexError::new_err(""));
                }

                Ok(ValueOrSlice::Slice(
                    (start..end)
                        .map(|idx| {
                            slf.nth_record(idx).unwrap_or_else(|_| {
                                panic!(
                                    "Could not read data for row {} from '{}'!",
                                    idx,
                                    (slf.header.file_root_name).as_str()
                                )
                            })
                        })
                        .collect::<Vec<Row>>(),
                ))
            }
        }
    }

    fn __setitem__(slf: PyRefMut<Self>, index: isize, record: PyRef<Row>) -> PyResult<()> {
        todo!()
    }

    fn __delitem__(slf: PyRefMut<Self>, index: isize) -> PyResult<()> {
        todo!()
    }

    fn __iter__(slf: PyRef<Self>) -> PyResult<Py<TableRowIterator>> {
        Py::new(slf.py(), slf.iter())
    }

    fn __contains__(slf: PyRef<Self>, record: PyRef<Row>) -> PyResult<bool> {
        todo!()
    }

    fn __reversed__(slf: PyRef<Self>) -> PyResult<Vec<Row>> {
        todo!()
    }

    // </editor-fold desc="// Magic Methods ...">

    // <editor-fold desc="// Getter/Setter Methods ...">

    #[getter(rows)]
    fn get_rows(slf: PyRefMut<Self>) -> PyResult<Vec<Row>> {
        Ok((*slf.rows)
            .read()
            .unwrap()
            .iter()
            .map(|(_, value)| value.clone())
            .collect::<Vec<Row>>())
    }

    // </editor-fold desc="// Getter/Setter Methods ...">

    // <editor-fold desc="// Instance Methods ...">

    fn pretty(slf: PyRefMut<Self>) -> String {
        slf._as_pretty_table().to_string()
    }

    fn pop(slf: PyRefMut<Self>, index: isize) -> PyResult<()> {
        todo!()
    }

    fn index(slf: PyRef<Self>, record: PyRef<Row>) -> PyResult<usize> {
        todo!()
    }

    fn insert(slf: PyRefMut<Self>, record: PyRef<Row>) -> PyResult<()> {
        todo!()
    }

    fn append(slf: PyRefMut<Self>, record: PyRef<Row>) -> PyResult<()> {
        todo!()
    }

    fn extend(slf: PyRefMut<Self>, records: &PyTuple) -> PyResult<()> {
        todo!()
    }

    fn remove(slf: PyRefMut<Self>, record: PyRef<Row>) -> PyResult<()> {
        todo!()
    }

    // </editor-fold desc="// Instance Methods ...">
}

// </editor-fold desc="// DataFlexTable ...">

// <editor-fold desc="// Tests ...">

#[cfg(test)]
mod tests {
    use crate::structs::table::DataFlexTable;

    #[test]
    /// Test that the `DataFlexTable` structure correctly gets rows
    fn gets_rows() {
        let table = DataFlexTable::from_path(
            "/Users/mark/Projects/personal/ferroflex/test_data/dev_data.dat",
        );

        for (idx, row) in table.iter().enumerate() {
            println!("Row {}:\t{:?}", idx, row);
        }
        assert_eq!(1, 1);
    }
}

// </editor-fold desc="// Tests ...">
