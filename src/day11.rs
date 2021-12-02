use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Day11;

impl Day11 {
    pub fn run() {
        print!("Day11 - part1: {}\n", day11_part1("./assets/day11.txt"));
        print!("Day11 - part2: {}\n", day11_part2("./assets/day11.txt"));
    }
}
#[derive(Clone)]
struct Octopus {
    energy: usize,
    flashed: bool,
}

impl Octopus {
    fn new(energy: usize) -> Octopus {
        return Octopus {
            energy: energy,
            flashed: false,
        };
    }

    fn tick(&mut self) -> bool {
        self.energy += 1;
        if self.energy > 9 && self.flashed == false {
            self.flashed = true;
            return true;
        }
        return false;
    }

    fn reset(&mut self) {
        if !self.flashed {
            return;
        }
        self.energy = 0;
        self.flashed = false;
    }
}

struct Grid {
    data: Vec<Vec<Octopus>>,
}

impl Grid {
    fn get_adjacent_octopus(&self, coord: (usize, usize)) -> Vec<(usize, usize)> {
        let mut output = Vec::<(usize, usize)>::new();

        //left
        if coord.0 > 0 {
            output.push((coord.0 - 1, coord.1));
        }
        // right
        if coord.0 < 9 {
            output.push((coord.0 + 1, coord.1));
        }
        // up
        if coord.1 > 0 {
            output.push((coord.0, coord.1 - 1));
        }
        // down
        if coord.1 < 9 {
            output.push((coord.0, coord.1 + 1));
        }

        // top left
        if coord.0 > 0 && coord.1 > 0 {
            output.push((coord.0 - 1, coord.1 - 1));
        }
        // top right
        if coord.0 < 9 && coord.1 > 0 {
            output.push((coord.0 + 1, coord.1 - 1));
        }
        // bottom left
        if coord.0 > 0 && coord.1 < 9 {
            output.push((coord.0 - 1, coord.1 + 1));
        }
        // bottom right
        if coord.0 < 9 && coord.1 < 9 {
            output.push((coord.0 + 1, coord.1 + 1));
        }

        output
    }

    fn simulate(&mut self) -> i32 {
        let mut total = 0;
        let mut next = Vec::<(usize, usize)>::new();
        for y in 0..10 {
            for x in 0..10 {
                next.push((x, y));
            }
        }
        loop {
            let current = next.clone();
            next = Vec::<(usize, usize)>::new();
            for (x, y) in current {
                if self.data[y][x].tick() {
                    total += 1;
                    next.extend(self.get_adjacent_octopus((x, y)));
                }
            }
            if next.len() == 0 {
                break;
            }
        }
        for y in &mut self.data {
            for x in y {
                x.reset();
            }
        }
        total
    }
}

fn load_input(filename: impl AsRef<Path>) -> Grid {
    let mut input = Vec::<Vec<Octopus>>::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(instruction) = line {
                let mut octoline = Vec::<Octopus>::new();
                for c in instruction.chars() {
                    octoline.push(Octopus::new(c.to_digit(10).unwrap() as usize));
                }
                input.push(octoline);
            }
        }
    }
    Grid { data: input }
}

fn day11_part1(filename: impl AsRef<Path>) -> i32 {
    let mut grid = load_input(filename);
    let mut total = 0;
    for _ in 0..100 {
        let step = grid.simulate();
        total += step;
    }
    return total;
}

fn day11_part2(filename: impl AsRef<Path>) -> i32 {
    let mut grid = load_input(filename);
    let mut step = 0;
    loop {
        step += 1;
        if grid.simulate() == 100 {
            return step;
        }
    }
}

#[test]
fn test_day11_part1() {
    assert_eq!(1656, day11_part1("./assets/day11_test.txt"));
}

#[test]
fn test_day11_part2() {
    assert_eq!(195, day11_part2("./assets/day11_test.txt"));
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
