// 2022 - Day 8 - Tree Top House (Part 1)

/* Problem Statement:
   The expedition comes across a peculiar patch of tall trees all planted carefully in a grid. 
   The Elves explain that a previous expedition planted these trees as a reforestation effort. 
   Now, they're curious if this would be a good location for a tree house.

   First, determine whether there is enough tree cover here to keep a tree house hidden. 
   To do this, you need to count the number of trees that are visible from outside the grid when 
   looking directly along a row or column.

   The Elves have already launched a quadcopter to generate a map with the height of each tree (your puzzle input). 
   For example:
   [
     30373
     25512
     65332
     33549
     35390
   ]

   Each tree is represented as a single digit whose value is its height, 
   where 0 is the shortest and 9 is the tallest.

   A tree is visible if all of the other trees between it and an edge of the grid are shorter than it. 
   Only consider trees in the same row or column; that is, only look up, down, left, or right from any given tree.

   All of the trees around the edge of the grid are visible - since they are already on the edge, 
   there are no trees to block the view. In this example, that only leaves the interior nine trees to consider:

   - The top-left 5 is visible from the left and top. (It isn't visible from the right or 
     bottom since other trees of height 5 are in the way.)
   - The top-middle 5 is visible from the top and right.
   - The top-right 1 is not visible from any direction; for it to be visible, 
     there would need to only be trees of height 0 between it and an edge.
   - The left-middle 5 is visible, but only from the right.
   - The center 3 is not visible from any direction; for it to be visible, 
     there would need to be only trees of at most height 2 between it and an edge.
   - The right-middle 3 is visible from the right.
   - In the bottom row, the middle 5 is visible, but the 3 and 4 are not.
   
   With 16 trees visible on the edge and another 5 visible in the interior, 
   a total of 21 trees are visible in this arrangement.

   Consider your map; how many trees are visible from outside the grid?
*/

use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(unused_variables, dead_code)]

fn main() {
    // Read input and parse to accaptable value.
    let file = File::open("input.txt").unwrap();
    let input = parse_input(file);

    let result = toptree_tree_house(input);
    println!(
        "AoC:2022 • Day 8 • Toptree Tree House(Part 1)\nResult: {}",
        result
    )
}

// First takes the default visible tree elements from edge, then loops through 
// sharped(input - already calculated edge) list and checks each tree's visibility
// from left, right, top, and bottom sides.
fn toptree_tree_house(input: Vec<Vec<i32>>) -> i32 {
    // Apply default viewable trees. 'cause all of the trees around the edge of the grid are visible - 
    // since they are already on the edge, there are no trees to block the view. 
    let mut res = (input.clone()[0].len() * 2 + (input.clone().len() - 2) * 2) as i32;

    for i in 1..input.len() - 1 {
        let row = input.clone()[i].clone();

        // In case of being one of the condition check [true],
        // we call continue to ignore other case checkers.
        // Because each check may be expansive.
        for l in 1..row.len() - 1 {

            let left = is_visible_row(l, row.clone(), -1);
            if left {
                res += 1;
                continue;
            }

            let right = is_visible_row(l, row.clone(), 1);
            if right {
                res += 1;
                continue;
            }

            let top = is_visible_column(i, l, input.clone(), -1);
            if top {
                res += 1;
                continue;
            }

            let bottom = is_visible_column(i, l, input.clone(), 1);
            if bottom {
                res += 1;
                continue;
            }
        }
    }

    return res;
}

// A function implemnetation for checking visibility of an tree element in same row.
// Direction decides the side of column, [left or right] - (-1 or 1).
fn is_visible_row(l: usize, row: Vec<i32>, direction: i32) -> bool {
    let start = { if direction == 1 { l.clone() + 1 } else { 0 } };
    let end = { if direction == 1 { row.clone().len() } else { l.clone() } };

    let current = row.clone()[l].clone();
    for i in start..end {
        if row.clone()[i] >= current {
            return false;
        } else {
        }
    }
    true
}

// A function implemnetation for checking visibility of an tree element in column.
// Direction decides the side of column, [top or bottom] - (-1 or 1).
fn is_visible_column(l: usize, rl: usize, column: Vec<Vec<i32>>, direction: i32) -> bool {
    let start = { if direction == 1 { l.clone() + 1 } else { 0 } };
    let end = { if direction == 1 { column.clone().len() } else { l.clone() } };

    let current = column.clone()[l].clone()[rl].clone();
    for i in start..end {
        if column.clone()[i].clone()[rl].clone() >= current {
            return false;
        }
    }

    true
}

// Parses the [input.txt] file to the accaptable value.
fn parse_input(data: File) -> Vec<Vec<i32>> {
    let reader = BufReader::new(data);

    let mut res: Vec<Vec<i32>> = Vec::new();
    let mut current: Vec<i32> = Vec::new();

    for line in reader.lines() {
        for nc in line.unwrap().to_string().chars() {
          current.push(nc.to_string().parse::<i32>().unwrap());
        }

        res.push(current.clone());
        current = Vec::new();
    }

    return res;
}
