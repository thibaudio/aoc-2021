use bimap::BiMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Day8;

impl Day8 {
    pub fn run() {
        let patterns = load_input("./assets/day8.txt");
        let mut count = 0;
        for p in patterns {
            let mut d = Decoder::new();
            d.analyse(p.signal);
            count += d.decode(p.display);
        }
        print!("Day8 part2: {}\n", count);
    }
}

fn letter_number() -> BiMap<usize, String> {
    let mut map: BiMap<usize, String> = BiMap::new();
    map.insert(0, String::from("abcefg"));
    map.insert(1, String::from("cf"));
    map.insert(2, String::from("acdeg"));
    map.insert(3, String::from("acdfg"));
    map.insert(4, String::from("bcdf"));
    map.insert(5, String::from("abdfg"));
    map.insert(6, String::from("abdefg"));
    map.insert(7, String::from("acf"));
    map.insert(8, String::from("abcdefg"));
    map.insert(9, String::from("abcdfg"));
    map
}

struct Decoder {
    ln: BiMap<usize, String>,

    // encrypted,decrypted
    code: BiMap<String, String>,
}

impl Decoder {
    fn new() -> Decoder {
        return Decoder {
            code: BiMap::new(),
            ln: letter_number(),
        };
    }

    fn decode(self, encrypted: Vec<String>) -> i32 {
        let mut rs = String::from("");
        for i in encrypted {
            let (_, sorted_string) = a_string(i);
            rs.push_str(
                &self
                    .ln
                    .get_by_right(self.code.get_by_left(&sorted_string).unwrap())
                    .unwrap()
                    .to_string(),
            );
        }
        return rs.parse().unwrap();
    }

    fn analyse(&mut self, encrypted: Vec<String>) {
        // 1st round
        let mut ttf: Vec<String> = Vec::new(); //235
        let mut zsn: Vec<String> = Vec::new(); //069
        for i in encrypted {
            let (count, sorted_string) = a_string(i);
            match count {
                2 => {
                    self.code
                        .insert(sorted_string, self.ln.get_by_left(&1).unwrap().clone());
                }
                3 => {
                    self.code
                        .insert(sorted_string, self.ln.get_by_left(&7).unwrap().clone());
                }
                4 => {
                    self.code
                        .insert(sorted_string, self.ln.get_by_left(&4).unwrap().clone());
                }
                7 => {
                    self.code
                        .insert(sorted_string, self.ln.get_by_left(&8).unwrap().clone());
                }
                5 => {
                    ttf.push(sorted_string);
                }
                6 => {
                    zsn.push(sorted_string);
                }
                _ => (),
            };
        }

        // Find 9
        for i in zsn {
            if common_char(
                self.code
                    .get_by_right(self.ln.get_by_left(&4).unwrap())
                    .unwrap(),
                &i,
            ) == 4
            {
                self.code
                    .insert(i, self.ln.get_by_left(&9).unwrap().to_string());
            } else if common_char(
                self.code
                    .get_by_right(self.ln.get_by_left(&1).unwrap())
                    .unwrap(),
                &i,
            ) == 2
            {
                self.code
                    .insert(i, self.ln.get_by_left(&0).unwrap().to_string());
            } else {
                self.code
                    .insert(i, self.ln.get_by_left(&6).unwrap().to_string());
            }
        }
        // Find 2, 3, 5
        for i in ttf {
            if contains_char(
                &i,
                self.code
                    .get_by_right(self.ln.get_by_left(&1).unwrap())
                    .unwrap()
                    .chars()
                    .collect(),
            ) {
                self.code
                    .insert(i, self.ln.get_by_left(&3).unwrap().to_string());
            } else if common_char(
                self.code
                    .get_by_right(self.ln.get_by_left(&9).unwrap())
                    .unwrap(),
                &i,
            ) == 5
            {
                self.code
                    .insert(i, self.ln.get_by_left(&5).unwrap().to_string());
            } else {
                self.code
                    .insert(i, self.ln.get_by_left(&2).unwrap().to_string());
            }
        }
    }
}

fn a_string(string: String) -> (usize, String) {
    let mut chars = string.chars().collect::<Vec<char>>();
    chars.sort_by(|a, b| b.cmp(a));
    return (chars.len(), chars.into_iter().collect::<String>());
}

fn common_char(a: &String, b: &String) -> usize {
    let mut count = 0;
    for l in a.chars() {
        if b.contains(l) {
            count += 1;
        }
    }
    count
}

fn contains_char(a: &String, b: Vec<char>) -> bool {
    for i in b {
        if !a.contains(i) {
            return false;
        }
    }
    return true;
}

struct Pattern {
    signal: Vec<String>,
    display: Vec<String>,
}

impl Pattern {
    fn default() -> Pattern {
        Pattern {
            signal: Vec::new(),
            display: Vec::new(),
        }
    }
}

fn load_input(filename: impl AsRef<Path>) -> Vec<Pattern> {
    let mut input: Vec<Pattern> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(instruction) = line {
                let mut pattern = Pattern::default();
                let mut it = instruction.split('|');
                let signals = it.next().unwrap().split_whitespace();
                for signal in signals {
                    pattern.signal.push(signal.to_string());
                }
                let displays = it.next().unwrap().split_whitespace();
                for display in displays {
                    pattern.display.push(display.to_string());
                }
                input.push(pattern);
            }
        }
    }
    return input;
}

#[test]
fn test_day8_part2() {
    let patterns = load_input("./assets/day8_test.txt");
    let mut count = 0;
    for p in patterns {
        let mut d = Decoder::new();
        d.analyse(p.signal);
        count += d.decode(p.display);
    }
    assert_eq!(61229, count);
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
