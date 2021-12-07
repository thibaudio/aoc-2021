use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Day3;

impl Day3 {
    pub fn run() {
        let input = load_input("./assets/day3.txt");
        let x = process_power_consumption(get_gamma_epsilon(input.clone()));
        print!("Day3 - part1: {}\n", x);

        let oxygen = process_oxygen(input.clone());
        let co2 = process_co2(input.clone());
        print!("Day3 - part2: {}\n", process_tuple((oxygen, co2)));
    }
}

struct GammaEpsilon {
    gamma: String,
    epsilon: String,
}

fn get_gamma_epsilon(parsed_lines: Vec<Vec<bool>>) -> GammaEpsilon {
    let mut ge = GammaEpsilon {
        gamma: String::from(""),
        epsilon: String::from(""),
    };
    for i in 0..parsed_lines[0].len() {
        let mut count_0 = 0;
        let mut count_1 = 0;
        for j in 0..parsed_lines.len() {
            if parsed_lines[j][i] {
                count_1 = count_1 + 1;
            } else {
                count_0 = count_0 + 1;
            }
        }
        if count_0 > count_1 {
            ge.gamma.push_str("0");
            ge.epsilon.push_str("1");
        } else if count_1 > count_0 {
            ge.gamma.push_str("1");
            ge.epsilon.push_str("0");
        } else {
            ge.gamma.push_str("1");
            ge.epsilon.push_str("0");
        }
    }
    return ge;
}

fn load_input(filename: impl AsRef<Path>) -> Vec<Vec<bool>> {
    let mut parsed_lines: Vec<Vec<bool>> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(instruction) = line {
                let mut parsed_line: Vec<bool> = Vec::new();
                for c in instruction.chars() {
                    match c {
                        '0' => parsed_line.push(false),
                        '1' => parsed_line.push(true),
                        _ => panic!("Waiting for 0 or 1"),
                    };
                }
                parsed_lines.push(parsed_line);
            }
        }
    }
    return parsed_lines;
}

fn process_power_consumption(ge: GammaEpsilon) -> i32 {
    let gamma_rate = i32::from_str_radix(&*ge.gamma, 2).unwrap();
    let epsilon_rate = i32::from_str_radix(&*ge.epsilon, 2).unwrap();
    return gamma_rate * epsilon_rate;
}

fn process_tuple(tuple: (String, String)) -> i32 {
    let left = i32::from_str_radix(&*tuple.0, 2).unwrap();
    let right = i32::from_str_radix(&*tuple.1, 2).unwrap();
    return left * right;
}

fn process_oxygen(mut parsed_lines: Vec<Vec<bool>>) -> String {
    for i in 0..parsed_lines[0].len() {
        let mut count_0 = 0;
        let mut count_1 = 0;
        for j in 0..parsed_lines.len() {
            if parsed_lines[j][i] {
                count_1 = count_1 + 1;
            } else {
                count_0 = count_0 + 1;
            }
        }
        if count_1 >= count_0 {
            parsed_lines.retain(|x| x[i] == true);
        } else {
            parsed_lines.retain(|x| x[i] == false);
        }
        if parsed_lines.len() == 1 {
            break;
        }
    }
    return from_vec_to_string(parsed_lines[0].clone());
}

fn process_co2(mut parsed_lines: Vec<Vec<bool>>) -> String {
    for i in 0..parsed_lines[0].len() {
        let mut count_0 = 0;
        let mut count_1 = 0;
        for j in 0..parsed_lines.len() {
            if parsed_lines[j][i] {
                count_1 = count_1 + 1;
            } else {
                count_0 = count_0 + 1;
            }
        }
        if count_0 <= count_1 {
            parsed_lines.retain(|x| x[i] == false);
        } else {
            parsed_lines.retain(|x| x[i] == true);
        }
        if parsed_lines.len() == 1 {
            break;
        }
    }
    return from_vec_to_string(parsed_lines[0].clone());
}

fn from_vec_to_string(input: Vec<bool>) -> String {
    let mut out = String::from("");
    for i in 0..input.len() {
        match input[i] {
            true => out.push_str("1"),
            false => out.push_str("0"),
        }
    }
    return out;
}

fn _print_vect(input: Vec<Vec<bool>>) {
    for i in 0..input.len() {
        print!("{}\n", from_vec_to_string(input[i].clone()));
    }
    print!("-----------------\n");
}

#[test]
fn test_day3_part1() {
    let x = process_power_consumption(get_gamma_epsilon(load_input("./assets/day3_test.txt")));
    assert_eq!(x, 198);
}

#[test]
fn test_day3_part2() {
    let input = load_input("./assets/day3_test.txt");
    let oxygen = process_oxygen(input.clone());
    let co2 = process_co2(input.clone());
    //let x = process_tuple(tuple.clone());
    assert_eq!(oxygen, "10111");
    assert_eq!(co2, "01010");
    assert_eq!(process_tuple((oxygen, co2)), 230);
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
