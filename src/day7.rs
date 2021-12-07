use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Day7;

impl Day7 {
    pub fn run() {
        let crabs = load_input("./assets/day7.txt");
        let total = process_1(crabs.clone());
        print!("Day7 - Part1: {}\n", total);

        let total = process_2(crabs);
        print!("Day7 - Part2: {}\n", total);
    }
}

fn load_input(filename: impl AsRef<Path>) -> Vec<i32> {
    let mut input: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(instruction) = line {
                let numbers = instruction.split(',');
                for f in numbers {
                    input.push(f.parse::<i32>().unwrap());
                }
            }
        }
    }
    return input;
}

fn process_1(input: Vec<i32>) -> i32 {
    let mean: i32 = input.iter().sum::<i32>() / input.len() as i32;
    let mut cost: i32 = input.iter().map(|&x| (x - mean).abs()).sum();
    for i in 0..mean {
        cost = cost.min(input.iter().map(|&x| (x - (mean + i as i32)).abs()).sum());
        cost = cost.min(input.iter().map(|&x| (x - (mean - i as i32)).abs()).sum());
    }
    return cost;
}

fn process_2(input: Vec<i32>) -> i32 {
    let mean: i32 = input.iter().sum::<i32>() / input.len() as i32;
    let mut cost: i32 = input.iter().map(|&x| new_cost((x - mean).abs())).sum();
    for i in 0..mean {
        cost = cost.min(
            input
                .iter()
                .map(|&x| new_cost((x - (mean + i as i32)).abs()))
                .sum(),
        );
        cost = cost.min(
            input
                .iter()
                .map(|&x| new_cost((x - (mean - i as i32)).abs()))
                .sum(),
        );
    }
    return cost;
}

fn new_cost(m: i32) -> i32 {
    (m + 1) * m / 2
}

#[test]
fn test_day7_part1() {
    let crabs = load_input("./assets/day7_test.txt");
    print!("{:?}", crabs);
    let total = process_1(crabs);
    assert_eq!(37, total);
}

#[test]
fn test_day7_part2() {
    let crabs = load_input("./assets/day7_test.txt");
    print!("{:?}", crabs);
    let total = process_2(crabs);
    assert_eq!(168, total);
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
