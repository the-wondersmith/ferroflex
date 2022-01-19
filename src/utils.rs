// Utility functions for working with byte-level data from DataFlex table files

// External Crates
extern crate caseless;

// Standard Library Imports
use bstr::ByteSlice;
use std::cmp::min;
use std::fs;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

// Third-Party Imports
use byteorder::{ByteOrder, LittleEndian};
use itertools::zip;
use num::traits::abs;
use pyo3;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3_chrono::chrono::{Duration, NaiveDate};
use pyo3_chrono::NaiveDate as PyDate;

// Crate-Level Imports
use crate::exceptions::{BCDDecodingError, TextFieldDecodingError};

// <editor-fold desc="// Component Registration ...">

/// Register the Rust code to be "exported" to Python
pub(crate) fn register_components(py: Python, ferroflex_module: &PyModule) -> PyResult<()> {
    // Create the `utils` sub-module
    let utils_module = PyModule::new(py, "ferroflex.utils")?;

    // Add the utility functions to the module
    utils_module.add_function(pyo3::wrap_pyfunction!(date_from_bytes, utils_module)?)?;
    utils_module.add_function(pyo3::wrap_pyfunction!(string_from_bytes, utils_module)?)?;
    utils_module.add_function(pyo3::wrap_pyfunction!(bytes_from_file_py, utils_module)?)?;
    utils_module.add_function(pyo3::wrap_pyfunction!(int_from_bcd_bytes, utils_module)?)?;
    utils_module.add_function(pyo3::wrap_pyfunction!(float_from_bcd_bytes, utils_module)?)?;
    utils_module.add_function(pyo3::wrap_pyfunction!(int_from_packed_bcd, utils_module)?)?;
    utils_module.add_function(pyo3::wrap_pyfunction!(int_from_unpacked_bcd, utils_module)?)?;

    // Add the populated sub-module to the top-level `ferroflex` module
    ferroflex_module.add("utils", utils_module)?;

    // Return an OK
    Ok(())
}

// </editor-fold desc="// Component Registration ...">

// <editor-fold desc="// Helpers ...">

pub fn string_from_path(path: &Path, resolve: Option<bool>) -> String {
    let resolve = resolve.unwrap_or(true);

    let str_val = if resolve {
        format!("{:?}", path.canonicalize().unwrap_or_default())
    } else {
        format!("{:?}", path)
    };

    let mut chars = str_val.chars();

    chars.next();
    chars.next_back();

    chars.as_str().to_string()
}

pub fn path_from_string<T: AsRef<str>>(string_path: T, resolve: Option<bool>) -> PathBuf {
    let resolve = resolve.unwrap_or(true);
    let string_path = string_path.as_ref();

    if !resolve {
        PathBuf::from(string_path)
    } else {
        PathBuf::from(string_path)
            .canonicalize()
            .unwrap_or_else(|_| PathBuf::from(string_path))
    }
}

pub fn bytes_from_file<P: AsRef<str>, U: Into<u64>>(
    filepath: P,
    start: Option<U>,
    end: Option<U>,
) -> PyResult<Vec<u8>> {
    // Turn `filepath` into a usable PathBuf
    let filepath: PathBuf = path_from_string(filepath.as_ref(), None);

    // Ensure we have a valid starting offset
    let start = match start {
        Some(value) => value.into(),
        None => 0u64,
    };

    // Grab a read-handle for the specified file
    let mut origin: fs::File = fs::OpenOptions::new().read(true).open(&filepath)?;

    // Setup a mutable buffer for the file's contents
    let mut file_bytes: Vec<u8> = Vec::new();

    // Seek to the specified offset
    origin.seek(SeekFrom::Start(start))?;

    // Read the requested number of bytes into the buffer
    if let Some(val) = end {
        Read::by_ref(&mut origin)
            .take(val.into() - start)
            .read_to_end(&mut file_bytes)?;
    } else {
        Read::by_ref(&mut origin).read_to_end(&mut file_bytes)?;
    }

    // Return the read bytes
    Ok(file_bytes.to_vec())
}

