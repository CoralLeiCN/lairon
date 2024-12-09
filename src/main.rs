// import the ndarray crate
use numpy::ndarray;
// import the crosstab function from lib.rs
pub mod descriptive;
pub mod utils;
use crate::descriptive::pivot::crosstab;

fn main() {
    // call argsort
    let arr1 = ndarray::arr1(&[0, 2, 1, 2, 3, 4, 2, 2]);
    let arr2 = ndarray::arr1(&[0, 1, 2, 3, 5, 5, 2, 2]);
    // let arr2 = Array::from_vec(vec![0, 1, 2, 3, 4, 5])
    //     .into_shape_with_order(ndarray::IxDyn(&[6]))
    //     .unwrap();
    let result = crosstab(arr1, arr2);
    println!("{:?}", result);
    // call crosstab
    // let c: (usize, usize) = get_array_lengths(a.view(), b.view());
    // let xtab_result = crosstab(a.view(), b.view());
    // println!("{:?}", xtab_result);
}
