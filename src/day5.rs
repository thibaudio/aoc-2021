use regex::Regex;
use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Day5;

impl Day5 {
    pub fn run() {
        let vents = load_input("./assets/day5.txt");
        let mut grid = init_grid(vents.clone());
        grid.parse_h_and_v_lines(vents.clone());
        print!("Day5 part1: {}\n", grid.clone().compute_overlap());
        grid.parse_diag_lines(vents);
        print!("Day5 part2: {}\n", grid.compute_overlap());
    }
}

#[derive(Debug, Clone)]
struct Vent {
    starting_x: usize,
    starting_y: usize,
    ending_x: usize,
    ending_y: usize,
}

#[derive(Clone)]
struct GridPoint {
    count: i32,
}

impl Default for GridPoint {
    fn default() -> Self {
        GridPoint { count: 0 }
    }
}

#[derive(Clone)]
struct Grid {
    points: Vec<Vec<GridPoint>>,
}

impl Grid {
    fn parse_h_and_v_lines(&mut self, input: Vec<Vent>) {
        for i in input {
            if i.starting_y == i.ending_y {
                let start = cmp::min(i.starting_x, i.ending_x);
                let end = cmp::max(i.starting_x, i.ending_x);
                for h in start..=end {
                    self.points[h][i.starting_y].count = self.points[h][i.starting_y].count + 1;
                }
            }
            if i.starting_x == i.ending_x {
                let start = cmp::min(i.starting_y, i.ending_y);
                let end = cmp::max(i.starting_y, i.ending_y);
                for v in start..=end {
                    self.points[i.starting_x][v].count = self.points[i.starting_x][v].count + 1;
                }
            }
        }
    }

    fn parse_diag_lines(&mut self, input: Vec<Vent>) {
        for i in input {
            if i.starting_x != i.ending_x && i.starting_y != i.ending_y {
                let lenx = isize::abs(i.ending_x as isize - i.starting_x as isize);
                let dirx = (i.ending_x as isize - i.starting_x as isize) / lenx;

                let leny = isize::abs(i.ending_y as isize - i.starting_y as isize);
                let diry = (i.ending_y as isize - i.starting_y as isize) / leny;

                if lenx != leny {
                    panic!("Not 45deg. line");
                }

                for s in 0..=lenx {
                    self.points[(i.starting_x as isize + s as isize * dirx) as usize]
                        [(i.starting_y as isize + s as isize * diry) as usize]
                        .count = self.points[(i.starting_x as isize + s as isize * dirx) as usize]
                        [(i.starting_y as isize + s as isize * diry) as usize]
                        .count
                        + 1;
                }
            }
        }
    }

    fn compute_overlap(self) -> i32 {
        let mut count = 0;
        for i in self.points {
            for j in i {
                if j.count > 1 {
                    count = count + 1;
                }
            }
        }
        return count;
    }

    fn _draw(self) {
        for j in 0..self.points[0].len() {
            for i in 0..self.points.len() {
                print!("{} ", self.points[i][j].count);
            }
            print!("\n");
        }
    }
}

fn load_input(filename: impl AsRef<Path>) -> Vec<Vent> {
    let mut input: Vec<Vent> = Vec::new();
    let re: Regex = Regex::new(r"^([0-9]+),([0-9]+) -> ([0-9]+),([0-9]+)$").unwrap();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(instruction) = line {
                if let Some(cap) = re.captures(&instruction) {
                    let vent = Vent {
                        starting_x: cap[1].parse::<usize>().unwrap(),
                        starting_y: cap[2].parse::<usize>().unwrap(),
                        ending_x: cap[3].parse::<usize>().unwrap(),
                        ending_y: cap[4].parse::<usize>().unwrap(),
                    };
                    input.push(vent);
                }
            }
        }
    }
    return input;
}

fn init_grid(input: Vec<Vent>) -> Grid {
    let mut min_x: usize = 0;
    let mut min_y: usize = 0;
    let mut max_x: usize = 0;
    let mut max_y: usize = 0;
    for i in input {
        if i.starting_x < min_x {
            min_x = i.starting_x;
        } else if i.starting_x > max_x {
            max_x = i.starting_x
        }
        if i.ending_x < min_x {
            min_x = i.ending_x;
        } else if i.ending_x > max_x {
            max_x = i.ending_x
        }
        if i.starting_y < min_y {
            min_y = i.starting_y;
        } else if i.starting_y > max_y {
            max_y = i.starting_y
        }
        if i.ending_y < min_y {
            min_y = i.ending_y;
        } else if i.ending_y > max_y {
            max_y = i.ending_y
        }
    }
    let mut grid = Grid { points: Vec::new() };
    for _ in min_x..=max_x {
        let mut col: Vec<GridPoint> = Vec::new();
        for _ in min_y..=max_y {
            col.push(GridPoint::default())
        }
        grid.points.push(col);
    }
    return grid;
}

#[test]
fn test_day5_part1() {
    let vents = load_input("./assets/day5_test.txt");
    let mut grid = init_grid(vents.clone());
    grid.parse_h_and_v_lines(vents);
    assert_eq!(5, grid.compute_overlap());
}

#[test]
fn test_day5_part2() {
    let vents = load_input("./assets/day5_test.txt");
    let mut grid = init_grid(vents.clone());
    grid.parse_h_and_v_lines(vents.clone());
    grid.parse_diag_lines(vents);
    grid.clone()._draw();
    assert_eq!(12, grid.compute_overlap());
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
