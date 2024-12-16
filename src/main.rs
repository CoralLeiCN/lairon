// import the ndarray crate
use numpy::ndarray;
// import the crosstab function from lib.rs
pub mod descriptive;
pub mod utils;
// use crate::descriptive::pivot::crosstab;

use ndarray::{Array1, Array2, Axis};
use std::collections::HashSet;

// // Example usage:
// #[test]
// fn test_crosstab() {
//     let arr1 = Array1::from(vec![0, 2, 1, 2, 3, 4, 2, 2]);
//     let arr2 = Array1::from(vec![0, 1, 2, 3, 5, 5, 2, 2]);
//     let result = crosstab(&arr1, &arr2);
//     println!("Crosstab:\n{:?}", result);
// }

fn main() {
    // call argsort
    let arr1 = ndarray::arr1(&[0, 2, 1, 2, 3, 4, 2, 2]);
    let arr2 = ndarray::arr1(&[0, 1, 2, 3, 5, 5, 2, 2]);
    // call crosstab
    // let xtab_result = crosstab(&arr1, &arr2);
    // println!("{:?}", xtab_result);
}
