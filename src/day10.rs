use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Day10;

impl Day10 {
    pub fn run() {
        print!("Day10 - part1: {}\n", day10_part1("./assets/day10.txt"));
        print!("Day10 - part2: {}\n", day10_part2("./assets/day10.txt"));
    }
}

fn load_input(filename: impl AsRef<Path>) -> Vec<String> {
    let mut input = Vec::<String>::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(instruction) = line {
                input.push(instruction);
            }
        }
    }
    input
}

fn find_first_illegal_character(input: &String) -> i32 {
    let mut open_chars = Vec::<char>::new();
    for c in input.chars() {
        match c {
            '(' | '[' | '{' | '<' => open_chars.push(c),
            ')' => {
                if open_chars.pop() != Some('(') {
                    return 3;
                }
            }

            ']' => {
                if open_chars.pop() != Some('[') {
                    return 57;
                }
            }

            '}' => {
                if open_chars.pop() != Some('{') {
                    return 1197;
                }
            }

            '>' => {
                if open_chars.pop() != Some('<') {
                    return 25137;
                }
            }
            _ => {
                panic!["Unrecognized character: {}", c];
            }
        }
    }
    0
}

fn complete_sequence(input: &String) -> u128 {
    let mut open_chars = Vec::<char>::new();
    for c in input.chars() {
        match c {
            '(' | '[' | '{' | '<' => open_chars.push(c),
            ')' => {
                if open_chars.pop() != Some('(') {
                    panic!["Unrecognized character: {}", c];
                }
            }

            ']' => {
                if open_chars.pop() != Some('[') {
                    panic!["Unrecognized character: {}", c];
                }
            }

            '}' => {
                if open_chars.pop() != Some('{') {
                    panic!["Unrecognized character: {}", c];
                }
            }

            '>' => {
                if open_chars.pop() != Some('<') {
                    panic!["Unrecognized character: {}", c];
                }
            }
            _ => {
                panic!["Unrecognized character: {}", c];
            }
        }
    }
    let mut score: u128 = 0;
    for c in open_chars.iter().rev() {
        match c {
            '(' => score = score * 5 + 1,
            '[' => score = score * 5 + 2,
            '{' => score = score * 5 + 3,
            '<' => score = score * 5 + 4,
            _ => panic!["Nop"],
        }
    }
    score
}

fn day10_part1(filename: impl AsRef<Path>) -> i32 {
    let instructions = load_input(filename);
    return instructions
        .iter()
        .map(|x| find_first_illegal_character(x))
        .sum();
}

fn day10_part2(filename: impl AsRef<Path>) -> u128 {
    let mut instructions = load_input(filename);
    instructions.retain(|x| find_first_illegal_character(x) == 0);
    let mut scores: Vec<u128> = instructions.iter().map(|x| complete_sequence(x)).collect();
    scores.sort();
    return scores[(scores.len() - 1) / 2];
}

#[test]
fn test_day10_part1() {
    assert_eq!(26397, day10_part1("./assets/day10_test.txt"));
}

#[test]
fn test_day10_part2() {
    assert_eq!(288957, day10_part2("./assets/day10_test.txt"));
}
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
