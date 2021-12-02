use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Day4;

impl Day4 {
    pub fn run() {
        let (numbers, boards) = load_input("./assets/day4.txt");
        print!(
            "Day4_part1: {}\n",
            find_winner(numbers.clone(), boards.clone())
        );
        print!("Day4_part2: {}\n", find_last_winner(numbers, boards));
    }
}

#[derive(Clone)]
struct BoardNumber {
    number: i32,
    marked: bool,
}

impl BoardNumber {
    fn new(number: i32) -> BoardNumber {
        BoardNumber {
            number: number,
            marked: false,
        }
    }
}

#[derive(Clone)]
struct Board {
    data: Vec<Vec<BoardNumber>>,
}

impl Board {
    fn mark(&mut self, number: i32) {
        for i in self.data.iter_mut() {
            for j in i {
                if j.number == number {
                    j.marked = true;
                }
            }
        }
    }

    fn check_winner(&self) -> bool {
        for i in self.data.iter() {
            let mut allgood = true;
            for j in i {
                if !j.marked {
                    allgood = false;
                    break;
                }
            }
            if allgood {
                return true;
            }
        }
        for j in 0..self.data[0].len() {
            let mut allgood = true;
            for i in self.data.iter() {
                if !i[j].marked {
                    allgood = false;
                }
            }
            if allgood {
                return true;
            }
        }
        return false;
    }

    fn compute_score(&self) -> i32 {
        let mut score = 0;
        for i in self.data.iter() {
            for j in i {
                if !j.marked {
                    score = score + j.number;
                }
            }
        }
        return score;
    }
}

fn load_input(filename: impl AsRef<Path>) -> (Vec<i32>, Vec<Board>) {
    let mut parsed_lines: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(instruction) = line {
                parsed_lines.push(instruction);
            }
        }
    }
    let numbers: Vec<i32> = parsed_lines[0]
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    parsed_lines.remove(0);
    return (numbers, parse_boards(parsed_lines));
}

fn parse_boards(input: Vec<String>) -> Vec<Board> {
    let mut output: Vec<Board> = Vec::new();
    let mut current_board = Board { data: Vec::new() };
    for line in input {
        if line.is_empty() {
            if current_board.data.len() > 0 {
                output.push(current_board);
                current_board = Board { data: Vec::new() };
            }
            continue;
        }
        let parsed_line: Vec<BoardNumber> = line
            .split_whitespace()
            .map(|x| BoardNumber::new(x.parse::<i32>().unwrap()))
            .collect();
        current_board.data.push(parsed_line);
    }
    if current_board.data.len() > 0 {
        output.push(current_board);
    }
    return output;
}

fn find_winner(input: Vec<i32>, mut boards: Vec<Board>) -> i32 {
    for number in input {
        for board in boards.iter_mut() {
            board.mark(number);
            if board.check_winner() {
                return board.compute_score() * number;
            }
        }
    }
    return 0;
}

fn find_last_winner(input: Vec<i32>, mut boards: Vec<Board>) -> i32 {
    for number in input {
        for board in boards.iter_mut() {
            board.mark(number);
        }
        if boards.len() > 1 {
            boards.retain(|x| !x.check_winner());
        } else {
            if boards[0].check_winner() {
                return boards[0].compute_score() * number;
            }
        }
    }
    return 0;
}

#[test]
fn test_day4_part1() {
    let (numbers, boards) = load_input("./assets/day4_test.txt");
    assert_eq!(4512, find_winner(numbers, boards));
}

#[test]
fn test_day4_part2() {
    let (numbers, boards) = load_input("./assets/day4_test.txt");
    assert_eq!(1924, find_last_winner(numbers, boards));
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
