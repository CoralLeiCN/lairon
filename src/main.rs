// import the ndarray crate
use numpy::ndarray;
// import the crosstab function from lib.rs
pub mod descriptive;
pub mod utils;
use crate::utils::argsort;
use crate::utils::array_value_counts;
use crate::utils::inverse_index;
use numpy::ndarray::Array;
use numpy::ndarray::Array1;
use std::collections::HashMap;

fn main() {
    // call argsort
    let arr1 = ndarray::arr1(&[0, 2, 1, 2, 3, 4, 2, 2]);
    let arr2 = ndarray::arr1(&[0, 1, 2, 3, 5, 5, 2, 2]);
    // let arr2 = Array::from_vec(vec![0, 1, 2, 3, 4, 5])
    //     .into_shape_with_order(ndarray::IxDyn(&[6]))
    //     .unwrap();
    let inv_idx1 = inverse_index(&arr1);
    let inv_idx2 = inverse_index(&arr2);
    println!("{:?}", inv_idx1);
    println!("{:?}", inv_idx2);

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

    for ((i, a), (j, b)) in inv_idx1.iter().enumerate().zip(inv_idx2.iter().enumerate()) {
        zero_crosstab[(*a as usize, *b as usize)] += 1;
        println!("Index i : {}, Index j : {}", a, b)
    }
    println!("{:?}", zero_crosstab);

    // call crosstab
    // let c: (usize, usize) = get_array_lengths(a.view(), b.view());
    // let xtab_result = crosstab(a.view(), b.view());
    // println!("{:?}", xtab_result);
}
