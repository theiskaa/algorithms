// Valid Palindrome - https://leetcode.com/problems/valid-palindrome

/* Problem Statement:
 * A phrase is a palindrome if, after converting all uppercase letters 
 * into lowercase letters and removing all non-alphanumeric characters, 
 * it reads the same forward and backward. Alphanumeric characters include 
 * letters and numbers.
 *
 * Given a string s, return true if it is a palindrome, or false otherwise.
 */

/* Examples:
 *
 * Input: s = "race a car"
 * Output: false
 * Explanation: "raceacar" is not a palindrome.
 *
 * Input: s = "race car"
 * Output: true
 * Explanation: "racecar" is a palindrome.
 */

use std::collections::HashMap;

fn main() {
    let tests: HashMap<String, bool> = HashMap::from([
        (String::from("race a car"), false),
        (String::from("I'm me"), false),
        (String::from("race car"), true),
        (String::from("A man, a plan, a canal: Panama"), true),
        (String::from(" "), true),
    ]);

    for (s, result) in tests {
        let res: bool = is_palindrome(s);
        if result == res {
            println!("Correct answer:    it {} palindrome -> {}", "is", res);
        } else {
            println!("Incorrect answer: it {} palindrome -> {}", "is not", res);
        }
    }
}

/* Generates a alphanumeric variant of given parameter(s) by looping the [s.chars()].
 * Creates a reversed variant of [alphanumeric] string.
 * Then checks if [alphanumeric] equals to [reversed-alphanumeric].
 */
fn is_palindrome(s: String) -> bool {
    let mut alphanumeric: String = String::new();

    for c in s.chars() {
        if c.is_alphanumeric() {
            alphanumeric.push(c.to_ascii_lowercase());
        }
    }

    let reversed: String = alphanumeric.chars().rev().collect();
    alphanumeric == reversed
}
