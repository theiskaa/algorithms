// Single Number - https://leetcode.com/problems/single-number/

/* Problem Statement:
 * Given a non-empty array of integers nums, every element appears twice except for one. Find that single one.
 * You must implement a solution with a linear runtime complexity and use only constant extra space.
 */

/* Examples:
 *
 * Input: nums = [2,2,1]
 * Output: 1
 *
 * Input: nums = [4,1,2,1,2]
 * Output: 4
 *
 * Input: nums = [1]
 * Output: 1
 */

#![allow(dead_code, unused_variables)]

use std::collections::HashMap;

fn main() {
    let tests: HashMap<Vec<i32>, i32> = HashMap::from([
        (vec![2, 2, 1], 1),
        (vec![4, 1, 2, 1, 2], 4),
        (vec![1], 1),
        (vec![1, 2, 2, 2, 5, 5], 1),
    ]);

    for (nums, expected) in tests {
        let res: i32 = single_number(nums.clone());
        if res != expected {
            println!(
                "Invalid result: expected: {}, got: {} --> in {:?}",
                expected,
                res,
                nums.clone()
            );
        } else {
            println!("Valid result: {} --> in {:?}", res, nums.clone());
        }
    }
}

/* Classical [for loop] solution.
 *   Details: [5ms] and [2.1mb memory]
 */
fn single_number(nums: Vec<i32>) -> i32 {
    let mut res: i32 = 0;

    for n in nums.iter() {
        res = n ^ res;
    }

    res
}

/* Rustacean solution.
 *   Details: [0ms] and [2.3mb memory]
 */
fn single_number_rustacean(nums: Vec<i32>) -> i32 {
    nums.iter().fold(0, |x, y| x ^ y)
}
