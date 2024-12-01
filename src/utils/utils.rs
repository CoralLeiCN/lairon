use ndarray::Array1;

pub fn argsort<T: PartialOrd>(arr: &Array1<T>) -> Vec<usize> {
    let mut indices: Vec<usize> = (0..arr.len()).collect();
    indices.sort_by(|&a, &b| arr[a].partial_cmp(&arr[b]).unwrap());
    indices
}
