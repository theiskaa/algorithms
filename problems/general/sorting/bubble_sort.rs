// Bubble Sort Algorithm implementation in Rust.
// See more: https://frontendmasters.com/courses/algorithms/implementing-bubble-sort/

use std::collections::HashMap;

fn main() {
    let tests: HashMap<Vec<i32>, Vec<i32>> = HashMap::from([
        (vec![1, 3, 5, 7, 9], vec![5, 3, 1, 9, 7]),
        (vec![-2, -1, 3, 5, 6, 7, 9], vec![-1, -2, 3, 5, 6, 7, 9]),
    ]);

    for (sorted, unsorted) in tests.iter() {
        let result: Vec<i32> = bubble_sort(unsorted.clone());
        if result != *sorted {
            println!("Wrong Result | Want: {:?} | Got: {:?}", sorted, result);
        } else {
            println!("{:?} <<sorted to>> {:?}", unsorted.clone(), result);
        }
    }
}

fn bubble_sort(array: Vec<i32>) -> Vec<i32> {
    let mut sorted: Vec<i32> = array.clone();

    for i in 0..sorted.len() {
        for j in 0..(sorted.len() - 1 - i) {
            if sorted[j] > sorted[j + 1] {
                let j_holder = sorted[j];

                sorted[j] = sorted[j + 1];
                sorted[j + 1] = j_holder;
            }
        }
    }

    sorted
}
