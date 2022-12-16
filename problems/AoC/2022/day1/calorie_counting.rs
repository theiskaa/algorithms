// 2022 - Day 1 - Calorie Counting

/* Problem Statement
*  For example, suppose the Elves finish writing their items'
*  Calories and end up with the following list:
*  [
*    1000
*    2000
*    3000
*
*    4000
*
*    5000
*    6000
*
*    7000
*    8000
*    9000
*
*    10000
*  ]
*
*  This list represents the Calories of the food carried by five Elves:
*   - The first Elf is carrying food with 1000, 2000, and 3000 Calories, a total of 6000 Calories.
*   - The second Elf is carrying one food item with 4000 Calories.
*   - The third Elf is carrying food with 5000 and 6000 Calories, a total of 11000 Calories.
*   - The fourth Elf is carrying food with 7000, 8000, and 9000 Calories, a total of 24000 Calories.
*   - The fifth Elf is carrying one food item with 10000 Calories.
*
*  In case the Elves get hungry and need extra snacks, they need to know which Elf to ask:
*  they'd like to know how many Calories are being carried by the Elf carrying the most Calories.
*  In the example above, this is 24000 (carried by the fourth Elf).
*
*  Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?
*
*/

use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(unused_variables, dead_code)]

fn main() {
    // Read input and parse to vector value.
    let file = File::open("input.txt").unwrap();
    let input = parse_input(file);

    let result = calorie_counting(input);
    println!("AoC:2022 • Day 1 • Calorie counting\nResult: {}", result)
}

// Takes the parsed input as Vector of Vector of i32.
// Loops through it, and takes the sum of each line.
// then compares it with [res] and if it is more replaces with res.
fn calorie_counting(input: Vec<Vec<i32>>) -> i32 {
    let mut res: i32 = 0;
    for calories in input.iter() {
        let sum_of_calories = calories.iter().fold(0, |x, y| x + y);
        if sum_of_calories > res {
            res = sum_of_calories.clone();
        }
    }

    res
}

// Parses the [input.txt] file to the Vector of Vector of i32.
fn parse_input(file: File) -> Vec<Vec<i32>> {
    let reader = BufReader::new(file);

    let mut res: Vec<Vec<i32>> = Vec::new();
    let mut current: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let l = line.unwrap();

        if !l.is_empty() {
            current.push(l.to_string().parse::<i32>().unwrap());
            continue;
        }

        res.push(current.clone());
        current = Vec::new();
    }

    return res;
}
