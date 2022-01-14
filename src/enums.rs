// A structured representations of various semi-static values used in the header of a DataFlex table file

// Standard Library Imports
use std::fmt;

// Third-Party Imports
use pyo3;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

// <editor-fold desc="// IndexType ...">

#[derive(Debug, Copy, Clone, Eq, Ord, PartialOrd, PartialEq, Serialize, Deserialize)]
pub enum IndexType {
    Batch,
    Online,
    Unknown,
}

impl Default for IndexType {
    fn default() -> Self {
        Self::Unknown
    }
}

impl fmt::Display for IndexType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                IndexType::Batch => "BATCH",
                IndexType::Online => "ONLINE",
                IndexType::Unknown => "UNKNOWN",
            }
        )
    }
}

impl FromPyObject<'source> for IndexType {
    fn extract(obj: &'source PyAny) -> PyResult<Self> {
        let value: &str = obj.extract()?;

        match value.to_uppercase().as_str() {
            "BATCH" => Ok(IndexType::Batch),
            "ONLINE" => Ok(IndexType::Online),
            "UNKNOWN" => Ok(IndexType::Unknown),
            _ => Err(PyValueError::new_err("")),
        }
    }
}

impl IntoPy<PyObject> for IndexType {
    fn into_py(self, py: Python) -> PyObject {
        IntoPy::into_py(self.to_string(), py)
    }
}

impl From<bool> for IndexType {
    fn from(value: bool) -> Self {
        match value {
            true => IndexType::Batch,
            false => IndexType::Online,
        }
    }
}

// </editor-fold desc="// IndexType ...">

// <editor-fold desc="// IndexCollation ...">

#[derive(Debug, Copy, Clone, Eq, Ord, PartialOrd, PartialEq, Serialize, Deserialize)]
pub enum IndexCollation {
    Default,
    Ascending,
    Uppercase,
    Unknown,
}

impl Default for IndexCollation {
    fn default() -> Self {
        Self::Unknown
    }
}

impl fmt::Display for IndexCollation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                IndexCollation::Default => "DEFAULT",
                IndexCollation::Ascending => "ASCENDING",
                IndexCollation::Uppercase => "UPPERCASE",
                IndexCollation::Unknown => "UNKNOWN",
            }
        )
    }
}

impl<T> From<T> for IndexCollation
where
    T: AsRef<str>,
{
    fn from(value: T) -> Self {
        let value: &str = value.as_ref();

        match value.to_uppercase().as_str() {
            "DEFAULT" => IndexCollation::Default,
            "ASCENDING" => IndexCollation::Ascending,
            "UPPERCASE" => IndexCollation::Uppercase,
            _ => IndexCollation::Unknown,
        }
    }
}

impl FromPyObject<'source> for IndexCollation {
    fn extract(obj: &'source PyAny) -> PyResult<Self> {
        let value: &str = obj.extract()?;

        match value.to_uppercase().as_str() {
            "0" | "DEFAULT" => Ok(IndexCollation::Default),
            "1" | "ASCENDING" => Ok(IndexCollation::Ascending),
            "2" | "UPPERCASE" => Ok(IndexCollation::Uppercase),
            "UNKNOWN" => Ok(IndexCollation::Unknown),
            _ => Err(PyValueError::new_err("")),
        }
    }
}

impl IntoPy<PyObject> for IndexCollation {
    fn into_py(self, py: Python) -> PyObject {
        IntoPy::into_py(self.to_string(), py)
    }
}

// </editor-fold desc="// IndexCollation ...">

// <editor-fold desc="// DataType ...">

#[derive(Debug, Copy, Clone, Eq, Ord, PartialOrd, PartialEq, Serialize, Deserialize)]
pub enum DataType {
    Int, // Technically actually 'Numeric'
    Date,
    Text,
    Ascii,
    Float, // Technically actually 'Numeric'
    Binary,
    Unknown,
    // Overlap,  // Currently unimplemented
}

impl Default for DataType {
    fn default() -> Self {
        Self::Unknown
    }
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DataType::Date => "DATE",
                DataType::Text => "TEXT",
                DataType::Ascii => "ASCII",
                DataType::Int => "NUMERIC",
                DataType::Binary => "BINARY",
                DataType::Float => "NUMERIC",
                DataType::Unknown => "UNKNOWN",
            }
        )
    }
}

impl FromPyObject<'source> for DataType {
    fn extract(obj: &'source PyAny) -> PyResult<Self> {
        let value: &str = obj.extract()?;

        match value.to_uppercase().as_str() {
            "DATE" => Ok(DataType::Date),
            "UNKNOWN" => Ok(DataType::Unknown),
            "INT" | "INTEGER" => Ok(DataType::Int),
            "ASCII" | "CHAR" => Ok(DataType::Ascii),
            "TEXT" | "VARCHAR" => Ok(DataType::Text),
            "BYTES" | "BINARY" => Ok(DataType::Binary),
            "FLOAT" | "NUMERIC" | "DECIMAL" => Ok(DataType::Float),
            _ => Err(PyValueError::new_err("")),
        }
    }
}

impl IntoPy<PyObject> for DataType {
    fn into_py(self, py: Python) -> PyObject {
        IntoPy::into_py(self.to_string(), py)
    }
}

// </editor-fold desc="// DataType ...">

// <editor-fold desc="// CompressionType ...">

#[derive(Debug, Copy, Clone, Eq, Ord, PartialOrd, PartialEq, Serialize, Deserialize)]
pub enum CompressionType {
    None,
    Fast,
    Standard,
    Custom,
    Unknown,
}

