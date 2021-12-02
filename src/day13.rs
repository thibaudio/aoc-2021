use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Day13;

impl Day13 {
    pub fn run() {
        print!("Day13 - part1: {}\n", day13_part1("./assets/day13.txt"));
        print!("Day13 - part2: {}\n", day13_part2("./assets/day13.txt"));
    }
}

fn load_input(filename: impl AsRef<Path>) -> (Vec<(usize, usize)>, VecDeque<(usize, usize)>) {
    let mut fold_instruction = VecDeque::<(usize, usize)>::new();
    let mut initial_dots = Vec::<(usize, usize)>::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(instruction) = line {
                if instruction.is_empty() {
                    continue;
                }
                if instruction.starts_with("fold along") {
                    let mut fold = instruction.split('=');
                    fold.next();
                    if instruction.contains("x=") {
                        fold_instruction
                            .push_back((fold.next().unwrap().parse::<usize>().unwrap(), 0));
                    } else {
                        fold_instruction
                            .push_back((0, fold.next().unwrap().parse::<usize>().unwrap()));
                    }
                } else {
                    let mut coord = instruction.split(',');
                    let x = coord.next().unwrap().parse::<usize>().unwrap();
                    let y = coord.next().unwrap().parse::<usize>().unwrap();
                    initial_dots.push((x, y));
                }
            }
        }
    }
    (initial_dots, fold_instruction)
}

fn fold(dots: &mut Vec<(usize, usize)>, folds: &mut VecDeque<(usize, usize)>) {
    let fold = folds.pop_front().unwrap();
    *dots = dots
        .iter()
        .map(|x| {
            if x.0 > fold.0 && fold.0 > 0 {
                (2 * fold.0 - x.0, x.1)
            } else if x.1 > fold.1 && fold.1 > 0 {
                (x.0, 2 * fold.1 - x.1)
            } else {
                (x.0, x.1)
            }
        })
        .collect::<Vec<(usize, usize)>>();
    dots.sort();
    dots.dedup();
}

fn print(dots: &Vec<(usize, usize)>) {
    let mut xmax = 0;
    let mut ymax = 0;
    for i in dots {
        if i.0 > xmax {
            xmax = i.0
        }
        if i.1 > ymax {
            ymax = i.1
        }
    }
    let max = xmax.max(ymax) + 1;

    let mut grid = Vec::<Vec<char>>::new();

    for _ in 0..max {
        let mut grid_line = Vec::<char>::new();
        for _ in 0..max {
            grid_line.push('.');
        }
        grid.push(grid_line);
    }
    for i in dots {
        grid[i.0][i.1] = '#';
    }
    for y in 0..max {
        for x in 0..max {
            print! {"{} ", grid[x][y]};
        }
        print!("\n");
    }
    print!("\n");
}

fn day13_part1(filename: impl AsRef<Path>) -> i32 {
    let (mut dots, mut folds) = load_input(filename);
    fold(&mut dots, &mut folds);
    return dots.len() as i32;
}

fn day13_part2(filename: impl AsRef<Path>) -> i32 {
    let (mut dots, mut folds) = load_input(filename);
    while folds.len() > 0 {
        fold(&mut dots, &mut folds);
    }
    print(&dots);
    return dots.len() as i32;
}

#[test]
fn test_day13_part1() {
    assert_eq!(17, day13_part1("./assets/day13_test.txt"));
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
