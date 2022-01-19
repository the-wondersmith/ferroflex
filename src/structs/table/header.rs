// A structured representation of the header section of a DataFlex table file

// Standard Library Imports
use std::cmp::{max, min};
use std::fmt;

// Third-Party Imports
use byteorder::{ByteOrder, LittleEndian};
use caseless::compatibility_caseless_match_str as cl_eq;
use prettytable::Table as PrettyTable; // Cell, Row as PrintableRow,
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

// Crate-Level Imports
use crate::enums::{CompressionType, Version};
use crate::exceptions::{InternalError, NotSupportedError};
use crate::structs::{Column, Index, TagFile};
use crate::utils::{bytes_from_file, path_from_string, string_from_bytes};

// <editor-fold desc="// Header ...">

#[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Serialize, Deserialize)]
#[pyclass(dict, module = "ferroflex.structs")]
/// A structured representation of the header portion
/// of a DataFlex table file
pub struct Header {
    // Common Attributes
    #[pyo3(get)]
    /// The total number of columns in the table
    pub field_count: u64,
    #[pyo3(get)]
    /// The total number of records currently
    /// present in the table
    pub record_count: u64,
    #[pyo3(get)]
    /// The total length (in bytes) of the data
    /// that makes up one "row" in the table
    pub record_length: u64,
    #[pyo3(get)]
    /// The absolute maximum number of records
    /// that the table should be allowed to store
    pub max_record_count: u64,
    #[pyo3(get)]
    /// The absolute maximum number of records
    /// that the table has *ever* stored
    pub highest_record_count: u64,
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
    pub records_per_block: u64,
    #[pyo3(get)]
    /// The total number of "filler" bytes
    /// that should be expected at the end
    /// of every "block" of records
    pub fill_bytes_per_block: u64,
    #[pyo3(get)]
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
    _first_available_record: Option<u64>,
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

    fn _as_pretty_table(&self) -> PrettyTable {
        todo!()
    }

    fn _ensure_column_sizes(mut self) -> Self {
        let column_count = self.columns.len();

        if column_count == 1 {
            self.columns[0].length = self.record_length;
            return self;
        }

        let offset_pairs: Vec<(u64, u64)> = self
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
                    field_count: header_data[0x59] as u64,
                    record_count: LittleEndian::read_u16(&header_data[0x08..0x0C]) as u64,
                    record_length: LittleEndian::read_u16(&header_data[0x4E..0x50]) as u64,
                    max_record_count: LittleEndian::read_u16(&header_data[0x0C..0x10]) as u64,
                    highest_record_count: LittleEndian::read_u16(&header_data[..0x03]) as u64,
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
                    ) as u64,
                    fill_bytes_per_block: (512
                        % min(512, LittleEndian::read_u16(&header_data[0x4E..0x50])))
                        as u64,
                    version: Version::V23B,
                    ..Header::default()
                })
            }
            3072usize => {
                Ok(Header {
                    // Common Attributes
                    field_count: header_data[0xA5] as u64,
                    record_count: LittleEndian::read_u16(&header_data[0x08..0x0C]) as u64,
                    record_length: LittleEndian::read_u16(&header_data[0x9A..0x9C]) as u64,
                    max_record_count: LittleEndian::read_u16(&header_data[0x0C..0x10]) as u64,
                    highest_record_count: LittleEndian::read_u16(&header_data[..0x03]) as u64,
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
                    records_per_block: LittleEndian::read_u16(&header_data[0x98..0x9A]) as u64,
                    fill_bytes_per_block: (512
                        % min(512u16, LittleEndian::read_u16(&header_data[0x9A..0x9C])))
                        as u64,
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
                        LittleEndian::read_u16(&header_data[0x20..0x24]) as u64
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

// <editor-fold desc="// Tests ...">

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::Header;

    #[test]
    /// Test that the `Header` structure correctly gets a table header
    fn gets_header() {
        todo!()
    }
}

// </editor-fold desc="// Tests ...">
