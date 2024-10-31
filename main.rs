use std::time::Instant;

use pyo3::prelude::*;
fn timeit<F>(f: F, iterations: usize) -> u128
where
    F: Fn() -> PyResult<String>,
{
    let mut total_duration: u128 = 0;

    for _ in 0..iterations {
        let start = Instant::now();
        f().unwrap();
        let duration = start.elapsed();
        total_duration += duration.as_nanos();
    }

    total_duration / iterations as u128
}
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}
fn main() {
    let iterations = 100000;
    let avg_time = timeit(|| sum_as_string(1, 2), iterations);
    println!(
        "Average time over {} iterations: {} nanoseconds per run",
        iterations, avg_time
    );
}
