// 2022 - Day 3 - Rucksack Reorganization (Part 1)

/* Problem Statement:
   One Elf has the important job of loading all of the rucksacks with supplies
   for the jungle journey. Unfortunately, that Elf didn't quite follow the
   packing instructions, and so a few items now need to be rearranged.

   Each rucksack has two large compartments. All items of a given type
   are meant to go into exactly one of the two compartments.
   The Elf that did the packing failed to follow this rule for exactly one
   item type per rucksack.

   The Elves have made a list of all of the items currently in each
   rucksack (your puzzle input), but they need your help finding the errors.
   Every item type is identified by a single lowercase or uppercase letter
   (that is, a and A refer to different types of items).

   The list of items for each rucksack is given as characters all on a single line.
   A given rucksack always has the same number of items in each
   of its two compartments, so the first half of the characters represent items
   in the first compartment, while the second half of the characters represent
   items in the second compartment.

   For example, suppose you have the following list of contents from six rucksacks:

   [
     vJrwpWtwJgWrhcsFMMfFFhFp
     jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
     PmmdzqPrVvPwwTWBwg
     wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
     ttgJtRGJQctTZtZT
     CrZsJsPPZsGzwwsLwLmpwMDw
   ]

   - The first rucksack contains the items vJrwpWtwJgWrhcsFMMfFFhFp,
     which means its first compartment contains the items vJrwpWtwJgWr,
     while the second compartment contains the items hcsFMMfFFhFp.
     The only item type that appears in both compartments is lowercase p.
   - The second rucksack's compartments contain jqHRNqRjqzjGDLGL and rsFMfFZSrLrFZsSL.
     The only item type that appears in both compartments is uppercase L.
   - The third rucksack's compartments contain PmmdzqPrV and vPwwTWBwg;
     the only common item type is uppercase P.
   - The fourth rucksack's compartments only share item type v.
   - The fifth rucksack's compartments only share item type t.
   - The sixth rucksack's compartments only share item type s.

   To help prioritize item rearrangement,
   every item type can be converted to a priority:

   • Lowercase item types a through z have priorities 1 through 26.
   • Uppercase item types A through Z have priorities 27 through 52.

   In the above example, the priority of the item type that appears in both
   compartments of each rucksack is
   16 (p), 38 (L), 42 (P), 22 (v), 20 (t), and 19 (s); the sum of these is 157.

   Find the item type that appears in both compartments of each rucksack.
   What is the sum of the priorities of those item types?
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
        "AoC:2022 • Day 3 • Rucksack Reorganization (Part 1)\nResult: {}",
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
