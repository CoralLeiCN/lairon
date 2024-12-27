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

// modified from https://github.com/scipy/scipy/blob/92d2a8592782ee19a1161d0bf3fc2241ba78bb63/scipy/stats/contingency.py#L90-L135
// for 2d only
pub fn expected_freq_2d(arr: &Array2<usize>) -> Array2<f64> {
    let (row_margins, col_margins) = margins(&arr);
    let total = row_margins.sum();
    let mut result = Array2::<f64>::zeros(arr.dim());
    for i in 0..arr.dim().0 {
        for j in 0..arr.dim().1 {
            result[[i, j]] = (row_margins[i] as f64 * col_margins[j] as f64) / total as f64;
        }
    }
    result
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
            [1, 0, 0, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 1, 2, 1, 0],
            [0, 0, 0, 0, 1],
            [0, 0, 0, 0, 1],
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

    #[test]
    fn test_expected_frequencies() {
        let arr = ndarray::arr2(&[[10, 10, 20], [20, 20, 20]]);
        let expected_result = ndarray::arr2(&[[12., 12., 16.], [18., 18., 24.]]);
        let result = expected_frequencies(&arr);
        assert_eq!(result, expected_result);
    }
}
