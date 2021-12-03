use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Day2;

impl Day2 {
    pub fn run() {
        let (x, depth) = get_final_position("./assets/day2.txt");
        print!("Day2 - part1: {}\n", x * depth);

        let (x, depth) = get_final_position_with_aim("./assets/day2.txt");
        print!("Day2 - part2: {}\n", x * depth);
    }
}

fn get_final_position(filename: impl AsRef<Path>) -> (i32, i32) {
    let mut depth: i32 = 0;
    let mut x: i32 = 0;
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(instruction) = line {
                let parsed: Vec<&str> = instruction.split_whitespace().collect();
                match parsed[0] {
                    "forward" => x = x + parsed[1].parse::<i32>().unwrap(),
                    "down" => depth = depth + parsed[1].parse::<i32>().unwrap(),
                    "up" => depth = depth - parsed[1].parse::<i32>().unwrap(),
                    _ => panic!("Wrong instruction"),
                };
            }
        }
    }
    (x, depth)
}

fn get_final_position_with_aim(filename: impl AsRef<Path>) -> (i32, i32) {
    let mut depth: i32 = 0;
    let mut x: i32 = 0;
    let mut aim: i32 = 0;
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(instruction) = line {
                let parsed: Vec<&str> = instruction.split_whitespace().collect();
                match parsed[0] {
                    "forward" => {
                        let y = parsed[1].parse::<i32>().unwrap();
                        x = x + y;
                        depth = depth + aim * y;
                    }
                    "down" => aim = aim + parsed[1].parse::<i32>().unwrap(),
                    "up" => aim = aim - parsed[1].parse::<i32>().unwrap(),
                    _ => panic!("Wrong instruction"),
                };
            }
        }
    }
    (x, depth)
}

#[test]
fn test_day2() {
    let (x, depth) = get_final_position("./assets/day2_test.txt");
    assert_eq!(x * depth, 150);
}

#[test]
fn test_day2_part2() {
    let (x, depth) = get_final_position_with_aim("./assets/day2_test.txt");
    assert_eq!(x, 15);
    assert_eq!(depth, 60);
    assert_eq!(x * depth, 900);
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
