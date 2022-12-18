// 2022 - Day 4 - Camp Cleanup (Part 1)

/* Problem Statement:
   Space needs to be cleared before the last supplies can be unloaded from the ships,
   and so several Elves have been assigned the job of cleaning up sections of the camp.
   Every section has a unique ID number, and each Elf is assigned a range of section IDs.

   However, as some of the Elves compare their section assignments with each other,
   they've noticed that many of the assignments overlap.
   To try to quickly find overlaps and reduce duplicated effort,
   the Elves pair up and make a big list of the section assignments for each pair (your puzzle input).

   For example, consider the following list of section assignment pairs:
   [
     2-4,6-8
     2-3,4-5
     5-7,7-9
     2-8,3-7
     6-6,4-6
     2-6,4-8
   ]

   For the first few pairs, this list means:

   - Within the first pair of Elves, the first Elf was assigned sections 2-4
     (sections 2, 3, and 4), while the second Elf was assigned sections 6-8 (sections 6, 7, 8).
   - The Elves in the second pair were each assigned two sections.
   - The Elves in the third pair were each assigned three sections:
     one got sections 5, 6, and 7, while the other also got 7, plus 8 and 9.
   - This example list uses single-digit section IDs to make it easier to draw;
     your actual list might contain larger numbers. Visually, these pairs of
     section assignments look like this:
    ----------------
   | .234.....  2-4 | Some of the pairs have noticed that one of their assignments
   | .....678.  6-8 | fully contains the other. For example, 2-8 fully contains 3-7,
   |                | and 6-6 is fully contained by 4-6. In pairs where one assignment
   | .23......  2-3 | fully contains the other, one Elf in the pair would be exclusively
   | ...45....  4-5 | cleaning sections their partner will already be cleaning,
   |                | so these seem like the most in need of reconsideration.
   | ....567..  5-7 | In this example, there are 2 such pairs.
   | ......789  7-9 |
   |                |
   | .2345678.  2-8 |
   | ..34567..  3-7 |
   |                |
   | .....6...  6-6 |
   | ...456...  4-6 |
   |                |
   | .23456...  2-6 |
   | ...45678.  4-8 |
    ----------------

   In how many assignment pairs does one range fully contain the other?
*/

use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(unused_variables, dead_code)]

fn main() {
    // Read input and parse to vector value.
    let file = File::open("input.txt").unwrap();
    let input = parse_input(file);

    let result = camp_cleanup(input);
    println!(
        "AoC:2022 • Day 4 • Camp Cleanup (Part 1)\nResult: {}",
        result
    )
}

// Gets parsed input as Vector of tuple of Strings.
// Loops through that parsed data, generates first and second pair's full-path digits.
// Then checks if small full-path digits are placed in the big one or not.
// If yes updates the [sum] with 1. and finally returns sum as result.
fn camp_cleanup(input: Vec<(String, String)>) -> i32 {
    let mut sum: i32 = 0;

    for i in input.iter() {
        let first: Vec<i32> = generate_digits(i.0.clone());
        let second: Vec<i32> = generate_digits(i.1.clone());

        // Define the actual search base: big, and searchable array: small.
        let (small, big) = if first.len() > second.len() {
            (second.clone(), first.clone())
        } else {
            (first.clone(), second.clone())
        };

        let includes = small.iter().all(|d| big.contains(d));
        if includes {
            sum += 1;
        }
    }

    return sum;
}

// Generates the full-range digits from exact one pair.
fn generate_digits(pair: String) -> Vec<i32> {
    let pair_vec: Vec<i32> = pair.split('-').map(|a| a.parse::<i32>().unwrap()).collect();
    return (pair_vec[0]..pair_vec[1] + 1).collect();
}

// Parses the [input.txt] file to the Vector of String tuples.
fn parse_input(data: File) -> Vec<(String, String)> {
    let reader = BufReader::new(data);

    let mut res: Vec<(String, String)> = Vec::new();
    for line in reader.lines() {
        let l = line.unwrap().to_string();
        let ls: Vec<String> = l.split(',').map(|a| a.to_string()).collect();
        res.push((ls[0].clone(), ls[1].clone()));
    }

    return res;
}
