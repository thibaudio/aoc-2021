use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Day1;

impl Day1 {
    pub fn run() {
        let count = count_depth_increase("./assets/day1.txt");
        print!("Number of depth increased: {}\n", count);

        let count_2 = count_depth_window_increase("./assets/day1.txt");
        print!("Number of sum depth increased: {}\n", count_2);
    }
}

fn count_depth_increase(filename: impl AsRef<Path>) -> i32 {
    if let Ok(lines) = read_lines(filename) {
        let mut depths = Vec::new();
        for line in lines {
            if let Ok(depth) = line {
                depths.push(depth.parse().unwrap());
            }
        }
        return count_int_increased(depths);
    }
    0
}

fn count_depth_window_increase(filename: impl AsRef<Path>) -> i32 {
    if let Ok(lines) = read_lines(filename) {
        let mut depths: Vec<i32> = Vec::new();
        for line in lines {
            if let Ok(depth) = line {
                depths.push(depth.parse().unwrap());
            }
        }
        let mut mean_depth = Vec::new();
        for i in 2..depths.len() {
            let sum = depths[i] + depths[i - 1] + depths[i - 2];
            mean_depth.push(sum);
        }
        return count_int_increased(mean_depth);
    }
    0
}

fn count_int_increased(ints: Vec<i32>) -> i32 {
    let mut count = 0;
    for i in 1..ints.len() {
        if ints[i - 1] < ints[i] {
            count = count + 1;
        }
    }
    count
}

#[test]
fn test_day1() {
    let count = count_depth_increase("./assets/day1_test.txt");
    assert_eq!(count, 7);
}

#[test]
fn test_day1_2() {
    let count = count_depth_window_increase("./assets/day1_test.txt");
    assert_eq!(count, 5);
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
