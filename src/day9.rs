use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Day9;

impl Day9 {
    pub fn run() {
        let grid = load_input("./assets/day9.txt");
        let mut count = 0;
        let mut basins = Vec::<Vec<(usize, usize)>>::new();
        for y in 0..grid.ysize {
            for x in 0..grid.xsize {
                if grid.is_low_point(x, y) {
                    count += grid.data[y][x] + 1;
                    basins.push(grid.find_basin((x, y)));
                }
            }
        }

        print!("Day9 - part1: {}\n", count);
        basins.sort_by(|b, a| a.len().cmp(&b.len()));
        print!(
            "Day9 - part2: {}\n",
            basins[0].len() * basins[1].len() * basins[2].len()
        );
    }
}

struct Grid {
    //y, x
    data: Vec<Vec<usize>>,
    xsize: usize,
    ysize: usize,
}

impl Grid {
    fn new(grid_data: Vec<Vec<usize>>) -> Grid {
        let ysize = grid_data.len();
        let xsize = grid_data[0].len();
        Grid {
            data: grid_data,
            xsize: xsize,
            ysize: ysize,
        }
    }
    fn get_adjacents_points(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut ap = Vec::<(usize, usize)>::new();

        //Store left
        if x > 0 {
            ap.push((x - 1, y));
        }
        // Right
        if x < self.xsize - 1 {
            ap.push((x + 1, y));
        }
        //Up
        if y > 0 {
            ap.push((x, y - 1));
        }
        //Down
        if y < self.ysize - 1 {
            ap.push((x, y + 1));
        }
        ap
    }

    fn is_low_point(&self, x: usize, y: usize) -> bool {
        let current_height = self.data[y][x];
        for (i, j) in self.get_adjacents_points(x, y) {
            if current_height >= self.data[j][i] {
                return false;
            }
        }
        return true;
    }

    fn find_basin(&self, lowest_point: (usize, usize)) -> Vec<(usize, usize)> {
        let mut output = vec![lowest_point];
        let mut index = 0;
        loop {
            let parent = output[index];
            for (i, j) in self.get_adjacents_points(parent.0, parent.1) {
                if !output.contains(&(i, j)) && self.data[j][i] < 9 {
                    output.push((i, j));
                }
            }
            if index >= output.len() - 1 {
                break;
            } else {
                index += 1;
            }
        }
        output
    }
}

fn load_input(filename: impl AsRef<Path>) -> Grid {
    let mut input = Vec::<Vec<usize>>::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(instruction) = line {
                let mut grid_line = Vec::<usize>::new();
                for c in instruction.chars() {
                    grid_line.push(c.to_digit(10).unwrap() as usize);
                }
                input.push(grid_line);
            }
        }
    }
    return Grid::new(input);
}

#[test]
fn test_day9_part1() {
    let grid = load_input("./assets/day9_test.txt");
    let mut count = 0;
    for y in 0..grid.ysize {
        for x in 0..grid.xsize {
            if grid.is_low_point(x, y) {
                count += grid.data[y][x] + 1;
            }
        }
    }
    assert_eq!(15, count);
}

#[test]
fn test_day9_part2() {
    let grid = load_input("./assets/day9_test.txt");
    let mut basins = Vec::<Vec<(usize, usize)>>::new();
    for y in 0..grid.ysize {
        for x in 0..grid.xsize {
            if grid.is_low_point(x, y) {
                basins.push(grid.find_basin((x, y)));
            }
        }
    }
    basins.sort_by(|b, a| a.len().cmp(&b.len()));
    assert_eq!(1134, basins[0].len() * basins[1].len() * basins[2].len());
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
