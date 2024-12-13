use crate::utils::array_value_counts;
use crate::utils::inverse_index;
use ndarray::Array2;
use numpy::ndarray;
use numpy::ndarray::Array;
use numpy::ndarray::ArrayView;
use numpy::ndarray::ArrayView1;

// crosstab
pub fn crosstab<'a>(arr1: ArrayView1<'a, i32>, arr2: ArrayView1<'a, i32>) -> Array2<i32> {
    // ArrayD<T> {
    let inv_idx1 = inverse_index(arr1);
    let inv_idx2 = inverse_index(arr2);
    // println!("{:?}", inv_idx1);
    // println!("{:?}", inv_idx2);

    let arr1_value_count = array_value_counts(arr1);
    let arr2_value_count = array_value_counts(arr2);

    // println!("{:?}", arr1_value_count);
    // println!("{:?}", arr2_value_count);
    let len_map_arr1: usize = arr1_value_count.len();
    let len_map_arr2: usize = arr2_value_count.len();

    // println!(
    //     "unique value of array1 {:?}, unique value of array2 {:?}",
    //     len_map_arr1, len_map_arr2
    // );

    // init empty crosstab
    let mut result = Array2::<i32>::zeros((len_map_arr1, len_map_arr2));
    // println!("{:?}", result);

    for ((_i, a), (_j, b)) in inv_idx1.iter().enumerate().zip(inv_idx2.iter().enumerate()) {
        result[(*a as usize, *b as usize)] += 1;
        // println!("Index i : {}, Index j : {}", a, b)
    }
    // println!("{:?}", result);

    result
}