// </editor-fold desc="// Helpers ...">

// <editor-fold desc="// Python Functions ...">

#[pyfunction]
#[pyo3(text_signature = "(data: bytes) -> int")]
/// Get the decimal integer value of a single byte from an unpacked Binary Coded Decimal.
pub fn int_from_unpacked_bcd(data: u8) -> i64 {
    min(9i64, (data & 0xF) as i64) * if data >> 4 == 0 { -1i64 } else { 1i64 }
}

#[pyfunction]
#[pyo3(text_signature = "(data: bytes, idx: int = 0) -> int")]
/// Get the pair of decimal integer values of a single byte from a packed Binary Coded Decimal.
pub fn int_from_packed_bcd(data: u8, idx: Option<u32>) -> i64 {
    let idx: u32 = idx.unwrap_or(0u32);

    i64::checked_add(
        min(9i64, (data & 0xF) as i64) * i64::pow(10, idx),
        min(9i64, (data >> 4) as i64) * i64::pow(10, idx + 1),
    )
    .unwrap_or(0i64)
}

#[pyfunction]
#[pyo3(text_signature = "(data: bytes) -> int")]
/// Get the value of an integer stored as a packed Binary Coded Decimal (i.e. a series of bytes).
pub fn int_from_bcd_bytes(data: &[u8], signed: Option<bool>) -> PyResult<i64> {
    let signed: bool = signed.unwrap_or(true);
    let start: usize = if signed { 1 } else { 0 };

    // Check the left-most nibble to see if the packed
    // bytes represent a positive or negative value
    let sign: i64 = if data.first().unwrap_or(&16u8) >> 4 == 0 && signed {
        -1i64
    } else {
        1i64
    };

    let value: i64 = match data.len() {
        0 => {
            // BAD BUFFER
            "not an integer".parse::<i64>().unwrap()
        }
        1 => {
            // PACKED BINARY CODED DECIMAL
            // two 4-bit integers
            int_from_packed_bcd(
                *data
                    .first()
                    .ok_or_else(|| BCDDecodingError::new_err("bad buffer"))?,
                Option::None,
            )
        }
        2 => {
            // 16-bit integer
            LittleEndian::read_i16(data) as i64
        }
        _ => {
            // PACKED BINARY CODED DECIMAL

            // The construct below is *dense* at first glance, but is really just
            // an odd looking enumerated loop. It just loops through the supplied
            // bytes in the array in reverse order (to account for the fact that
            // DataFlex stores all BCD numbers in little endian format) while
            // counting by two's so that the actual "index" value always aligns
            // on byte boundaries but can be "incremented" inside the loop itself
            // to ensure that the "place" of the decoded nibble is accurately
            // calculated.

            zip((0..).step_by(2), data[start..].iter().rev()).fold(0, |mut sum, (idx, byte)| {
                sum += int_from_packed_bcd(*byte, Some(idx));
                sum
            })
        }
    };

    Ok(value * sign)
}

#[pyfunction]
#[pyo3(text_signature = "(data: bytes, decimals: int = 1) -> float")]
/// Get the value of a floating point number stored as a packed Binary Coded Decimal (i.e. a series of bytes).
pub fn float_from_bcd_bytes(data: &[u8], decimals: Option<u64>) -> PyResult<f64> {
    let decimals: u64 = decimals.unwrap_or(1u64);

    let left = data[0..data.len() - (decimals as usize)].to_vec();
    let right = data[(data.len() - (decimals as usize))..].to_vec();

    let mut str_buffer: String = String::new();

    str_buffer.push_str(&format!(
        "{}.{}",
        int_from_bcd_bytes(&left, Some(true)).unwrap(),
        abs(int_from_bcd_bytes(&right, Some(false)).unwrap()),
    ));

    match str_buffer.parse::<f64>() {
        Ok(value) => Ok(value),
        Err(_) => Err(BCDDecodingError::new_err("unparse-able value")),
    }
}

