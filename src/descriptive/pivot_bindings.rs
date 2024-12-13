use crate::descriptive::pivot::crosstab;
use numpy::{IntoPyArray, PyArray2, PyReadonlyArray1};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::{types::PyModule, Bound, PyResult};
// official example
//https://github.com/PyO3/pyo3
/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn crosstab_bindings<'py>(
    py: Python<'py>,
    a: PyReadonlyArray1<'py, i32>,
    b: PyReadonlyArray1<'py, i32>,
) -> Bound<'py, PyArray2<i32>> {
    // fn crosstab_bindings(a: PyArrayDyn<i32>, b: PyArrayDyn<i32>) -> PyArrayDyn<i32> {
    let a = a.as_array();
    let b = b.as_array();
    let result: ndarray::ArrayBase<ndarray::OwnedRepr<i32>, ndarray::Dim<[usize; 2]>> =
        crosstab(a, b);
    result.into_pyarray(py)
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
