// Valid Anagram - https://leetcode.com/problems/valid-anagram

/* Problem Statement:
 * An Anagram is a word or phrase formed by rearranging
 * the letters of a different word or phrase, typically
 * using all the original letters exactly once.
 */

/*
 * Examples:
 *
 * Input: s = "anagram", t = "nagaram"
 * Output: true
 * Explanation: "sorted s & t would be same, so it's anagram"
 *
 * Input: s = "rat", t = "car"
 * Output: false
 * Explanation: "sorted s & t would not be same, so it's not anagram"
 */

use std::collections::HashMap;

fn main() {
    let tests: HashMap<String, String> = HashMap::from([
        (String::from("anagram"), String::from("nagaram")),
        (String::from("rat"), String::from("car")),
    ]);

    for (s, t) in tests {
        let res: bool = is_anagram(s, t);
        if res {
            println!("It's anagram");
        } else {
            println!("It's not anagram");
        }
    }
}
/* Via Sorting
* -----------
* Check both parameters' length. If they aren't equal these values couldn't be anagram.
* Sort both parameters via "increasing" order. And check each match. If "nth" indexed value of
* [s] sorted slice, doesn't equals to the "nth" indexed value of [t] sorted slice, it means that
* they are not an anagrams of each other.
* If cases doesn't match, return true. We founded anagram.
*/
fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let s_sorted: Vec<char> = s.chars().collect();
    let t_sorted: Vec<char> = t.chars().collect();
    s_sorted.sort_by(|a, b| a.cmp(b));
    t_sorted.sort_by(|a, b| a.cmp(b));

    for i in 0..s_sorted.len() {
        if s_sorted[i] != t_sorted[i] {
            return false;
        }
    }

    true
}
