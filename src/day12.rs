use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Day12;

impl Day12 {
    pub fn run() {
        print!("Day12 - part1: {}\n", day12_part1("./assets/day12.txt"));
        print!("Day12 - part2: {}\n", day12_part2("./assets/day12.txt"));
    }
}

struct Node {
    name: String,
    parent: Vec<String>,
    children: Vec<String>,
    small: bool,
}

impl Node {
    fn new(node: String) -> Node {
        if node == "start" || node == "end" {
            Node {
                name: node,
                parent: Vec::new(),
                small: true,
                children: Vec::new(),
            }
        } else {
            let char_node = node.chars().next().unwrap();
            if char_node.is_uppercase() {
                Node {
                    name: node,
                    parent: Vec::new(),
                    small: false,
                    children: Vec::new(),
                }
            } else {
                Node {
                    name: node,
                    parent: Vec::new(),
                    small: true,
                    children: Vec::new(),
                }
            }
        }
    }
}
fn walk(nodes: &HashMap<String, Node>) -> Vec<Vec<String>> {
    let mut queue = Vec::<(Vec<String>, String)>::new();
    queue.push((Vec::<String>::new(), "start".to_string()));
    let mut paths = Vec::<Vec<String>>::new();
    loop {
        let context = queue.pop().unwrap();

        let current_node = nodes.get(&context.1).unwrap();
        let mut current_path = context.0;

        if context.1 == "end" {
            current_path.push(current_node.name.clone());
            paths.push(current_path);
        } else if !current_node.small || !current_path.contains(&current_node.name) {
            current_path.push(current_node.name.clone());
            for n in current_node.children.iter() {
                queue.push((current_path.clone(), n.to_string()));
            }
            for n in current_node.parent.iter() {
                queue.push((current_path.clone(), n.to_string()));
            }
        }

        if queue.len() == 0 {
            break;
        }
    }
    paths
}

fn walk2(nodes: &HashMap<String, Node>) -> Vec<Vec<String>> {
    let mut queue = Vec::<(Vec<String>, String)>::new();
    queue.push((Vec::<String>::new(), "start".to_string()));
    let mut paths = Vec::<Vec<String>>::new();
    loop {
        let context = queue.pop().unwrap();

        let current_node = nodes.get(&context.1).unwrap();
        let mut current_path = context.0;

        if context.1 == "end" {
            current_path.push(current_node.name.clone());
            paths.push(current_path);
        } else if can_visit(current_node, &current_path) {
            current_path.push(current_node.name.clone());
            for n in current_node.children.iter() {
                if n == "start" {
                    continue;
                }
                queue.push((current_path.clone(), n.to_string()));
            }
            for n in current_node.parent.iter() {
                if n == "start" {
                    continue;
                }
                queue.push((current_path.clone(), n.to_string()));
            }
        }

        if queue.len() == 0 {
            break;
        }
    }
    paths
}

fn can_visit(node: &Node, path: &Vec<String>) -> bool {
    if !node.small {
        return true;
    }
    let mut already = false;
    for i in path {
        if i.chars().next().unwrap().is_uppercase() {
            continue;
        }
        if path.iter().filter(|x| x == &i).count() > 1 {
            already = true;
        }
    }

    if already {
        return !path.contains(&node.name);
    } else {
        return true;
    }
}
fn load_input(filename: impl AsRef<Path>) -> HashMap<String, Node> {
    let mut nodes: HashMap<String, Node> = HashMap::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(instruction) = line {
                let mut link = instruction.split('-');
                let nodea = link.next().unwrap();
                let nodeb = link.next().unwrap();
                if !nodes.contains_key(nodea) {
                    nodes.insert(nodea.to_string(), Node::new(nodea.to_string()));
                }
                if !nodes.contains_key(nodeb) {
                    nodes.insert(nodeb.to_string(), Node::new(nodeb.to_string()));
                }

                let parent = nodes.get_mut(nodea).unwrap();
                parent.children.push(nodeb.to_string());

                let child = nodes.get_mut(nodeb).unwrap();
                child.parent.push(nodea.to_string())
            }
        }
    }
    nodes
}

fn day12_part1(filename: impl AsRef<Path>) -> i32 {
    let nodes = load_input(filename);
    return walk(&nodes).len() as i32;
}

fn day12_part2(filename: impl AsRef<Path>) -> i32 {
    let nodes = load_input(filename);
    return walk2(&nodes).len() as i32;
}

#[test]
fn test_day12_part1() {
    assert_eq!(10, day12_part1("./assets/day12_test.txt"));
}

#[test]
fn test_day12_part2() {
    assert_eq!(36, day12_part2("./assets/day12_test.txt"));
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
