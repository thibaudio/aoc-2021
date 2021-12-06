use regex::Regex;
use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Day6;

impl Day6 {
    pub fn run() {
        let mut school = load_input("./assets/day6.txt");
        for _ in 0..80 {
            simulate(&mut school);
        }
        print!("Day6 - Part1: {}\n", school.len());
    }
}

#[derive(Clone, Copy)]
struct LanternFish {
    timer: usize,
}

impl LanternFish {
    fn tick(&mut self) -> Option<LanternFish> {
        if self.timer == 0 {
            let new_fish = LanternFish { timer: 8 };
            self.timer = 6;
            return Some(new_fish);
        } else {
            self.timer = self.timer - 1;
            return None;
        }
    }
}

fn load_input(filename: impl AsRef<Path>) -> Vec<LanternFish> {
    let mut input: Vec<LanternFish> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(instruction) = line {
                let fishes = instruction.split(',');
                for f in fishes {
                    let new_fish = LanternFish {
                        timer: f.parse().unwrap(),
                    };
                    input.push(new_fish);
                }
            }
        }
    }
    return input;
}

fn simulate(school: &mut Vec<LanternFish>) {
    let mut to_add: Vec<LanternFish> = Vec::new();
    for f in school.iter_mut() {
        match f.tick() {
            Some(new_fish) => to_add.push(new_fish),
            None => (),
        };
    }
    school.extend(to_add);
}

#[test]
fn test_day6_part1() {
    let mut school = load_input("./assets/day6_test.txt");
    for _ in 0..78 {
        simulate(&mut school);
    }
    print!("{}", school.len());
    assert_eq!(5934, school.len());
}

#[test]
fn test_day6_part2() {
    let mut school = load_input("./assets/day6_test.txt");
    for _ in 0..256 {
        simulate(&mut school);
    }
    assert_eq!(26984457539, school.len());
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
