// A structured representations of various semi-static values used in the header of a DataFlex table file

// Standard Library Imports
use std::fmt;

// Third-Party Imports
use gluesql::core::ast::DataType as SqlDataType;
use gluesql::core::data::SchemaIndexOrd;
use serde::{Deserialize, Serialize};

// <editor-fold desc="// IndexType ...">

#[derive(Clone, Debug, Eq, Ord, PartialOrd, PartialEq, Serialize, Deserialize)]
/// A structured representation of an index's "type"
pub enum IndexType {
    Batch,
    Online,
    Unknown,
}

unsafe impl Send for IndexType {}

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

#[derive(Clone, Debug, Eq, Ord, PartialOrd, PartialEq, Serialize, Deserialize)]
/// A structured representation of an index's collation / sort order
pub enum IndexCollation {
    Default,
    Ascending,
    Uppercase,
    Unknown,
}

unsafe impl Send for IndexCollation {}

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

impl Into<SchemaIndexOrd> for IndexCollation {
    fn into(self) -> SchemaIndexOrd {
        match self {
            IndexCollation::Ascending => SchemaIndexOrd::Asc,
            IndexCollation::Default => SchemaIndexOrd::Desc,
            _ => panic!(),
        }
    }
}

impl<T> From<T> for IndexCollation
where
    T: AsRef<str>,
{
    fn from(value: T) -> Self {
        let value: &str = value.as_ref();

        match value.to_uppercase().as_str() {
            "0" | "DEFAULT" => IndexCollation::Default,
            "1" | "ASCENDING" => IndexCollation::Ascending,
            "2" | "UPPERCASE" => IndexCollation::Uppercase,
            _ => IndexCollation::Unknown,
        }
    }
}

// </editor-fold desc="// IndexCollation ...">

// <editor-fold desc="// DataType ...">

#[derive(Clone, Debug, Eq, Ord, PartialOrd, PartialEq, Serialize, Deserialize)]
/// A structured representation of type of data stored in a given column
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

unsafe impl Send for DataType {}

impl Default for DataType {
    fn default() -> Self {
        Self::Unknown
    }
}

impl Into<SqlDataType> for DataType {
    fn into(self) -> SqlDataType {
        match self {
            DataType::Int => SqlDataType::Int,
            DataType::Date => SqlDataType::Date,
            DataType::Float => SqlDataType::Float,
            DataType::Text | DataType::Ascii => SqlDataType::Text,
            DataType::Binary => SqlDataType::List,
            DataType::Unknown => panic!(),
        }
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

// </editor-fold desc="// DataType ...">

// <editor-fold desc="// CompressionType ...">

#[derive(Clone, Debug, Eq, Ord, PartialOrd, PartialEq, Serialize, Deserialize)]
/// A structured representation of the compression
/// method in use on a given DataFlex table
pub enum CompressionType {
    None,
    Fast,
    Standard,
    Custom,
    Unknown,
}

unsafe impl Send for CompressionType {}

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

// </editor-fold desc="// CompressionType ...">

// <editor-fold desc="// TransactionType ...">

#[derive(Clone, Debug, Eq, Ord, PartialOrd, PartialEq, Serialize, Deserialize)]
/// A structured representation of the transaction
/// type in use on a given DataFlex table
pub enum TransactionType {
    None,
    Unknown,
    ClientAtomic,
    ServerAtomic,
    ServerLogged,
}

unsafe impl Send for TransactionType {}

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

// </editor-fold desc="// TransactionType ...">

// <editor-fold desc="// LockType ...">

#[derive(Clone, Debug, Eq, Ord, PartialOrd, PartialEq, Serialize, Deserialize)]
/// A structured representation of the type of
/// file locking in use on a given DataFlex table
pub enum LockType {
    None,
    File,
    Record,
    Unknown,
}

unsafe impl Send for LockType {}

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

// </editor-fold desc="// LockType ...">

// <editor-fold desc="// Version ...">

#[derive(Clone, Debug, Eq, Ord, PartialOrd, PartialEq, Serialize, Deserialize)]
/// A structured representation of the "version" of
/// a given DataFlex table's header structure
pub enum Version {
    Unknown,
    V23B,
    V30,
}

unsafe impl Send for Version {}

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

// </editor-fold desc="// Version ...">

// <editor-fold desc="// Tests ...">

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::{
        CompressionType, DataType, IndexCollation, IndexType, LockType, TransactionType, Version,
    };

    #[test]
    /// Test that the `CompressionType` enum behaves as expected
    fn describes_compression() {
        todo!()
    }

    #[test]
    /// Test that the `DataType` enum behaves as expected
    fn describes_data_type() {
        todo!()
    }

    #[test]
    /// Test that the `IndexCollation` enum behaves as expected
    fn describes_index_order() {
        todo!()
    }

    #[test]
    /// Test that the `IndexType` enum behaves as expected
    fn describes_index_type() {
        todo!()
    }

    #[test]
    /// Test that the `LockType` enum behaves as expected
    fn describes_lock_type() {
        todo!()
    }

    #[test]
    /// Test that the `TransactionType` enum behaves as expected
    fn describes_transaction_type() {
        todo!()
    }

    #[test]
    /// Test that the `Version` enum behaves as expected
    fn describes_versions() {
        todo!()
    }
}

// </editor-fold desc="// Tests ...">
