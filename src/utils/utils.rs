use ndarray::Array1;
use std::collections::HashMap;

pub fn argsort<T: PartialOrd>(arr: &Array1<T>) -> Vec<usize> {
    let mut indices: Vec<usize> = (0..arr.len()).collect();
    indices.sort_by(|&a, &b| arr[a].partial_cmp(&arr[b]).unwrap());
    indices
}

pub fn array_value_counts(
    arr: &ndarray::ArrayBase<ndarray::OwnedRepr<i32>, ndarray::Dim<ndarray::IxDynImpl>>,
) -> HashMap<&i32, i32> {
    let mut map = HashMap::new();
    let flat_view: ndarray::iter::Iter<'_, i32, ndarray::Dim<ndarray::IxDynImpl>> =
        arr.view().into_dyn().into_iter();

    for item in flat_view {
        *map.entry(item).or_insert(0) += 1;
    }
    map
}
