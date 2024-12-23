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

// calculate the margins of a crosstab
pub fn margins(arr: &Array2<usize>) -> (Array1<usize>, Array1<usize>) {
    let row_sums = arr.sum_axis(ndarray::Axis(1));
    let col_sums = arr.sum_axis(ndarray::Axis(0));
    (row_sums, col_sums)
}

// test for crosstab
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crosstab() {
        let arr1 = ndarray::arr1(&[0, 2, 1, 2, 3, 4, 2, 2]);
        let arr2 = ndarray::arr1(&[0, 1, 2, 3, 5, 5, 2, 2]);
        let xtab_result = crosstab(&arr1, &arr2);
        let expected_result = ndarray::arr2(&[
            [1, 0, 0, 0, 0, 0],
            [0, 0, 1, 0, 0, 0],
            [0, 0, 3, 0, 0, 0],
            [0, 0, 0, 1, 0, 0],
            [0, 0, 0, 0, 0, 1],
        ]);
        assert_eq!(xtab_result, expected_result);
    }

    #[test]
    fn test_margins() {
        let arr = ndarray::arr2(&[
            [1, 0, 0, 0, 0, 0],
            [0, 0, 1, 0, 0, 0],
            [0, 0, 3, 0, 0, 0],
            [0, 0, 0, 1, 0, 0],
            [0, 0, 0, 0, 0, 1],
        ]);
        let (row_margins, col_margins) = margins(&arr);
        let expected_row_margins = ndarray::arr1(&[1, 1, 3, 1, 1]);
        let expected_col_margins = ndarray::arr1(&[1, 0, 4, 1, 0, 1]);
        assert_eq!(row_margins, expected_row_margins);
        assert_eq!(col_margins, expected_col_margins);
    }
}
