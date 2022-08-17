// Two Sum - https://leetcode.com/problems/two-sum/

/* Problem Statement:
 *  Given an array of integers [nums] and an integer [target],
 *  return indices of the two numbers such that they add up to [target].
 *  You may assume that each input would have exactly one solution,
 *  and you may not use the same element twice.
 *  You can return the answer in any order.
 */

use std::collections::HashMap;
use std::convert::TryInto;

fn main() {
    // A collection of [nums] and [targets]
    // Defined as [target] to [nums];
    let tests: HashMap<i32, Vec<i32>> = HashMap::from([
        (9, vec![2, 7, 11, 15]),
        (6, vec![2, 3, 4]),
        (6, vec![3, 3]),
        (11, vec![2, 7, 11, 15]),
    ]);

    for (target, nums) in tests {
        let hash_map: Vec<i32> = two_sum_hash_map(nums, target);
        println!(" - Hash Map Result is {:?}", hash_map);
    }
}

/* Hash Map Solution
 * -----------------
 * Loop through the nums, take each [element]'s and [target]'s difference
 * Check differences existent at hash-map, if does exists, return current's index and founded value's
 * index.
 * If not exists, insert at hash-map, as value => index.
 */
fn two_sum_hash_map(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut garbage: HashMap<i32, i32> = HashMap::new();

    for i in 0..nums.len() {
        let diff: i32 = target - nums[i];

        let val: Option<&i32> = garbage.get(&diff);

        match val {
            Some(v) => return vec![i.try_into().unwrap(), *v],
            None => {
                garbage.insert(nums[i], i.try_into().unwrap());
            }
        }
    }

    vec![]
}

/* Brute Force Solution
 * --------------------
 * Create two nested loops by incremented indexes. -> [i] and [j].
 * Check nums' [i]th element's and nums' [j]th element's sum, if it equals to the target
 * return the [i] and [j] as vector.

 fn two_sum_brute_force(nums: Vec<i32>, target: i32) -> Vec<i32> {
     for i in 0..nums.len() {
         for j in i + 1..nums.len() {
             if nums[i] + nums[j] == target {
                 return vec![i.try_into().unwrap(), j.try_into().unwrap()];
             }
         }
     }

     vec![]
 }
*/
