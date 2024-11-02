// import the ndarray crate
use numpy::ndarray;
// import the crosstab function from lib.rs
pub mod descriptive;
use std::collections::HashSet;

use crate::descriptive::pivot::crosstab;
// use crate::descriptive::pivot::find_unique_values;
use crate::descriptive::pivot::get_array_lengths;
use numpy::ndarray::{Array, Array1, ArrayBase, ArrayView1, IxDyn};

fn main() {
    // init dummy array Array<A, Ix1>
    let a = ndarray::arr1(&[1.0, 2.0, 3.0]);
    let b = ndarray::arr1(&[4.0, 5.0, 6.0]);
    let c = Array::range(0., 10., 1.);

    // iterating over a array
    for i in c.iter() {
        println!("{}, ", i);
    }
    // call crosstab
    // let c: (usize, usize) = get_array_lengths(a.view(), b.view());
    // let xtab_result = crosstab(a.view(), b.view());
    // println!("{:?}", xtab_result);
}
