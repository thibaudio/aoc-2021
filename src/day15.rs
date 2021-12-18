use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Day15;

impl Day15 {
    pub fn run() {
        print!("Day15 - part1: {}\n", day15_part1("./assets/day15.txt"));
        print!("Day15 - part2: {}\n", day15_part2("./assets/day15.txt"));
    }
}

fn load_input(filename: impl AsRef<Path>) -> Vec<Vec<usize>> {
    let mut grid = Vec::<Vec<usize>>::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(instruction) = line {
                let mut grid_line = Vec::<usize>::new();
                for c in instruction.chars() {
                    grid_line.push(c.to_digit(10).unwrap() as usize);
                }
                grid.push(grid_line);
            }
        }
    }
    grid
}

fn neighbors(grid: &Vec<Vec<usize>>, coord: (usize, usize)) -> Vec<(usize, usize)> {
    let mut output = Vec::<(usize, usize)>::new();
    let maxy = grid.len() - 1;
    let maxx = grid[maxy].len() - 1;

    //left
    if coord.0 > 0 {
        output.push((coord.0 - 1, coord.1));
    }
    // right
    if coord.0 < maxx {
        output.push((coord.0 + 1, coord.1));
    }
    // up
    if coord.1 > 0 {
        output.push((coord.0, coord.1 - 1));
    }
    // down
    if coord.1 < maxy {
        output.push((coord.0, coord.1 + 1));
    }

    // // top left
    // if coord.0 > 0 && coord.1 > 0 {
    //     output.push((coord.0 - 1, coord.1 - 1));
    // }
    // // top right
    // if coord.0 < maxx && coord.1 > 0 {
    //     output.push((coord.0 + 1, coord.1 - 1));
    // }
    // // bottom left
    // if coord.0 > 0 && coord.1 < maxy {
    //     output.push((coord.0 - 1, coord.1 + 1));
    // }
    // // bottom right
    // if coord.0 < maxx && coord.1 < maxy {
    //     output.push((coord.0 + 1, coord.1 + 1));
    // }

    output
}

fn neighbors_2(grid: &Vec<Vec<usize>>, coord: (usize, usize)) -> Vec<(usize, usize)> {
    let mut output = Vec::<(usize, usize)>::new();
    let maxy = grid.len() * 5 - 1;
    let maxx = grid[grid.len() - 1].len() * 5 - 1;

    //left
    if coord.0 > 0 {
        output.push((coord.0 - 1, coord.1));
    }
    // right
    if coord.0 < maxx {
        output.push((coord.0 + 1, coord.1));
    }
    // up
    if coord.1 > 0 {
        output.push((coord.0, coord.1 - 1));
    }
    // down
    if coord.1 < maxy {
        output.push((coord.0, coord.1 + 1));
    }

    // // top left
    // if coord.0 > 0 && coord.1 > 0 {
    //     output.push((coord.0 - 1, coord.1 - 1));
    // }
    // // top right
    // if coord.0 < maxx && coord.1 > 0 {
    //     output.push((coord.0 + 1, coord.1 - 1));
    // }
    // // bottom left
    // if coord.0 > 0 && coord.1 < maxy {
    //     output.push((coord.0 - 1, coord.1 + 1));
    // }
    // // bottom right
    // if coord.0 < maxx && coord.1 < maxy {
    //     output.push((coord.0 + 1, coord.1 + 1));
    // }

    output
}

fn find_path(grid: &Vec<Vec<usize>>) -> usize {
    let mut frontier = PriorityQueue::<(usize, usize), Reverse<usize>>::new();
    frontier.push((0, 0), Reverse(0));
    let mut came_from = HashMap::<(usize, usize), (usize, usize)>::new();
    let mut cost_so_far = HashMap::<(usize, usize), usize>::new();
    let goal = (grid[grid.len() - 1].len() - 1, grid.len() - 1);

    cost_so_far.insert((0, 0), 0);

    while frontier.len() > 0 {
        let current = frontier.pop().unwrap();
        if current.0 == goal {
            break;
        }

        for next in neighbors(grid, current.0) {
            let new_cost = cost_so_far[&current.0] + grid[next.1][next.0];
            if !cost_so_far.contains_key(&next) || new_cost < *cost_so_far.get(&next).unwrap() {
                cost_so_far.insert(next, new_cost);
                frontier.push(next, Reverse(new_cost));
                came_from.insert(next, current.0);
            }
        }
    }

    let mut current = goal;
    let mut path = Vec::<(usize, usize)>::new();

    while current != (0, 0) {
        path.push(current);
        current = *came_from.get(&current).unwrap();
    }
    path.push((0, 0));
    path.reverse();

    *cost_so_far.get(&goal).unwrap()
}

fn find_path_2(grid: &Vec<Vec<usize>>) -> usize {
    let mut frontier = PriorityQueue::<(usize, usize), Reverse<usize>>::new();
    frontier.push((0, 0), Reverse(0));
    let mut came_from = HashMap::<(usize, usize), (usize, usize)>::new();
    let mut cost_so_far = HashMap::<(usize, usize), usize>::new();
    let goal = ((grid[grid.len() - 1].len() * 5 - 1), (grid.len() * 5 - 1));

    cost_so_far.insert((0, 0), 0);

    while frontier.len() > 0 {
        let current = frontier.pop().unwrap();
        if current.0 == goal {
            break;
        }

        for next in neighbors_2(grid, current.0) {
            let new_cost = cost_so_far[&current.0] + get_cost(grid, next);
            if !cost_so_far.contains_key(&next) || new_cost < *cost_so_far.get(&next).unwrap() {
                cost_so_far.insert(next, new_cost);
                frontier.push(next, Reverse(new_cost));
                came_from.insert(next, current.0);
            }
        }
    }

    let mut current = goal;
    let mut path = Vec::<((usize, usize), usize)>::new();

    while current != (0, 0) {
        path.push((current, *cost_so_far.get(&current).unwrap()));
        current = *came_from.get(&current).unwrap();
    }
    path.push(((0, 0), 0));
    path.reverse();

    *cost_so_far.get(&goal).unwrap()
}

fn get_cost(grid: &Vec<Vec<usize>>, coord: (usize, usize)) -> usize {
    let maxy = grid.len();
    let maxx = grid[grid.len() - 1].len();

    let coord_on_grid = (coord.0 % maxx, coord.1 % maxy);
    let offset = (coord.0 / maxx, coord.1 / maxy);

    let result = grid[coord_on_grid.1][coord_on_grid.0] + 1 * (offset.0 + offset.1);
    if result > 9 {
        result - 9
    } else {
        result
    }
}

fn day15_part1(filename: impl AsRef<Path>) -> i32 {
    let grid = load_input(filename);
    find_path(&grid) as i32
}

fn day15_part2(filename: impl AsRef<Path>) -> i32 {
    let grid = load_input(filename);
    find_path_2(&grid) as i32
}

#[test]
fn test_day15_part1() {
    assert_eq!(40, day15_part1("./assets/day15_test.txt"));
}

#[test]
fn test_day15_part2() {
    assert_eq!(315, day15_part2("./assets/day15_test.txt"));
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
