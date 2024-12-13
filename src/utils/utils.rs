use ndarray::Array1;
use numpy::ndarray::ArrayView1;
use std::collections::HashMap;

pub fn argsort<T: PartialOrd>(arr: ArrayView1<T>) -> Vec<usize> {
    let mut indices: Vec<usize> = (0..arr.len()).collect();
    indices.sort_by(|&a, &b| arr[a].partial_cmp(&arr[b]).unwrap());
    indices
}

pub fn array_value_counts(arr: ArrayView1<i32>) -> HashMap<i32, i32> {
    let mut map = HashMap::new();
    let flat_view = arr.iter();

    for &item in flat_view {
        *map.entry(item).or_insert(0) += 1;
    }
    map
}

pub fn inverse_index(arr: ArrayView1<i32>) -> Vec<i32> {
    // call argsort
    let perm: Vec<usize> = argsort(arr);

    // println!("Array: {:?}", arr);

    // println!("Sorted indices: {:?}", perm);
    // Descending order
    let desc_indices: Vec<_> = perm.into_iter().rev().collect();
    // println!("\nDescending indices: {:?}", desc_indices);
    // Print sorted array using indices
    let aux: Vec<_> = desc_indices.iter().map(|&i| arr[i]).collect();
    // println!("Sorted array: {:?}", aux);
    // masked the sorted array, if different from previous value
    let masked: Vec<_> = aux
        .iter()
        .scan(None, |prev, &x| {
            let mask = match *prev {
                Some(prev) if prev == x => false,
                _ => {
                    *prev = Some(x);
                    true
                }
            };
            Some(mask)
        })
        .collect();
    // println!("Masked array: {:?}", masked);

    // calculate cumulative sum for array masked
    let cumsum: Vec<_> = masked
        .iter()
        .scan(0, |state, &x| {
            *state += x as i32;
            Some(*state)
        })
        .collect();
    // println!("Cumulative sum: {:?}", cumsum);

    // -1 for cumsum
    let cumsum_index: Vec<_> = cumsum.iter().map(|&x| x - 1).collect();
    // println!("Cumulative sum index: {:?}", cumsum_index);
    // convert descending indices to array
    let desc_indices: Array1<usize> = Array1::from(desc_indices);

    let inv_idx: Vec<_> = desc_indices.iter().map(|&i| cumsum_index[i]).collect();
    // println!("Inverse index: {:?}", inv_idx);

    inv_idx
}
