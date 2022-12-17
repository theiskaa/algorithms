// 2022 - Day 3 - Rucksack Reorganization (Part 2)

/* Problem Statement:
   For safety, the Elves are divided into groups of three.
   Every Elf carries a badge that identifies their group.
   For efficiency, within each group of three Elves,
   the badge is the only item type carried by all three Elves.
   That is, if a group's badge is item type B, then all three Elves will
   have item type B somewhere in their rucksack, and at most two of
   the Elves will be carrying any other item type.

   The problem is that someone forgot to put this year's updated
   authenticity sticker on the badges. All of the badges need to be
   pulled out of the rucksacks so the new authenticity stickers can be attached.

   Additionally, nobody wrote down which item type corresponds to each group's badges.
   The only way to tell which item type is the right one is by finding the one item
   type that is common between all three Elves in each group.

   Every set of three lines in your list corresponds to a single group,
   but each group can have a different badge item type. So, in the above example,
   the first group's rucksacks are the first three lines:

   [
     vJrwpWtwJgWrhcsFMMfFFhFp
     jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
     PmmdzqPrVvPwwTWBwg
   ]

   And the second group's rucksacks are the next three lines:

   [
     wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
     ttgJtRGJQctTZtZT
     CrZsJsPPZsGzwwsLwLmpwMDw
   ]

   In the first group, the only item type that appears in all three rucksacks is lowercase r;
   this must be their badges. In the second group, their badge item type must be Z.

   Priorities for these items must still be found to organize the sticker attachment efforts:
   here, they are 18 (r) for the first group and 52 (Z) for the second group. The sum of these is 70.

   Find the item type that corresponds to the badges of each three-Elf group.
   What is the sum of the priorities of those item types?
*/

use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(unused_variables, dead_code)]

fn main() {
    // Read input and parse to vector value.
    let file = File::open("input.txt").unwrap();
    let input = parse_input(file);

    let result = rucksack_reorganization_2(input);
    println!(
        "AoC:2022 • Day 3 • Rucksack Reorganization (Part 2)\nResult: {}",
        result
    )
}

// Takes the parsed input as Vector of Vector(s) of String.
// Loops through it, finds each sub vector's elements' only share item type.
// Adds priority of that only share item to general [sum] variable.
// Then returns that [sum].
fn rucksack_reorganization_2(input: Vec<Vec<String>>) -> i32 {
    let mut sum: i32 = 0;

    for s in input {
        let letter = s[0]
            .chars()
            .find(|&c| s[1].contains(c) && s[2].contains(c))
            .unwrap_or(' ');

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

// Parses the [input.txt] file to the Vector of Vector of Strings.
// As provided in the problem statement, parse_input automatically
// splits the lines to 3-3 vectors of strings.
fn parse_input(data: File) -> Vec<Vec<String>> {
    let reader = BufReader::new(data);

    let mut res: Vec<Vec<String>> = Vec::new();
    let mut segment: Vec<String> = Vec::new();

    for line in reader.lines() {
        let l = line.unwrap().to_string();

        segment.push(l);
        if segment.len() == 3 {
            res.push(segment);
            segment = Vec::new();
        }
    }

    return res;
}
