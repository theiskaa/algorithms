// 2022 - Day 4 - Camp Cleanup (Part 2)

/* Problem Statement:
   It seems like there is still quite a bit of duplicate work planned.
   Instead, the Elves would like to know the number of pairs that overlap at all.

   In the above example, the first two pairs (2-4,6-8 and 2-3,4-5) don't overlap,
   while the remaining four pairs (5-7,7-9, 2-8,3-7, 6-6,4-6, and 2-6,4-8) do overlap:

   - 5-7,7-9 overlaps in a single section, 7.
   - 2-8,3-7 overlaps all of the sections 3 through 7.
   - 6-6,4-6 overlaps in a single section, 6.
   - 2-6,4-8 overlaps in sections 4, 5, and 6.

   So, in this example, the number of overlapping assignment pairs is 4.

   In how many assignment pairs do the ranges overlap?
*/

use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(unused_variables, dead_code)]

fn main() {
    // Read input and parse to vector value.
    let file = File::open("input.txt").unwrap();
    let input = parse_input(file);

    let result = camp_cleanup_2(input);
    println!(
        "AoC:2022 • Day 4 • Camp Cleanup (Part 2)\nResult: {}",
        result
    )
}

// Gets parsed input as Vector of tuple of Strings.
// Loops through that parsed data, generates first and second pair's full-path digits.
// Checks if first's any element are included in second vector. If included updates
// the [sum]. And at final step returns the [sum] as result.
fn camp_cleanup_2(input: Vec<(String, String)>) -> i32 {
    let mut sum: i32 = 0;

    for i in input.iter() {
        let first: Vec<i32> = generate_digits(i.0.clone());
        let second: Vec<i32> = generate_digits(i.1.clone());

        let includes = first.iter().any(|d| second.contains(d));
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
