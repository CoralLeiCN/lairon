use ndarray::Array1;
use ndarray::Array2;
use std::collections::HashSet;

use numpy::ndarray;

pub fn crosstab(arr1: &Array1<i32>, arr2: &Array1<i32>) -> Array2<usize> {
    // Get unique values
    let unique1: Vec<i32> = arr1
        .iter()
        .copied()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    let unique2: Vec<i32> = arr2
        .iter()
        .copied()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    // Sort unique values
    let mut unique1 = unique1;
    let mut unique2 = unique2;
    unique1.sort();
    unique2.sort();

    // Create result matrix
    let mut result = Array2::<usize>::zeros((unique1.len(), unique2.len()));

    // Fill the matrix with frequencies
    for (i, &val1) in unique1.iter().enumerate() {
        for (j, &val2) in unique2.iter().enumerate() {
            let count = arr1
                .iter()
                .zip(arr2.iter())
                .filter(|(&a, &b)| a == val1 && b == val2)
                .count();
            result[[i, j]] = count;
        }
    }

    result
}
