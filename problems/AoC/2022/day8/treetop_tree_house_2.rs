// 2022 - Day 8 - Tree Top House (Part 2)

/* Problem Statement:
   Content with the amount of tree cover available, the Elves just need to know the best spot to build their tree house: 
   they would like to be able to see a lot of trees.

   To measure the viewing distance from a given tree, look up, down, left, and right from that tree; 
   stop if you reach an edge or at the first tree that is the same height or taller than the tree under consideration. 
   (If a tree is right on the edge, at least one of its viewing distances will be zero.)

   The Elves don't care about distant trees taller than those found by the rules above; 
   the proposed tree house has large eaves to keep it dry, so they wouldn't be able to see higher than the tree house anyway.

   In the example above, consider the middle 5 in the second row:
   [
     30373
     25512
     65332
     33549
     35390
   ]

   - Looking up, its view is not blocked; it can see 1 tree (of height 3).
   - Looking left, its view is blocked immediately; it can see only 1 tree (of height 5, right next to it).
   - Looking right, its view is not blocked; it can see 2 trees.
   - Looking down, its view is blocked eventually; it can see 2 trees (one of height 3, then the tree of 
     height 5 that blocks its view).

   A tree's scenic score is found by multiplying together its viewing distance in each of the four directions. 
   For this tree, this is 4 (found by multiplying 1 * 1 * 2 * 2).

   However, you can do even better: consider the tree of height 5 in the middle of the fourth row:
   [
     30373
     25512
     65332
     33549
     35390
   ]

   - Looking up, its view is blocked at 2 trees (by another tree with a height of 5).
   - Looking left, its view is not blocked; it can see 2 trees.
   - Looking down, its view is also not blocked; it can see 1 tree.
   - Looking right, its view is blocked at 2 trees (by a massive tree of height 9).

   This tree's scenic score is 8 (2 * 2 * 1 * 2); this is the ideal spot for the tree house.
   Consider each tree on your map. What is the highest scenic score possible for any tree?
*/

use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(unused_variables, dead_code)]

fn main() {
    // Read input and parse to accaptable value.
    let file = File::open("input.txt").unwrap();
    let input = parse_input(file);

    let result = toptree_tree_house_2(input);
    println!(
        "AoC:2022 • Day 8 • Toptree Tree House(Part 2)\nResult: {}",
        result
    )
}


// Loops throgh input and each input element's row, 
// calculates left, right, top, and bottom side visible trees' values by taking their product.
// and generates maximum value from there.
fn toptree_tree_house_2(input: Vec<Vec<i32>>) -> i32 {
    let mut res = 0;

    for i in 0..input.len() {
        let row = input.clone()[i].clone();

        for l in 0..row.len() {
            let mut row_res: i32 = 1;

            let left = { if l == 0 { 1 } else { row_visible_trees(l, row.clone(), -1) } };
            row_res *= left; 

            let right = { if l == row.len() - 1 { 1 } else { row_visible_trees(l, row.clone(), 1) } };
            row_res *= right;

            let top = { if i == 0 { 1 } else { column_visible_trees(i, l, input.clone(), -1) } };
            row_res *= top;

            let bottom = { if i == input.len() - 1 { 1 } else { column_visible_trees(i, l, input.clone(), 1) } };
            row_res *= bottom;

            if row_res > res {
                res = row_res;
            }
        }
    }

    return res;
}

// A function implemnetation for counting visible trees in same row.
// Direction decides the side of column, [top or bottom] - (-1 or 1).
fn row_visible_trees(l: usize, row: Vec<i32>, direction: i32) -> i32 {
    let current = row.clone()[l].clone();

    let mut res: i32 = 0;

    let mut i = ({ if direction == 1 { l.clone() + 1 } else { l.clone() - 1 } }) as i32;
    let end = ({ if direction == 1 { row.clone().len() } else { 0 } }) as i32;

    while if direction == 1 { i < end } else { i >= end } {
        let r = row.clone()[i as usize];
        if current >= r {
            res += 1;
            if current == r { return res; }
        } else {
            return res + 1;
        }

        i += direction;
    }

    res
}

// A function implemnetation for counting visible trees in same column.
// Direction decides the side of column, [top or bottom] - (-1 or 1).
fn column_visible_trees(l: usize, rl: usize, column: Vec<Vec<i32>>, direction: i32) -> i32 {
    let current = column.clone()[l].clone()[rl].clone();

    let mut res: i32 = 0;
    
    let mut i = ({ if direction == 1 { l.clone() + 1 } else { l.clone() - 1 } }) as i32;
    let end = ({ if direction == 1 { column.clone().len() } else { 0 } }) as i32;

    while if direction == 1 { i < end } else { i >= end } {
        let c = column.clone()[i as usize].clone()[rl].clone();
        if current >= c {
            res += 1;
            if current == c { return res; }
        } else {
            return res + 1;
        }

        i += direction;
    }

    res
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