#[pyfunction]
#[pyo3(text_signature = "(data: bytes) -> Optional[datetime.date]")]
/// Get the value of a date stored as a series of packed Binary Coded Decimals.
pub fn date_from_bytes(data: &[u8]) -> PyResult<Option<PyDate>> {
    // epoch start date - 1642-09-17
    let date_offset: i64 = match int_from_bcd_bytes(data, Some(false)) {
        Ok(offset) => (offset - 700003),
        Err(_) => {
            return Ok(None);
        }
    };

    let epoch_start: NaiveDate = if date_offset < 0 {
        // date so far in the past that
        // it's probably just a null field
        return Ok(None);
    } else {
        NaiveDate::from_ymd(1642, 9, 17)
    };

    // Add the offset to the epoch start date
    Ok(Some(PyDate::from(
        epoch_start + Duration::days(date_offset),
    )))
}

#[pyfunction]
#[pyo3(text_signature = "(data: bytes, text_field: bool = False) -> str")]
/// Get the value of an ASCII or TEXT field from a DataFlex table file.
pub fn string_from_bytes(data: &[u8], text_field: Option<bool>) -> PyResult<String> {
    // Ensure that `text_field` has a usable value
    let text_field: bool = text_field.unwrap_or(false);

    // The first two bytes of TEXT fields are actually
    // a u16 integer denoting the number of the field's
    // allotted bytes that are actually "occupied"
    if text_field && data.len() < 4 {
        return Err(TextFieldDecodingError::new_err("Too few bytes!"));
    }

    let (text_length, data) = if text_field {
        (LittleEndian::read_u16(&data[..2]), &data[2..])
    } else {
        (0u16, data)
    };

    let text: String = data
        .iter()
        .filter(|entry| ((&8u8 < *entry && *entry < &14u8) || (&31u8 < *entry)))
        .map(|val| char::from(*val))
        .collect::<String>()
        .trim()
        .into();

    if text_field && text.len() as u16 != text_length {
        return Err(TextFieldDecodingError::new_err(format!(
            "Expected a {}-character but actually got {} characters!",
            text_length,
            text.len()
        )));
    }

    Ok(text)
}

#[pyfunction]
#[pyo3(name = "bytes_from_file")]
#[pyo3(text_signature = "(filepath: str, start: int = 0, end: Optional[int] = None) -> str")]
/// Get the byte-level contents from the specified file path with
/// respect to the (optional) offsets specified by `start` and `end`.
pub fn bytes_from_file_py(
    filepath: &str,
    start: Option<u64>,
    end: Option<u64>,
) -> PyResult<Py<PyBytes>> {
    match bytes_from_file(filepath, start, end) {
        Err(error) => Err(error),
        Ok(bytearray) => Python::with_gil(|py| {
            let py_obj: Py<PyBytes> = PyBytes::new(py, bytearray.as_bytes()).into();

            Ok(py_obj)
        }),
    }
}

// </editor-fold desc="// Python Functions ...">

