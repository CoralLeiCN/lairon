use crate::descriptive::pivot::crosstab;
use numpy::{IntoPyArray, PyArray2, PyReadonlyArray1};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::{types::PyModule, Bound, PyResult};
// official example
//https://github.com/PyO3/pyo3
// use crate::descriptive::pivot::crosstab;
/// Formats the sum of two numbers as string.
// import the ndarray crate
use ndarray::Array;
use numpy::ndarray;

use ndarray::{Array1, Array2, Axis};
use std::collections::HashSet;

fn crosstab1(arr1: &Array1<i32>, arr2: &Array1<i32>) -> Array2<usize> {
    // Get unique values
    let unique1: Vec<i32> = arr1
        .iter()
        .copied()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    let unique2: Vec<i32> = arr2
        .iter()
        .copied()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    // Sort unique values
    let mut unique1 = unique1;
    let mut unique2 = unique2;
    unique1.sort();
    unique2.sort();

    // Create result matrix
    let mut result = Array2::<usize>::zeros((unique1.len(), unique2.len()));

    // Fill the matrix with frequencies
    for (i, &val1) in unique1.iter().enumerate() {
        for (j, &val2) in unique2.iter().enumerate() {
            let count = arr1
                .iter()
                .zip(arr2.iter())
                .filter(|(&a, &b)| a == val1 && b == val2)
                .count();
            result[[i, j]] = count;
        }
    }

    result
}

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn crosstab_bindings<'py>(
    py: Python<'py>,
    a: PyReadonlyArray1<'py, i32>,
    b: PyReadonlyArray1<'py, i32>,
) -> Bound<'py, PyArray2<usize>> {
    // fn crosstab_bindings(a: PyArrayDyn<i32>, b: PyArrayDyn<i32>) -> PyArrayDyn<i32> {
    let a = a.as_array();
    let b = b.as_array();
    let result = crosstab1(&a.to_owned(), &b.to_owned());
    result.into_pyarray(py)
}

// #[pyfunction]
// fn crosstab_bindings<'py>(
//     py: Python<'py>,
//     a: PyReadonlyArray1<'py, i32>,
//     b: PyReadonlyArray1<'py, i32>,
// ) -> Bound<'py, PyArray2<i32>> {
//     // fn crosstab_bindings(a: PyArrayDyn<i32>, b: PyArrayDyn<i32>) -> PyArrayDyn<i32> {
//     let a = a.as_array();
//     let b = b.as_array();
//     let result: ndarray::ArrayBase<ndarray::OwnedRepr<i32>, ndarray::Dim<[usize; 2]>> =
//         crosstab(a, b);
//     result.into_pyarray(py)
// }

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
