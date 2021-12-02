use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Day1;

impl Day1 {
    pub fn run() {
        let count = count_depth_increase("./assets/day1.txt");
        print!("Number of depth increased: {}", count);
    }
}

fn count_depth_increase(filename: impl AsRef<Path>) -> i32 {
    let mut count: i32 = 0;
    if let Ok(lines) = read_lines(filename) {
        let mut last_line = "".to_string();
        for line in lines {
            if let Ok(depth) = line {
                if !last_line.is_empty() {
                    let last_depth: i32 = last_line.parse().unwrap();
                    let current_depth: i32 = depth.parse().unwrap();
                    if current_depth > last_depth {
                        count = count + 1;
                    }
                }
                last_line = depth;
            }
        }
    }
    count
}

#[test]
fn test_day1() {
    let count = count_depth_increase("./assets/day1_test.txt");
    assert_eq!(count, 7);
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
