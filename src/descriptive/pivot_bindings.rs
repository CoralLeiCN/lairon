use crate::utils::array_value_counts;
use crate::utils::inverse_index;
use itertools::Itertools;
use numpy::ndarray::Array;
use numpy::ndarray::Array1;
use numpy::{IntoPyArray, PyArrayDyn, PyArrayMethods, PyReadonlyArrayDyn};
use pyo3::prelude::*;
use pyo3::{pymodule, types::PyModule, Bound, PyResult, Python};
// import the ndarray crate
use numpy::ndarray;
// import the crosstab function from lib.rs

use crate::descriptive::pivot::crosstab;
use std::collections::HashMap;

use numpy::ndarray::{Array1, ArrayBase, ArrayD, ArrayView1, ArrayViewD, ArrayViewMutD, IxDyn};
use numpy::{IntoPyArray, PyArrayDyn, PyArrayMethods, PyReadonlyArrayDyn};
use pyo3::prelude::*;
use pyo3::{pymodule, types::PyModule, Bound, PyResult, Python};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn crosstab_bindings(a: Array1<int32>, b: Array1<int32>) -> PyResult<Integer> {
    Ok(crosstab(arr1, arr2))
}

/// A Python module implemented in Rust.
#[name = ("stats")]
fn lairon<'py>(_py: Python<'py>, m: &Bound<'py, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(crosstab_bindings, m)?)?;

    // wrapper of `crosstab`
    #[pyfn(m)]
    #[pyo3(name = "crosstab")]
    fn crosstab_py<'py>(
        py: Python<'py>,
        a: PyReadonlyArrayDyn<'py, f64>,
        b: PyReadonlyArrayDyn<'py, f64>,
    ) -> Bound<'py, PyArrayDyn<f64>> {
        let a = a.as_array();
        let b = b.as_array();
        let c = crosstab_bindings(a, b);
        c.into_pyarray_bound(py)
    }
    Ok(())
}
