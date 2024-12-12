use crate::descriptive::pivot::crosstab;
use ndarray::Array;
use ndarray::ArrayBase;
use pyo3::wrap_pyfunction;

use numpy::{PyArray2, PyArrayDyn, PyReadonlyArray1};
use pyo3::prelude::*;
use pyo3::{types::PyModule, Bound, PyResult};
// official example
//https://github.com/PyO3/pyo3
/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn crosstab_bindings(
    py: Python<'_>,
    a: PyReadonlyArray1<i32>,
    b: PyReadonlyArray1<i32>,
) -> PyResult<Py<PyAny>> {
    // fn crosstab_bindings(a: PyArrayDyn<i32>, b: PyArrayDyn<i32>) -> PyArrayDyn<i32> {
    let a = a.as_array();
    let b: ndarray::ArrayBase<ndarray::ViewRepr<&i32>, ndarray::Dim<[usize; 1]>> = b.as_array();
    let result: ndarray::ArrayBase<ndarray::OwnedRepr<i32>, ndarray::Dim<[usize; 2]>> =
        crosstab(a.to_owned(), b.to_owned());

    Ok(PyArray2::from_owned_array_bound(py, result).into_py(py))
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
#[pyo3(name = "lairon")]
fn lairon(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(crosstab_bindings, m)?)?;
    Ok(())
}
