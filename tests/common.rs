//! Some common macros for tests

#[macro_export]
macro_rules! py_assert {
    ($py:expr, $($val:ident)+, $assertion:literal) => {
        pyo3::py_run!($py, $($val)+, concat!("assert ", $assertion))
    };
    ($py:expr, *$dict:expr, $assertion:literal) => {
        pyo3::py_run!($py, *$dict, concat!("assert ", $assertion))
    };
}
