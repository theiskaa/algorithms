// 2022 - Day 5 - Supply Stacks (Part 2)

/* Problem Statement:
   As you watch the crane operator expertly rearrange the crates,
   you notice the process isn't following your prediction.

   Some mud was covering the writing on the side of the crane,
   and you quickly wipe it away. The crane isn't a CrateMover 9000 - it's a CrateMover 9001.

   The CrateMover 9001 is notable for many new and exciting features:
   air conditioning, leather seats, an extra cup holder, and the ability to pick up
   and move multiple crates at once.

   Again considering the example above, the crates begin in the same configuration:
    -------------
   |     [D]     |
   | [N] [C]     |
   | [Z] [M] [P] |
   |  1   2   3  |
    -------------
   Moving a single crate from stack 2 to stack 1 behaves the same as before:
    -------------
   | [D]         |
   | [N] [C]     |
   | [Z] [M] [P] |
   |  1   2   3  |
    -------------
   However, the action of moving three crates from stack 1 to stack 3 means that
   those three moved crates stay in the same order, resulting in this new configuration:
    ------------
   |        [D] |
   |        [N] |
   |    [C] [Z] |
   |    [M] [P] |
   | 1   2   3  |
    ------------
   Next, as both crates are moved from stack 2 to stack 1, they retain their order as well:
    -------------
   |         [D] |
   |         [N] |
   | [C]     [Z] |
   | [M]     [P] |
   |  1   2   3  |
    -------------
   Finally, a single crate is still moved from stack 1 to stack 2,
   but now it's crate C that gets moved:
    -------------
   |         [D] |
   |         [N] |
   |         [Z] |
   | [M] [C] [P] |
   |  1   2   3  |
    -------------
   In this example, the CrateMover 9001 has put the crates in a totally different order: MCD.

   Before the rearrangement process finishes, update your simulation so that the
   Elves know where they should stand to be ready to unload the final supplies.
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

    let result = supply_stacks_2(input);
    println!(
        "AoC:2022 • Day 5 • Supply Stacks (Part 2)\nResult: {}",
        result
    )
}

// Takes full parsed tuple input (first the default stack info and second instructions vector).
// Then runs through the instructions vector, and makes update to default stack appropriate
// to each line of instruction.
//
// When instructions are completed, takes the final top elements of modified stack and returns it
// as string.
//
// Same implementation as (Part 1) but reverse movable array functionality removed.
fn supply_stacks_2(input: (HashMap<i32, Vec<char>>, Vec<(i32, i32, i32)>)) -> String {
    let mut stack: HashMap<i32, Vec<char>> = input.clone().0.clone();
    for (mv, from, to) in input.1.iter() {
        let mut m = stack.get(from).unwrap().clone();

        let movable = m.split_off(m.len() - (*mv as usize));

        // Removed the reverse.

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
