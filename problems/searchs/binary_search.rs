// Binary Search - https://leetcode.com/problems/binary-search

/* Problem Statement:
 * Given an array of integers nums which is sorted in ascending order,
 * and an integer target, write a function to search target in nums.
 * If target exists, then return its index. Otherwise, return -1.
 *
 * You must write an algorithm with O(log n) runtime complexity.
*/

/* Examples:
 *
 * Input: nums = [-1,0,3,5,9,12] & target = 9
 * Output: 4
 * Explanation: 9 exists in nums and its index is 4
 *
*/

use std::collections::HashMap;

fn main() {
    let tests: HashMap<i32, Vec<i32>> = HashMap::from([
        (9, vec![-1, 0, 3, 5, 9, 12]),
        (5, vec![-12, -10, -1, 0, 3, 5, 9, 12]),
        (99, vec![-12, -10, -1, 0, 3, 5, 9, 12]),
    ]);

    for (target, nums) in tests.iter() {
        let result: i32 = search_loop(nums.to_vec(), *target);
        println!(" - Binary Search (by loop) result is {}", result);
    }
}

//
// Binary search implementation with single method implementing via loop.
// Check for the PERFECT ThePrimeagen explanation:
// - https://frontendmasters.com/courses/algorithms/implementing-binary-search/
//
fn search_loop(nums: Vec<i32>, target: i32) -> i32 {
    let mut low: f32 = 0.0;
    let mut high: f32 = nums.len() as f32;

    while low < high {
        let middle: f32 = ((low + high) / 2.0).floor();
        let value: i32 = nums[middle as usize];

        if value == target {
            return middle as i32;
        } else if value > target {
            high = middle.clone();
        } else {
            low = middle + 1.0;
        }
    }

    return -1;
}
