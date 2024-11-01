use itertools::Itertools;
use numpy::ndarray::{Array, ArrayBase, ArrayView1, IxDyn};

// ArrayViewD, ArrayViewMutD, CowRepr, Array1, ArrayD
// use numpy::{IntoPyArray, PyArrayDyn, PyArrayMethods, PyReadonlyArrayDyn};
// use pyo3::prelude::*;
// use pyo3::{pymodule, types::PyModule, Bound, PyResult, Python};

pub fn get_array_lengths<'a, T>(a: ArrayView1<'a, T>, b: ArrayView1<'a, T>) -> (usize, usize) {
    (a.len(), b.len())
}

// crosstab
pub fn crosstab<'a, T>(a: ArrayView1<'a, T>, b: ArrayView1<'a, T>) -> Array<f64, IxDyn> {
    // ArrayD<T> {
    let crosstab_size: (usize, usize) = get_array_lengths(a, b);
    let crosstab_shape = IxDyn(&[crosstab_size.0, crosstab_size.1]);

    let mut xtab = Array::<f64, _>::zeros(crosstab_shape);
    for (i, a) in a.iter().enumerate() {
        for (j, b) in b.iter().enumerate() {
            xtab[[i, j]] += 1.0;
            println!(
                "Index i : {}, Index j : {}, Value: {:?}",
                i,
                j,
                xtab[[i, j]]
            );
        }
    }

    xtab
    // let mut c = Array::zeros(IxDyn(&[crosstab_size.0, crosstab_size.1]));
    // c
    // for (i, a) in a.iter().enumerate() {
    //     for (j, b) in b.iter().enumerate() {
    //         c[[i, j]] = a * b;
    //     }
    // }
    // c
}