impl Default for CompressionType {
    fn default() -> Self {
        Self::Unknown
    }
}

impl fmt::Display for CompressionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CompressionType::None => "NONE",
                CompressionType::Fast => "FAST",
                CompressionType::Custom => "CUSTOM",
                CompressionType::Unknown => "UNKNOWN",
                CompressionType::Standard => "STANDARD",
            }
        )
    }
}

impl FromPyObject<'source> for CompressionType {
    fn extract(obj: &'source PyAny) -> PyResult<Self> {
        let value: &str = obj.extract()?;

        match value.to_uppercase().as_str() {
            "FAST" => Ok(CompressionType::Fast),
            "NONE" => Ok(CompressionType::None),
            "CUSTOM" => Ok(CompressionType::Custom),
            "UNKNOWN" => Ok(CompressionType::Unknown),
            "STANDARD" => Ok(CompressionType::Standard),
            _ => Err(PyValueError::new_err("")),
        }
    }
}

impl IntoPy<PyObject> for CompressionType {
    fn into_py(self, py: Python) -> PyObject {
        IntoPy::into_py(self.to_string(), py)
    }
}

// </editor-fold desc="// CompressionType ...">

// <editor-fold desc="// TransactionType ...">

#[derive(Debug, Copy, Clone, Eq, Ord, PartialOrd, PartialEq, Serialize, Deserialize)]
pub enum TransactionType {
    None,
    Unknown,
    ClientAtomic,
    ServerAtomic,
    ServerLogged,
}

impl Default for TransactionType {
    fn default() -> Self {
        Self::Unknown
    }
}

impl fmt::Display for TransactionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TransactionType::None => "NONE",
                TransactionType::Unknown => "UNKNOWN",
                TransactionType::ClientAtomic => "CLIENT ATOMIC",
                TransactionType::ServerAtomic => "SERVER ATOMIC",
                TransactionType::ServerLogged => "SERVER LOGGED",
            }
        )
    }
}

impl FromPyObject<'source> for TransactionType {
    fn extract(obj: &'source PyAny) -> PyResult<Self> {
        let value: &str = obj.extract()?;

        match value.to_uppercase().as_str() {
            "NONE" => Ok(TransactionType::None),
            "CLIENT ATOMIC" => Ok(TransactionType::ClientAtomic),
            "SERVER ATOMIC" => Ok(TransactionType::ServerAtomic),
            "SERVER LOGGED" => Ok(TransactionType::ServerLogged),
            _ => Err(PyValueError::new_err("")),
        }
    }
}

impl IntoPy<PyObject> for TransactionType {
    fn into_py(self, py: Python) -> PyObject {
        IntoPy::into_py(self.to_string(), py)
    }
}

// </editor-fold desc="// TransactionType ...">

// <editor-fold desc="// LockType ...">

#[derive(Debug, Copy, Clone, Eq, Ord, PartialOrd, PartialEq, Serialize, Deserialize)]
pub enum LockType {
    None,
    File,
    Record,
    Unknown,
}

impl Default for LockType {
    fn default() -> Self {
        Self::Unknown
    }
}

impl fmt::Display for LockType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LockType::None => "NONE",
                LockType::File => "FILE",
                LockType::Record => "RECORD",
                LockType::Unknown => "UNKNOWN",
            }
        )
    }
}

impl FromPyObject<'source> for LockType {
    fn extract(obj: &'source PyAny) -> PyResult<Self> {
        let value: &str = obj.extract()?;

        match value.to_uppercase().as_str() {
            "FILE" => Ok(LockType::File),
            "NONE" => Ok(LockType::None),
            "RECORD" => Ok(LockType::Record),
            _ => Err(PyValueError::new_err("")),
        }
    }
}

impl IntoPy<PyObject> for LockType {
    fn into_py(self, py: Python) -> PyObject {
        IntoPy::into_py(self.to_string(), py)
    }
}
// </editor-fold desc="// LockType ...">

// <editor-fold desc="// Version ...">

#[derive(Debug, Copy, Clone, Eq, Ord, PartialOrd, PartialEq, Serialize, Deserialize)]
pub enum Version {
    Unknown,
    V23B,
    V30,
}

impl Default for Version {
    fn default() -> Self {
        Self::Unknown
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Version::Unknown => "UNKNOWN",
                Version::V23B => "2.3b",
                Version::V30 => "3.0",
            }
        )
    }
}

impl<T> From<T> for Version
where
    T: Into<i64>,
{
    fn from(value: T) -> Self {
        let value: i64 = value.into();
        match value {
            2 | 23 => Version::V23B,
            3 | 30 => Version::V30,
            _ => Version::Unknown,
        }
    }
}

impl FromPyObject<'source> for Version {
    fn extract(obj: &'source PyAny) -> PyResult<Self> {
        let value: &str = obj.extract()?;

        match value.to_uppercase().as_str() {
            "3" | "3.0" | "V30" => Ok(Version::V30),
            "2" | "2.3" | "2.3B" | "V23B" => Ok(Version::V23B),
            "UNKNOWN" => Ok(Version::Unknown),
            _ => Err(PyValueError::new_err("")),
        }
    }
}

impl IntoPy<PyObject> for Version {
    fn into_py(self, py: Python) -> PyObject {
        IntoPy::into_py(self.to_string(), py)
    }
}

// </editor-fold desc="// Version ...">
