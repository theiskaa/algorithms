// 2022 - Day 5 - Supply Stacks (Part 1)

/* Problem Statement:
   The expedition can depart as soon as the final supplies have been unloaded from the ships.
   Supplies are stored in stacks of marked crates, but because the needed supplies are buried
   under many other crates, the crates need to be rearranged.

   The ship has a giant cargo crane capable of moving crates between stacks.
   To ensure none of the crates get crushed or fall over, the crane operator will
   rearrange them in a series of carefully-planned steps.
   After the crates are rearranged, the desired crates will be at the top of each stack.

   The Elves don't want to interrupt the crane operator during this delicate procedure,
   but they forgot to ask her which crate will end up where, and they want to be ready to
   unload them as soon as possible so they can embark.

   They do, however, have a drawing of the starting stacks of crates and the rearrangement
   procedure (your puzzle input). For example:
    --------------------
   |     [D]            |
   | [N] [C]            |
   | [Z] [M] [P]        |
   |  1   2   3         |
   |                    |
   | move 1 from 2 to 1 |
   | move 3 from 1 to 3 |
   | move 2 from 2 to 1 |
   | move 1 from 1 to 2 |
    --------------------
   In this example, there are three stacks of crates. Stack 1 contains two crates:
   crate Z is on the bottom, and crate N is on top. Stack 2 contains three crates;
   from bottom to top, they are crates M, C, and D. Finally, stack 3 contains a single crate, P.

   Then, the rearrangement procedure is given. In each step of the procedure,
   a quantity of crates is moved from one stack to a different stack.
   In the first step of the above rearrangement procedure, one crate is moved from
   stack 2 to stack 1, resulting in this configuration:
    -------------
   | [D]         |
   | [N] [C]     |
   | [Z] [M] [P] |
   |  1   2   3  |
    -------------
   In the second step, three crates are moved from stack 1 to stack 3.
   Crates are moved one at a time, so the first crate to be moved (D)
   ends up below the second and third crates:
    ------------
   |        [Z] |
   |        [N] |
   |    [C] [D] |
   |    [M] [P] |
   | 1   2   3  |
    ------------
   Then, both crates are moved from stack 2 to stack 1. Again,
   because crates are moved one at a time, crate C ends up below crate M:
    -------------
   |         [Z] |
   |         [N] |
   | [M]     [D] |
   | [C]     [P] |
   |  1   2   3  |
    -------------
   Finally, one crate is moved from stack 1 to stack 2:
    -------------
   |         [Z] |
   |         [N] |
   |         [D] |
   | [C] [M] [P] |
   |  1   2   3  |
    ------------
   The Elves just need to know which crate will end up on top of each stack;
   in this example, the top crates are C in stack 1, M in stack 2, and Z in stack 3,
   so you should combine these together and give the Elves the message CMZ.

   After the rearrangement procedure completes, what crate ends up on top of each stack?
*/

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(unused_variables, dead_code)]

fn main() {
    // Read input and parse to appropriate value.
    let file = File::open("input.txt").unwrap();
    let input = parse_input(file);

    let result = supply_stacks(input);
    println!(
        "AoC:2022 • Day 5 • Supply Stacks (Part 1)\nResult: {}",
        result
    )
}

// Takes full parsed tuple input (first the default stack info and second instructions vector).
// Then runs through the instructions vector, and makes update to default stack appropriate
// to each line of instruction.
//
// When instructions are completed, takes the final top elements of modified stack and returns it
// as string.
fn supply_stacks(input: (HashMap<i32, Vec<char>>, Vec<(i32, i32, i32)>)) -> String {
    let mut stack: HashMap<i32, Vec<char>> = input.clone().0.clone();
    for (mv, from, to) in input.1.iter() {
        let mut m = stack.get(from).unwrap().clone();

        let mut movable = m.split_off(m.len() - (*mv as usize));
        movable.reverse();

        let mut moved = stack.get(to).unwrap().clone();
        moved.append(&mut movable.clone());

        stack.insert(from.clone(), m.clone());
        stack.insert(to.clone(), moved.clone());
    }

    let mut kstack: Vec<&i32> = stack.keys().collect();
    kstack.sort();

    let mut res = String::new();
    for k in kstack.iter() {
        let v = stack.get(k).unwrap().clone();
        res.push(v[v.len() - 1]);
    }

    return res;
}

// Parses the [input.txt] file to two part:
// - The default stack data, using the [parse_stack_line].
// - The instructions vector, using the [parse_input].
fn parse_input(data: File) -> (HashMap<i32, Vec<char>>, Vec<(i32, i32, i32)>) {
    let reader = BufReader::new(data);

    let mut stack: HashMap<i32, Vec<char>> = HashMap::new();
    let mut instructions: Vec<(i32, i32, i32)> = Vec::new();

    for line in reader.lines() {
        let l = line.unwrap().to_string();

        // Start reading the instructions.
        if l.clone().chars().nth(0) == Some('m') {
            let pps = parse_instruction(l.clone());
            instructions.push(pps);
        } else {
            // Continue reading and updating the stack.
            let current = parse_stack_line(l.clone());
            for (k, v) in current.iter() {
                let mut old: Vec<char> = match stack.get(&k) {
                    None => vec![],
                    Some(vs) => vs.to_vec(),
                };

                old.insert(0, *v);
                stack.insert(*k, old);
            }
        }
    }

    return (stack, instructions);
}

// Parses the single line of stack and creates appropriate hash-map stack clone.
fn parse_stack_line(line: String) -> HashMap<i32, char> {
    let mut res: HashMap<i32, char> = HashMap::new();

    for i in 0..line.clone().chars().count() {
        let c: char = line.clone().chars().nth(i).unwrap();
        if c != '[' {
            continue;
        }

        let stack = (((i as f32) / 4.0).floor() as i32) + 1;
        res.insert(stack, line.clone().chars().nth(i + 1).unwrap());
    }

    return res;
}

// Parses the concrete instruction line taken from [parse_input].
fn parse_instruction(line: String) -> (i32, i32, i32) {
    let l = line
        .replace("move ", "")
        .replace(" from ", ",")
        .replace(" to ", ",");

    let parsed: Vec<i32> = l.split(",").map(|a| a.parse::<i32>().unwrap()).collect();
    return (parsed[0], parsed[1], parsed[2]);
}
