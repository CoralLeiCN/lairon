// import the ndarray crate
use numpy::ndarray;
// import the crosstab function from lib.rs
pub mod descriptive;
pub mod utils;
use crate::utils::argsort;
use crate::utils::array_value_counts;

use numpy::ndarray::Array;
use std::collections::HashMap;

fn main() {
    // init dummy array Array<A, Ix1>
    let a = ndarray::arr1(&[1.0, 2.0, 3.0]);
    let b = ndarray::arr1(&[4.0, 5.0, 6.0]);
    let c = Array::range(0., 10., 1.);

    // call argsort
    let perm: Vec<usize> = argsort(&a);

    let arr1: ndarray::ArrayBase<ndarray::OwnedRepr<i32>, ndarray::Dim<ndarray::IxDynImpl>> =
        Array::from_vec(vec![0, 2, 1, 2, 3, 4])
            .into_shape_with_order(ndarray::IxDyn(&[6]))
            .unwrap();
    let map: HashMap<&i32, i32> = array_value_counts(&arr1);
    // call crosstab
    // let c: (usize, usize) = get_array_lengths(a.view(), b.view());
    // let xtab_result = crosstab(a.view(), b.view());
    // println!("{:?}", xtab_result);
}
