// 2022 - Day 2 - Rock Paper Scissors (Part Two)

/* Problem Statement:
   The Elf finishes helping with the tent and sneaks back over to you.
   "Anyway, the second column says how the round needs to end:
   X means you need to lose, Y means you need to end the round in a draw,
   and Z means you need to win. Good luck!"

   The total score is still calculated in the same way, but now you need
   to figure out what shape to choose so the round ends as indicated.
   The example above now goes like this:

   - In the first round, your opponent will choose Rock (A),
     and you need the round to end in a draw (Y), so you also choose Rock.
     This gives you a score of 1 + 3 = 4.
   - In the second round, your opponent will choose Paper (B),
     and you choose Rock so you lose (X) with a score of 1 + 0 = 1.
   - In the third round, you will defeat your opponent's Scissors with Rock
     for a score of 1 + 6 = 7.

   Now that you're correctly decrypting the ultra top secret strategy guide,
   you would get a total score of 12.

   Following the Elf's instructions for the second column,
   what would your total score be if everything goes exactly according to your strategy guide?
*/

use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(unused_variables, dead_code)]

fn main() {
    // Read input and parse to vector value.
    let file = File::open("input.txt").unwrap();
    let input = parse_input(file);

    let result = rock_paper_scissors_2(input);
    println!(
        "AoC:2022 • Day 2 • Rock Paper Scissors (Part 2) \nResult: {}",
        result
    )
}

// Takes the parsed input as Vector of tuples of chars/
// Loops through it, and calculates each round's final result for human(not elf).
fn rock_paper_scissors_2(input: Vec<(char, char)>) -> i32 {
    let mut myscore: i32 = 0;

    for guide in input.iter() {
        let elf = guide.clone().0;
        let you = convert_to(guide.clone().1, elf);

        // The bonus score from you(r) action.
        let bonus = match you {
            'X' => 1,
            'Y' => 2,
            'Z' => 3,
            _ => 0,
        };

        // Check for draw by converting Elf char to converted You char.
        let is_draw = match elf {
            'A' => 'X',
            'B' => 'Y',
            'C' => 'Z',
            _ => ' ',
        } == you;

        if is_draw {
            myscore += 3 + bonus.clone();
            continue;
        }

        let is_win = match (elf, you) {
            ('A', 'Y') => true,  // Rock vs Paper
            ('A', 'Z') => false, // Rock vs Scissors
            ('B', 'X') => false, // Paper vs Rock
            ('B', 'Z') => true,  // Paper vs Scissors
            ('C', 'Y') => false, // Scissors vs Paper
            ('C', 'X') => true,  // Scissors vs Rock
            _ => false,
        };

        let score = if is_win { 6 } else { 0 };
        myscore += score + bonus.clone();
    }

    return myscore;
}

// Converts the Elf char to actual act-appropriate YOU char.
fn convert_to(act: char, elf: char) -> char {
    match act {
        'X' => match elf {
            'A' => 'Z', // Rock wins to Scissors
            'B' => 'X', // Paper wins to Rock
            'C' => 'Y', // Scissors wins to Paper
            _ => ' ',
        },
        'Y' => match elf {
            'A' => 'X', // Rock
            'B' => 'Y', // Paper
            'C' => 'Z', // Scissors
            _ => ' ',
        },
        'Z' => match elf {
            'A' => 'Y', // Rock loses to Paper
            'B' => 'Z', // Paper loses to Scissors
            'C' => 'X', // Scissors loses to Rock
            _ => ' ',
        },
        _ => ' ',
    }
}

// Parses the [input.txt] file to the Vector of string tuple.
fn parse_input(data: File) -> Vec<(char, char)> {
    let reader = BufReader::new(data);

    let mut res: Vec<(char, char)> = Vec::new();
    for line in reader.lines() {
        let l = line.unwrap();
        let values: Vec<&str> = l.split_whitespace().collect();
        let tuple = (
            values[0].chars().next().unwrap(),
            values[1].chars().next().unwrap(),
        );

        res.push(tuple);
    }

    return res;
}
