// Ransom Note - https://leetcode.com/problems/ransom-note

/* Problem Statement:
 * Given two strings [ransomNote] and magazine, return true if
 * [ransomNote] can be constructed by using the letters
 * from magazine and false otherwise.
 * Each letter in magazine can only be used once in ransomNote.
 */

/* Examples:
 *
 * Input: ransomNote = "a", magazine = "b"
 * Output: false
 * Explanation: "You cannot generate "a" from "b""
 *
 * Input: ransomNote = "aa", magazine = "aab"
 * Output: true
 * Explanation: "You can generate "aa" from "aab""
*/

use std::collections::HashMap;

fn main() {
    let tests: HashMap<String, String> = HashMap::from([
        (String::from("a"), String::from("b")),
        (String::from("aa"), String::from("aab")),
        (String::from("iss"), String::from("jkhgalksjgiaslfkhhjja")),
    ]);

    for (ransom_note, magazine) in tests {
        let res = can_construct(ransom_note, magazine);
        if res {
            println!("Ransom note CAN be constructed by magazine -> {}", res);
        } else {
            println!("Ransom note CAN NOT be constructed by magazine -> {}", res);
        }
    }
}

/* Easy construct Solution
 * -----------------------
 * Loops through the [ransom_note], finds the character representation at magazine.
 *  - If it doesn't exists, returns false directly.
 *  - If some value does exists, removes it from variable, cause of further checking.
 *    (Since we can use one character for one).
 */
fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut m: String = String::from(magazine);

    for ch in ransom_note.chars() {
        match m.find(ch) {
            None => return false,
            Some(index) => m.remove(index),
        };
    }

    true
}

/* Hash Map Solution
 * -----------------
 * Breaks each given [String] parameter to [char]-to-[appearance] [HasMap].
 * Then checks [ransom_note]'s character appearance count in magazine via using hash-maps.
 ```
   fn can_construct(ransom_note: String, magazine: String) -> bool {
       let rn_map: HashMap<char, i32> = break_as_char_to_count(ransom_note);
       let m_map: HashMap<char, i32> = break_as_char_to_count(magazine);

       for (ch, co) in rn_map {
           let m_map_co: Option<&i32> = m_map.get(&ch);
           match m_map_co {
               Some(v) => {
                   if co > *v {
                       return false;
                   }
               }
               None => return false,
           }
       }

       true
   }

   fn break_as_char_to_count(target: String) -> HashMap<char, i32> {
       let mut res: HashMap<char, i32> = HashMap::new();

       for ch in target.chars() {
           let get: Option<&i32> = res.get(&ch);
           match get {
               Some(v) => res.insert(ch, v + 1),
               None => res.insert(ch, 1),
           };
       }

       res
   }
```
*/
