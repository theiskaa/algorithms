// 2022 - Day 6 - Tuning Trouble (Part 2)

/* Problem Statement:
   Your device's communication system is correctly detecting packets, but still isn't working. 
   It looks like it also needs to look for messages.

   A start-of-message marker is just like a start-of-packet marker, except it consists of 
   14 distinct characters rather than 4.

   Here are the first positions of start-of-message markers for all of the above examples:
    --------------------------------
   | mjqjpqmgbljsphdztnvjfqwrcgsmlb | -> first marker after character 19
    --------------------------------
    ------------------------------
   | bvwbjplbgvbhsrlpgdmjqwftvncz | -> first marker after character 23
    ------------------------------
    ------------------------------
   | nppdvjthqldpwncqszvftbrmjlhg | -> first marker after character 23
    ------------------------------
    -----------------------------------
   | nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg | -> first marker after character 29
    -----------------------------------
    ----------------------------------
   | zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw | -> first marker after character 26
    ----------------------------------
*/

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

#[allow(unused_variables, dead_code)]

fn main() {
    let file = File::open("input.txt").unwrap();
    let input = parse_input(file);

    let result = tuning_trouble_2(input);
    println!(
        "AoC:2022 • Day 6 • Tuning Trouble (Part 2)\nResult: {}",
        result
    )
}

// Runs through the input getc each char's next for item combined.
// and checks if the combined value has any repeated char or not.
// If it doens't have any, returns the i + 14 index.
fn tuning_trouble_2(input: String) -> i32 {
    let mut i: usize = 0;
    while i+14 < input.len() {
        let next = input.get(i..(i+14)).unwrap_or("").to_string(); 
        if !has_repeated_value(next) {
            return (i + 14) as i32;
        }

        i += 1;
    }

    return -1; // cannot find
}

// Checks if given string(s) has any repeated char or not.
fn has_repeated_value(s: String) -> bool {
    let mut m: HashMap<char, i32> = HashMap::new();
    for ch in s.chars() {
        match m.get(&ch) {
            None => m.insert(ch, 1),
            Some(_) => return true,
        };
    }

    false
}

// Reads the [input.txt] file as String.
fn parse_input(data: File) -> String {
    let reader = BufReader::new(data);
    reader.lines().map(|x| x.unwrap()).collect()
}
