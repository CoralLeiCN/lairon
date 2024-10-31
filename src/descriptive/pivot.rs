use numpy::ndarray::{Array1, ArrayBase, ArrayD, ArrayView1, ArrayViewD, ArrayViewMutD, IxDyn};
use numpy::{IntoPyArray, PyArrayDyn, PyArrayMethods, PyReadonlyArrayDyn};
use pyo3::prelude::*;
use pyo3::{pymodule, types::PyModule, Bound, PyResult, Python};

pub fn get_array_lengths<T>(a: ArrayView1<T>, b: ArrayView1<T>) -> (usize, usize) {
    (a.len(), b.len())
}
