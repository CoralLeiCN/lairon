pub mod descriptive;
pub mod utils;
use descriptive::pivot_bindings::crosstab_bindings;
use descriptive::pivot_bindings::sum_as_string;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::{types::PyModule, Bound, PyResult};
// official example
//https://github.com/PyO3/pyo3
// use crate::descriptive::pivot::crosstab;
/// Formats the sum of two numbers as string.
// import the ndarray crate

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn lairon(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(crosstab_bindings, m)?)?;
    Ok(())
}
