use crate::descriptive::pivot::crosstab;
use crate::descriptive::pivot::margins;
use numpy::{IntoPyArray, PyArray1, PyArray2, PyReadonlyArray1, PyReadonlyArray2};
use pyo3::prelude::*;
use pyo3::{Bound, PyResult};
// official example
//https://github.com/PyO3/pyo3
// use crate::descriptive::pivot::crosstab;
// Formats the sum of two numbers as string.
// import the ndarray crate
#[pyfunction]
pub fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
pub fn crosstab_bindings<'py>(
    py: Python<'py>,
    a: PyReadonlyArray1<'py, i32>,
    b: PyReadonlyArray1<'py, i32>,
) -> Bound<'py, PyArray2<usize>> {
    let a = a.as_array();
    let b = b.as_array();
    let result = crosstab(&a.to_owned(), &b.to_owned());
    result.into_pyarray(py)
}

#[pyfunction]
pub fn margins_bindings<'py>(
    py: Python<'py>,
    arr: PyReadonlyArray2<'py, usize>,
) -> (Bound<'py, PyArray1<usize>>, Bound<'py, PyArray1<usize>>) {
    let arr = arr.as_array();
    let (row_sums, col_sums) = margins(&arr.to_owned());
    (row_sums.into_pyarray(py), col_sums.into_pyarray(py))
}
