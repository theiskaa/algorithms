// 2022 - Day 7 - No Space Left On Device (Part 2)

/* Problem Statement:
   Now, you're ready to choose a directory to delete.

   The total disk space available to the filesystem is 70000000. 
   To run the update, you need unused space of at least 30000000. 
   You need to find a directory you can delete that will free up enough space to run the update.

   In the example above, the total size of the outermost directory 
   (and thus the total amount of used space) is 48381165; this means that the size of the 
   unused space must currently be 21618835, which isn't quite the 30000000 required by the update. 
   Therefore, the update still requires a directory with total size of at least 8381165 to be 
   deleted before it can run.

   To achieve this, you have the following options:

    - Delete directory e, which would increase unused space by 584.
    - Delete directory a, which would increase unused space by 94853.
    - Delete directory d, which would increase unused space by 24933642.
    - Delete directory /, which would increase unused space by 48381165.
     
   Directories e and a are both too small; deleting them would not free up enough space. 
   However, directories d and / are both big enough! Between these, choose the smallest: d, 
   increasing unused space by 24933642.

   Find the smallest directory that, if deleted, would free up enough space on the filesystem 
   to run the update. What is the total size of that directory?
*/

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::collections::HashMap;
use std::iter::FromIterator;

#[allow(unused_variables, dead_code)]

fn main() {
    // Read input and parse to vector value.
    let file = File::open("input.txt").unwrap();
    let input = parse_input(file);

    let result = no_space_left_on_device_2(input);
    println!(
        "AoC:2022 • Day 7 • No Space Left On Device(Part 2)\nResult: {}",
        result
    )
}

//
fn no_space_left_on_device_2(input: Vec<String>) -> u32 {
    let mut sizes = HashMap::new();
    let mut trigged = Vec::new();

    for l in input.iter() {
        let part: Vec<_> = l.split_whitespace().collect();

        match part[..] {
            ["$", "cd", ".."] => {
                trigged.pop();
            }
            ["$", "cd", name] => {
                trigged.push(name);
            }
            [size, _name] => {
                let size: u32 = size.parse().unwrap();
                for idx in 0..trigged.len() {
                    let path = PathBuf::from_iter(&trigged[..=idx]);
                    *sizes.entry(path).or_insert(0) += size;
                }
            }
            _ => {}
        };
    }

    let disk = 70_000_000;
    let required = 30_000_000;

    let current = disk - sizes.get(&PathBuf::from("/")).unwrap();
    return sizes.into_values().filter(|s| current + s >= required).min().unwrap();
}

// Parses the [input.txt] file to the Vector of String(s).
// By ignoring the [ls] and [dir] commands.
fn parse_input(data: File) -> Vec<String> {
    let reader = BufReader::new(data);
    let mut res: Vec<String> = Vec::new();

    for line in reader.lines() {
        let l = line.unwrap();

        if l.starts_with("$ ls") || l.starts_with("dir") {
            continue;
        }

        res.push(l);
    }

    return res;
}
