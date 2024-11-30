// import the ndarray crate
use numpy::ndarray;
// import the crosstab function from lib.rs
pub mod descriptive;
use std::collections::HashSet;

use crate::descriptive::pivot::crosstab;
// use crate::descriptive::pivot::find_unique_values;
use crate::descriptive::pivot::get_array_lengths;
// use crate::utils::util::argsort;
use numpy::ndarray::{Array, Array1, ArrayBase, ArrayView1, IxDyn};
pub fn argsort<T: PartialOrd>(arr: &Array1<T>) -> Vec<usize> {
    let mut indices: Vec<usize> = (0..arr.len()).collect();
    indices.sort_by(|&a, &b| arr[a].partial_cmp(&arr[b]).unwrap());
    indices
}
fn main() {
    // init dummy array Array<A, Ix1>
    let a = ndarray::arr1(&[1.0, 2.0, 3.0]);
    let b = ndarray::arr1(&[4.0, 5.0, 6.0]);
    let c = Array::range(0., 10., 1.);

    // call argsort
    let perm: Vec<usize> = argsort(&a);

    // call crosstab
    // let c: (usize, usize) = get_array_lengths(a.view(), b.view());
    // let xtab_result = crosstab(a.view(), b.view());
    // println!("{:?}", xtab_result);
}
