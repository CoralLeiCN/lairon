// import the ndarray crate
use numpy::ndarray;
// import the crosstab function from lib.rs
pub mod descriptive;
pub mod utils;
use crate::descriptive::pivot::crosstab;
use crate::descriptive::pivot::margins;

fn main() {
    // call argsort
    let arr1 = ndarray::arr1(&[0, 2, 1, 2, 3, 4, 2, 2]);
    let arr2 = ndarray::arr1(&[0, 1, 2, 3, 5, 5, 2, 2]);
    // call crosstab
    let xtab_result = crosstab(&arr1, &arr2);
    println!("{:?}", xtab_result);

    // call margins
    let (row_margins, col_margins) = margins(&xtab_result);
    println!("row margins{:?}", row_margins);
    println!("col margins{:?}", col_margins);
}
