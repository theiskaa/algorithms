// Valid Parentheses - https://leetcode.com/problems/valid-parentheses/

/* Problem Statement:
 * Given a string s containing just the characters
 * '(', ')', '{', '}', '[' and ']',
 * determine if the input string is valid.
 *
 * An input string is valid if:
 * Open brackets must be closed by the same type of brackets.
 * Open brackets must be closed in the correct order.
*/

/* Examples:
 *
 * Input: s = "()"
 * Output: true
 *
 * Input: s = "()[]{}"
 * Output: true
 *
 * Input: s = "(]"
 * Output: false
 */

use std::collections::HashMap;

fn main() {
    let tests: HashMap<String, bool> = HashMap::from([
        (String::from("(]"), false),
        (String::from("()"), true),
        (String::from("()[]{}"), true),
        (String::from("]"), false),
        (String::from("([}}])"), false),
    ]);

    for (s, expected) in tests {
        let res: bool = is_valid(s.clone());
        if res {
            println!(
                "The {} parenthesis is valid: {} | expected result: {}",
                s.clone(),
                res,
                expected
            )
        } else {
            println!(
                "The {} parenthesis is not valid: {} | expected result: {}",
                s.clone(),
                res,
                expected
            )
        }
    }
}

/* Vector(Stack) Solution
 * ----------------------
 * Loops through the given string: s.
 * Checks each character's bracket type (if it's opening bracket or closing)
 *  - If char is opening bracket, pushes it to defined vector: [ss].
 *  - If char is closing bracket, and [ss] vector's last character is a same type of opening
 *    bracket, removes the last element of [ss] vector.
 *
 *  It works because, for example: if [s] is "()[]{}"
 *  loops through it, first is opening, so adds it to the vector.
 *  Second one is same type of closing, so removes the first added opening bracket.
 *  And so on ...
 *  So at the final, the [ss] vector has to be empty to represent the valid parentheses.
 */
fn is_valid(s: String) -> bool {
    let mut ss: Vec<char> = Vec::new();
    let close_to_open: HashMap<char, char> = HashMap::from([(')', '('), ('}', '{'), (']', '[')]);

    for ch in s.chars() {
        if ch == '(' || ch == '{' || ch == '[' {
            ss.push(ch);
            continue;
        }

        let last: char = match ss.last() {
            None => '0', // represents not valid value
            Some(v) => *v,
        };

        let ctp: char = match close_to_open.get(&ch) {
            None => '0', // represents not valid value
            Some(v) => *v,
        };

        if !ss.is_empty() && last == ctp {
            ss.pop();
        } else {
            // Break directly, because if we have unmatched closing parentheses
            // without same type of opening bracket. It couldn't be valid result.
            return false;
        }
    }

    ss.is_empty()
}
