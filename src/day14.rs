use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Day14;

impl Day14 {
    pub fn run() {
        print!("Day14 - part1: {}\n", day14_part1("./assets/day14.txt"));
        print!("Day14 - part2: {}\n", day14_part2("./assets/day14.txt"));
    }
}

fn load_input(filename: impl AsRef<Path>) -> (String, HashMap<String, String>) {
    let mut start = String::from("");
    let mut rules = HashMap::<String, String>::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(instruction) = line {
                if instruction.is_empty() {
                    continue;
                }
                if instruction.contains("->") {
                    let mut split = instruction.split(" -> ");
                    let key = split.next().unwrap();
                    let value = split.next().unwrap();
                    rules.insert(key.to_string(), value.to_string());
                } else {
                    start = instruction;
                }
            }
        }
    }
    (start, rules)
}

fn grow(start: String, rules: &HashMap<String, String>) -> String {
    let chars = start.chars().collect::<Vec<char>>();
    let mut output = String::from(chars[0]);
    for i in 0..chars.len() - 1 {
        let key = chars[i].to_string() + &chars[i + 1].to_string();
        if rules.contains_key(&key) {
            output = output + &rules[&key];
        }
        output = output + &chars[i + 1].to_string();
    }
    output
}

fn count(start: String) -> i128 {
    let mut count = HashMap::<String, i128>::new();
    for i in start.chars() {
        if count.contains_key(&i.to_string()) {
            *count.get_mut(&i.to_string()).unwrap() = count[&i.to_string()] + 1;
        } else {
            count.insert(i.to_string(), 1);
        }
    }
    let mut max_v: i128 = 0;
    let mut min_v = i128::MAX;
    for (_, v) in count.iter() {
        if v > &max_v {
            max_v = *v;
        }
        if v < &min_v {
            min_v = *v;
        }
    }
    return max_v - min_v;
}

fn day14_part1(filename: impl AsRef<Path>) -> i128 {
    let (mut start, rules) = load_input(filename);
    for _ in 0..10 {
        start = grow(start, &rules);
        // print!("{}\n", start);
    }
    return count(start);
}

fn day14_part2(filename: impl AsRef<Path>) -> i128 {
    let (mut start, rules) = load_input(filename);
    for _ in 0..40 {
        start = grow(start, &rules);
        // print!("{}\n", start);
    }
    return count(start);
}

#[test]
fn test_day14_part1() {
    assert_eq!(1588, day14_part1("./assets/day14_test.txt"));
}

#[test]
fn test_day14_part2() {
    assert_eq!(2188189693529, day14_part2("./assets/day14_test.txt"));
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
