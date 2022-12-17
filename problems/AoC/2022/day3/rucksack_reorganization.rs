// 2022 - Day 3 - Rucksack Reorganization

/* Problem Statement:
*/

use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(unused_variables, dead_code)]

fn main() {
    // Read input and parse to vector value.
    let file = File::open("input.txt").unwrap();
    let input = parse_input(file);

    let result = rucksack_reorganization(input);
    println!(
        "AoC:2022 • Day 3 • Rucksack Reorganization\nResult: {}",
        result
    )
}

// Takes the parsed input as Vector of String(s).
// Loops through it, finds each string's compartments' only share item, and
// adds priority of that only share item to general [sum] variable.
// Then returns that [sum].
fn rucksack_reorganization(input: Vec<String>) -> i32 {
    let mut sum: i32 = 0;

    for s in input {
        let (first, second): (String, String) = s
            .chars()
            .take(s.len() / 2)
            .zip(s.chars().skip(s.len() / 2))
            .unzip();

        let letter = first.chars().find(|&f| second.contains(f)).unwrap_or(' ');

        let mut priority: i32 = 0;
        if letter.is_lowercase() {
            priority = (letter as i32) - ('a' as i32) + 1;
        } else if letter.is_uppercase() {
            priority = (letter as i32) - ('A' as i32) + 27;
        }

        sum += priority;
    }

    return sum;
}

// Parses the [input.txt] file to the Vector of string.
fn parse_input(data: File) -> Vec<String> {
    let reader = BufReader::new(data);

    let mut res: Vec<String> = Vec::new();
    for line in reader.lines() {
        let l = line.unwrap().to_string();
        res.push(l);
    }

    return res;
}