// <editor-fold desc="// Tests ...">

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::{
        bytes_from_file, bytes_from_file_py, date_from_bytes, float_from_bcd_bytes,
        int_from_bcd_bytes, int_from_packed_bcd, int_from_unpacked_bcd, path_from_string,
        string_from_bytes, string_from_path,
    };
    use pyo3::PyResult;

    #[test]
    /// Test that the `int_from_packed_bcd` function correctly decodes packed BCDs
    fn gets_packed_bcds() -> PyResult<()> {
        assert_eq!(int_from_packed_bcd(0x00, None), 0i64);
        assert_eq!(int_from_packed_bcd(0x01, None), 1i64);
        assert_eq!(int_from_packed_bcd(0x02, None), 2i64);
        assert_eq!(int_from_packed_bcd(0x03, None), 3i64);
        assert_eq!(int_from_packed_bcd(0x04, None), 4i64);
        assert_eq!(int_from_packed_bcd(0x05, None), 5i64);
        assert_eq!(int_from_packed_bcd(0x06, None), 6i64);
        assert_eq!(int_from_packed_bcd(0x07, None), 7i64);
        assert_eq!(int_from_packed_bcd(0x08, None), 8i64);
        assert_eq!(int_from_packed_bcd(0x09, None), 9i64);
        assert_eq!(int_from_packed_bcd(0x10, None), 10i64);

        assert_eq!(int_from_packed_bcd(0x11, None), 11i64);
        assert_eq!(int_from_packed_bcd(0x22, None), 22i64);
        assert_eq!(int_from_packed_bcd(0x33, None), 33i64);
        assert_eq!(int_from_packed_bcd(0x44, None), 44i64);
        assert_eq!(int_from_packed_bcd(0x55, None), 55i64);
        assert_eq!(int_from_packed_bcd(0x66, None), 66i64);
        assert_eq!(int_from_packed_bcd(0x77, None), 77i64);
        assert_eq!(int_from_packed_bcd(0x88, None), 88i64);
        assert_eq!(int_from_packed_bcd(0x99, None), 99i64);

        Ok(())
    }

    #[test]
    /// Test that the `int_from_unpacked_bcd` function correctly decodes unpacked BCDs
    fn gets_unpacked_bcds() -> PyResult<()> {
        assert_eq!(int_from_unpacked_bcd(0x00), 0i64);
        assert_eq!(int_from_unpacked_bcd(0x01), 1i64);
        assert_eq!(int_from_unpacked_bcd(0x02), 2i64);
        assert_eq!(int_from_unpacked_bcd(0x03), 3i64);
        assert_eq!(int_from_unpacked_bcd(0x04), 4i64);
        assert_eq!(int_from_unpacked_bcd(0x05), 5i64);
        assert_eq!(int_from_unpacked_bcd(0x06), 6i64);
        assert_eq!(int_from_unpacked_bcd(0x07), 7i64);
        assert_eq!(int_from_unpacked_bcd(0x08), 8i64);
        assert_eq!(int_from_unpacked_bcd(0x09), 9i64);
        assert_eq!(int_from_unpacked_bcd(0x10), 0i64);

        Ok(())
    }

    #[test]
    /// Test that the `date_from_bytes` function correctly decodes BCD-encoded dates
    fn gets_dates_from_bytes() -> PyResult<()> {
        // date_from_bytes
        todo!()
    }

    #[test]
    /// Test that the `int_from_bcd_bytes` function
    /// correctly decodes BCD-encoded integers
    fn gets_ints_from_bcd_bytes() -> PyResult<()> {
        assert_eq!(
            int_from_bcd_bytes(&[0x10, 0x00, 0x00, 0x00, 0x00, 0x02, 0x36], Some(true))?,
            236i64
        );
        assert_eq!(
            int_from_bcd_bytes(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x58, 0x23], Some(true))?,
            -5823i64
        );

        Ok(())
    }

    #[test]
    /// Test that the `float_from_bcd_bytes` function
    /// correctly decodes BCD-encoded floating point numbers
    fn gets_floats_from_bcd_bytes() -> PyResult<()> {
        // float_from_bcd_bytes
        todo!()
    }

    #[test]
    /// Test that the `string_from_bytes` function
    /// correctly decodes DataFlex-encoded byte strings
    fn gets_strings_from_bytes() -> PyResult<()> {
        // string_from_bytes
        todo!()
    }

    #[test]
    /// Test that the `string_from_bytes` function
    /// correctly decodes DataFlex-encoded TEXT fields
    fn gets_strings_from_df_text_fields() -> PyResult<()> {
        // string_from_bytes
        todo!()
    }
}

// </editor-fold desc="// Tests ...">
