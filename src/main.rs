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
    let arr2 = Array::from_vec(vec![0, 1, 2, 3, 4, 5])
        .into_shape_with_order(ndarray::IxDyn(&[6]))
        .unwrap();
    let arr1_value_count: HashMap<&i32, i32> = array_value_counts(&arr1);
    let arr2_value_count: HashMap<&i32, i32> = array_value_counts(&arr2);

    println!("{:?}", arr1_value_count);
    println!("{:?}", arr2_value_count);
    let len_map_arr1: usize = arr1_value_count.len();
    let len_map_arr2: usize = arr2_value_count.len();

    println!(
        "unique value of array1 {:?}, unique value of array2 {:?}",
        len_map_arr1, len_map_arr2
    );

    // init empty crosstab
    let mut zero_crosstab: ndarray::ArrayBase<ndarray::OwnedRepr<i32>, ndarray::Dim<[usize; 2]>> =
        Array::<i32, _>::zeros((len_map_arr1, len_map_arr2));
    println!("{:?}", zero_crosstab);
    // call crosstab
    // let c: (usize, usize) = get_array_lengths(a.view(), b.view());
    // let xtab_result = crosstab(a.view(), b.view());
    // println!("{:?}", xtab_result);
}
