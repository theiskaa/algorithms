// 2022 - Day 2 - Rock Paper Scissors

/* Problem Statement:
   The first column is what your opponent is going to play:
   A for Rock, B for Paper, and C for Scissors.

   The second column, you reason, must be what you should play in response:
   X for Rock, Y for Paper, and Z for Scissors.
   Winning every time would be suspicious,
   so the responses must have been carefully chosen.

   The winner of the whole tournament is the player with the highest score.
   Your total score is the sum of your scores for each round.
   The score for a single round is the score for the shape you selected
   (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the
   outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).

   For example, suppose you were given the following strategy guide:
   |A Y|
   |B X|
   |C Z|

   This strategy guide predicts and recommends the following:

   - In the first round, your opponent will choose Rock (A),
     and you should choose Paper (Y). This ends in a win for you with a
     score of 8 (2 because you chose Paper + 6 because you won).
   - In the second round, your opponent will choose Paper (B),
     and you should choose Rock (X). This ends in a loss for you with a
     score of 1 (1 + 0).
   - The third round is a draw with both players choosing Scissors,
     giving you a score of 3 + 3 = 6.

   In this example, if you were to follow the strategy guide,
   you would get a total score of 15 (8 + 1 + 6).
*/

use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(unused_variables, dead_code)]

fn main() {
    // Read input and parse to vector value.
    let file = File::open("input.txt").unwrap();
    let input = parse_input(file);

    let result = rock_paper_scissors(input);
    println!("AoC:2022 • Day 2 • Rock Paper Scissors\nResult: {}", result)
}

// Takes the parsed input as Vector of tuples of chars/
// Loops through it, and calculates each round's final result for human(not elf).
fn rock_paper_scissors(input: Vec<(char, char)>) -> i32 {
    let mut myscore: i32 = 0;

    for guide in input.iter() {
        let elf = guide.clone().0;
        let you = guide.clone().1;

        // The bonus score from you(r) action.
        // Where 'A' is Rock, 'B' is Paper, and 'C' is Scissors.
        let bonus = match you.clone() {
            'A' | 'X' => 1,
            'B' | 'Y' => 2,
            'C' | 'Z' => 3,
            _ => 0,
        };

        // Check for draw by converting Elf char to You char.
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

        let is_win = match guide.clone() {
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
