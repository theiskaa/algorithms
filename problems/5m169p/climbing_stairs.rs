// Climbing Stairs - https://leetcode.com/problems/climbing-stairs

/* Problem Statement:
   You are climbing a staircase. It takes n steps to reach the top.
   Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?
 */

/* Examples:
   1.
    Input: n = 2
    Output: 2
    Explanation: There are two ways to climb to the top.
     - 1. 1 step + 1 step
     - 2. 2 steps

   2.
    Input: n = 3
    Output: 3
    Explanation: There are three ways to climb to the top.
     - 1. 1 step + 1 step + 1 step
     - 2. 1 step + 2 steps
     - 3. 2 steps + 1 step
*/

use std::collections::HashMap;

fn main() {
    let tests: HashMap<i32, i32> = HashMap::from([
        (2, 2),
        (5, 8), 
    ]);

    println!("Climbing Stairs:");
    let mut i: i32 = 1;
    for (n, expected) in tests {
        let got = climb_stairs(n);
        println!(" {}'th input | expected: {}, got: {}", i, expected, got);
        i += 1;
    }
}

/* Dynamic Programming 
*  -------------------
*  one_step is updated by one_step + two_step
*  and two_step is updated by previous one_step.
*  - Steps for N=5
*     -----------------
*    | one: 1 ~ two: 0 |
*    | one: 1 ~ two: 1 |
*    | one: 2 ~ two: 1 |
*    | one: 3 ~ two: 2 |
*    | one: 5 ~ two: 3 |
*    | one: 8 ~ two: 5 |
*     -----------------
*/
pub fn climb_stairs(n: i32) -> i32 {
    let mut one_step: i32 = 1;
    let mut two_step: i32 = 0;

    for _ in 1..=n {
        let combined = one_step + two_step;
        two_step = one_step;
        one_step = combined;
    }

    return one_step
}
