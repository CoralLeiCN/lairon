use crate::descriptive::pivot::crosstab;
use numpy::ndarray::Array1;
use numpy::{PyArrayDyn, PyReadonlyArrayDyn};
use pyo3::prelude::*;
use pyo3::{types::PyModule, Bound, PyResult, Python};

// official example
//https://github.com/PyO3/pyo3
/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

fn crosstab_bindings(a: PyReadonlyArrayDyn<i32>, b: PyReadonlyArrayDyn<i32>) -> PyArrayDyn<i32> {
    let a = a.as_array();
    let b = b.as_array();
    let c = crosstab(a, b);

    Ok(c)
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
