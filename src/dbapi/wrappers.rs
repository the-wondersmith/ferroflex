// NewType wrappers for converting between GlueSQL & PyO3 structs

// Standard Library Imports
use std::collections::HashMap;
use std::ops::Deref;

// Third-Party Imports
use gluesql::core::data::{Row, Value};
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::type_object::PyTypeObject;
use pyo3::types::{PyDate, PyFloat, PyInt, PyList, PyString, PyTuple, PyType};
use pyo3_chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use serde::{Deserialize, Serialize};

// <editor-fold desc="// FerricValue ...">

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
/// A single value held by a column in a given row of a DataFlex table.
pub struct FerricValue(pub Value);

impl Deref for FerricValue {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

unsafe impl Send for FerricValue {}

impl From<Value> for FerricValue {
    fn from(value: Value) -> Self {
        Self(value)
    }
}

impl FromPyObject<'obj> for FerricValue {
    fn extract(obj: &'obj PyAny) -> PyResult<Self> {
        let value_type: &PyType = match obj.downcast::<PyType>() {
            Ok(value) => value,
            Err(_) => obj.get_type(),
        };

        let py: Python = obj.py();

        if value_type.eq(PyInt::type_object(py)) {
            let value: i64 = obj.extract()?;
            return Ok(FerricValue(Value::I64(value)));
        }

        if value_type.eq(PyDate::type_object(py)) {
            let value: NaiveDate = obj.extract()?;
            return Ok(FerricValue(Value::Date(value.0)));
        }

        if value_type.eq(PyFloat::type_object(py)) {
            let value: f64 = obj.extract()?;
            return Ok(FerricValue(Value::F64(value)));
        }

        if value_type.eq(PyString::type_object(py)) {
            let value: String = obj.extract()?;
            return Ok(FerricValue(Value::Str(value)));
        }

        let type_name = value_type.name()?;

        match type_name.to_uppercase().as_str() {
            "NONE" | "NONETYPE" => Ok(FerricValue(Value::Null)),
            _ => Err(PyTypeError::new_err(format!(
                "Unsupported value type: <{type_name}>!"
            ))),
        }
    }
}

impl ToPyObject for FerricValue {
    fn to_object(&self, py: Python) -> PyObject {
        match &self.0 {
            // DataFlex-Supported Variants
            Value::Null => py.None(),
            Value::Bool(value) => value.to_object(py),
            Value::I64(value) => value.to_object(py),
            Value::F64(value) => value.to_object(py),
            Value::Str(value) => value.to_object(py),
            Value::Date(value) => NaiveDate::from(*value).to_object(py),
            // Unused-But-Supportable Variants
            Value::I8(value) => value.to_object(py),
            Value::Map(value) => value
                .into_iter()
                .map(|(key, value)| (key.clone(), FerricValue(value.clone())))
                .collect::<HashMap<String, FerricValue>>()
                .to_object(py),
            Value::List(values) => FerricRow::from(values).to_object(py),
            Value::Time(value) => NaiveTime::from(*value).to_object(py),
            Value::Timestamp(value) => NaiveDateTime::from(*value).to_object(py),
            // Unsupported Variants
            #[allow(unused_variables)]
            Value::Decimal(value) => {
                // PyDecimal (?)
                PyTypeError::new_err("Unsupported data type: <rust_decimal::Decimal>").to_object(py)
            }
            #[allow(unused_variables)]
            Value::Uuid(value) => {
                // u128 requires uuid = "0.8"
                PyTypeError::new_err("Unsupported data type: <gluesql::core::data::Interval>")
                    .to_object(py)
            }
            #[allow(unused_variables)]
            Value::Interval(value) => {
                // PyTimeDelta (?)
                PyTypeError::new_err("Unsupported data type: <gluesql::core::data::Interval>")
                    .to_object(py)
            }
        }
    }
}

impl IntoPy<PyObject> for FerricValue {
    fn into_py(self, py: Python) -> PyObject {
        match self.0 {
            // DataFlex-Supported Variants
            Value::Null => py.None(),
            Value::Bool(value) => value.into_py(py),
            Value::I64(value) => value.into_py(py),
            Value::F64(value) => value.into_py(py),
            Value::Str(value) => value.into_py(py),
            Value::Date(value) => NaiveDate::from(value).into_py(py),
            // Unsupported Variants
            Value::I8(value) => value.into_py(py),
            Value::Map(value) => value
                .into_iter()
                .map(|(key, value)| (key, FerricValue(value)))
                .collect::<HashMap<String, FerricValue>>()
                .into_py(py),
            Value::List(values) => FerricRow::from(values).into_py(py),
            Value::Time(value) => NaiveTime::from(value).into_py(py),
            Value::Timestamp(value) => NaiveDateTime::from(value).into_py(py),
            #[allow(unused_variables)]
            Value::Uuid(value) => {
                // u128 requires uuid = ">=0.8"
                PyTypeError::new_err("Unsupported data type: <gluesql::core::data::Interval>")
                    .into_py(py)
            }
            #[allow(unused_variables)]
            Value::Decimal(value) => {
                // PyDecimal (?)
                PyTypeError::new_err("Unsupported data type: <rust_decimal::Decimal>").into_py(py)
            }
            #[allow(unused_variables)]
            Value::Interval(value) => {
                // PyTimeDelta (?)
                PyTypeError::new_err("Unsupported data type: <gluesql::core::data::Interval>")
                    .into_py(py)
            }
        }
    }
}

// </editor-fold desc="// FerricValue ...">

// <editor-fold desc="// FerricRow ...">

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// A single row of a given DataFlex table.
pub struct FerricRow {
    inner: Row,
}

impl Deref for FerricRow {
    type Target = Row;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

unsafe impl Send for FerricRow {}

impl From<Vec<Value>> for FerricRow {
    fn from(values: Vec<Value>) -> Self {
        Self { inner: Row(values) }
    }
}

impl From<&Vec<Value>> for FerricRow {
    fn from(values: &Vec<Value>) -> Self {
        Self {
            inner: Row(values
                .into_iter()
                .map(|value| value.clone())
                .collect::<Vec<Value>>()),
        }
    }
}

impl From<Row> for FerricRow {
    fn from(values: Row) -> Self {
        Self { inner: values }
    }
}

impl FromPyObject<'obj> for FerricRow {
    fn extract(obj: &'obj PyAny) -> PyResult<Self> {
        let value_type: &PyType = match obj.downcast::<PyType>() {
            Ok(value) => value,
            Err(_) => obj.get_type(),
        };

        let py: Python = obj.py();

        if !(value_type.eq(PyTuple::type_object(py)) || value_type.eq(PyList::type_object(py))) {
            return Err(PyTypeError::new_err(""));
        }

        let values: Vec<PyObject> = obj.extract()?;

        Ok(FerricRow {
            inner: Row(values
                .into_iter()
                .map(|value| value.extract::<FerricValue>(py).unwrap().0)
                .collect()),
        })
    }
}

impl ToPyObject for FerricRow {
    fn to_object(&self, py: Python) -> PyObject {
        PyTuple::new(
            py,
            (*self.inner.0)
                .into_iter()
                .map(|value| FerricValue(value.clone()).to_object(py)),
        )
        .to_object(py)
    }
}

impl IntoPy<PyObject> for FerricRow {
    fn into_py(self, py: Python) -> PyObject {
        PyTuple::new(
            py,
            self.inner
                .0
                .into_iter()
                .map(|value| FerricValue(value).to_object(py)),
        )
        .into_py(py)
    }
}

// </editor-fold desc="// FerricRow ...">
