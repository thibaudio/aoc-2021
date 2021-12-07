use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Day6;

impl Day6 {
    pub fn run() {
        let school = load_input("./assets/day6.txt");
        print!("Day6 part1: {}\n", process(school.clone(), 80));
        print!("Day6 part2: {}\n", process(school, 256));
    }
}

fn load_input(filename: impl AsRef<Path>) -> Vec<u128> {
    let mut input = vec![0, 0, 0, 0, 0, 0, 0];
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(instruction) = line {
                let fishes = instruction.split(',');
                for f in fishes {
                    let index = f.parse::<usize>().unwrap();
                    input[index] = input[index] + 1;
                }
            }
        }
    }
    return input;
}

fn process(mut school: Vec<u128>, days: usize) -> u128 {
    let mut delayed = vec![0, 0, 0, 0, 0, 0, 0];
    for d in 0..days {
        let mod_day = d % 7;
        school[((d + 2) % 7)] = school[((d + 2) % 7)] + delayed[mod_day];
        delayed[mod_day] = school[mod_day];
    }
    let mut count = 0;
    for i in 0..school.len() {
        count = count + school[i] + delayed[i];
    }
    return count;
}

#[test]
fn test_day6_part1() {
    let school = load_input("./assets/day6_test.txt");
    print!("{:?}", school);
    let total = process(school, 80);
    assert_eq!(5934, total);
}

#[test]
fn test_day6_part2() {
    let school = load_input("./assets/day6_test.txt");
    print!("{:?}", school);
    let total = process(school, 256);
    assert_eq!(26984457539, total);
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
