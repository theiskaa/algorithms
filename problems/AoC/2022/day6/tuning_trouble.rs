// 2022 - Day 6 - Tuning Trouble (Part 1)

/* Problem Statement:
   The preparations are finally complete; you and the Elves leave camp on foot and 
   begin to make your way toward the star fruit grove.

   As you move through the dense undergrowth, one of the Elves gives you a handheld device. 
   He says that it has many fancy features, but the most important one to set up right now 
   is the communication system.

   However, because he's heard you have significant experience dealing with signal-based systems, 
   he convinced the other Elves that it would be okay to give you their one malfunctioning 
   device - surely you'll have no problem fixing it.

   As if inspired by comedic timing, the device emits a few colorful sparks.

   To be able to communicate with the Elves, the device needs to lock on to their signal. 
   The signal is a series of seemingly-random characters that the device receives one at a time.

   To fix the communication system, you need to add a subroutine to the device that detects a 
   start-of-packet marker in the datastream. In the protocol being used by the Elves, the 
   start of a packet is indicated by a sequence of four characters that are all different.

   The device will send your subroutine a datastream buffer (your puzzle input); 
   your subroutine needs to identify the first position where the four most recently received 
   characters were all different. Specifically, it needs to report the number of characters 
   from the beginning of the buffer to the end of the first such four-character marker.

   For example, suppose you receive the following datastream buffer:
    --------------------------------
   | mjqjpqmgbljsphdztnvjfqwrcgsmlb |
    --------------------------------
   After the first three characters (mjq) have been received, there haven't been enough characters 
   received yet to find the marker. The first time a marker could occur is after the fourth character 
   is received, making the most recent four characters mjqj. Because j is repeated, this isn't a marker.

   The first time a marker appears is after the seventh character arrives. Once it does, 
   the last four characters received are jpqm, which are all different. 
   In this case, your subroutine should report the value 7, because the first start-of-packet marker 
   is complete after 7 characters have been processed.

   Here are a few more examples:
    ------------------------------
   | bvwbjplbgvbhsrlpgdmjqwftvncz | -> first marker after character 5
    ------------------------------
    ------------------------------
   | nppdvjthqldpwncqszvftbrmjlhg | -> first marker after character 6
    ------------------------------
    -----------------------------------
   | nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg | -> first marker after character 10
    -----------------------------------
    ----------------------------------
   | zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw | -> first marker after character 11
    ----------------------------------

   How many characters need to be processed before the first start-of-packet marker is detected?
*/

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

#[allow(unused_variables, dead_code)]

fn main() {
    let file = File::open("input.txt").unwrap();
    let input = parse_input(file);

    let result = tuning_trouble(input);
    println!(
        "AoC:2022 • Day 6 • Tuning Trouble (Part 1)\nResult: {}",
        result
    )
}

// Runs through the input getc each char's next for item combined.
// and checks if the combined value has any repeated char or not.
// If it doens't have any, returns the i + 4 index.
fn tuning_trouble(input: String) -> i32 {
    let mut i: usize = 0;
    while i+3 < input.len() {
        let next = input.get(i..(i+4)).unwrap_or("").to_string(); 
        if !has_repeated_value(next) {
            return (i + 4) as i32;
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
