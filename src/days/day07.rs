use std::{fs::File, collections::HashMap};

use itertools::Itertools;

use crate::common::Solution;


//type NodeLink = Option<Box<NodeData>>;
struct NodeData {
    parent: Option<String>,
    children: Vec<String>,
    node_type: NodeType,
    size: usize
}

impl NodeData {
    fn new(parent: Option<String>, node_type: NodeType, size: usize) -> Self {
        NodeData { parent, children: Vec::new(), node_type, size }
    }

    fn new_dir(parent: Option<String>) -> Self {
        Self::new(parent, NodeType::Dir, 0)
    }

    fn new_file(parent: Option<String>, size: usize) -> Self {
        Self::new(parent, NodeType::File, size)
    }
}

#[derive(PartialEq, Eq)]
enum NodeType {
    Dir,
    File
}

struct Filesystem {
    filemap: HashMap<String, NodeData>,
}

impl Filesystem {
    fn new() -> Self {
        Self { filemap: HashMap::new() }
    }

    fn build(&mut self, input: &Vec<String>) {
        let mut current_node = "".to_owned();
        self.filemap.insert(current_node.to_string(), NodeData::new_dir(None));

        for line in input {
            // Handle going out
            if line.eq("$ cd ..") {
                if let Some(entry) = self.filemap.get(&current_node) {
                    if let Some(parent_name) = &entry.parent {
                        current_node = parent_name.to_string();
                    }
                }
                continue;
            }

            // Handle going in
            if let Some(dir) = line.strip_prefix("$ cd ") {
                let key = current_node.to_string() + "_" + dir;
                let exists = self.filemap.contains_key(&key);
                if !exists {
                    self.filemap.get_mut(&current_node).unwrap().children.push(key.to_string());
                    self.filemap.insert(key.to_string(), NodeData::new_dir(Some(current_node.to_string())));
                }
                current_node = key;
                continue;
            }

            // Discard ls - any line without $ is done after ls anyway
            if line.eq("$ ls") {
                continue;
            }

            // Handle contents of ls
            let (info, name) = line.split_once(' ').unwrap();
            let key = current_node.to_string() + "_" + name;
            if info.eq("dir") { // Handle dir
                if !self.filemap.contains_key(&key.to_string()) {
                    self.filemap.get_mut(&current_node).unwrap().children.push(key.to_string());
                    self.filemap.insert(key.to_string(), NodeData::new_dir(Some(current_node.to_string())));
                }
            } else { // Handle file
                if !self.filemap.contains_key(&key.to_string()) {
                    self.filemap.get_mut(&current_node).unwrap().children.push(key.to_string());
                    self.filemap.insert(key.to_string(), NodeData::new_file(Some(current_node.to_string()), info.parse().unwrap()));
                }
            }
        }
    }

    fn print_tree(&self, root: &String, indent: usize) {
        for child in self.filemap.get(root).unwrap().children.iter() {
            let child_node = self.filemap.get(child).unwrap();
            println!("{}{} [{}]", "\t".repeat(indent), child, child_node.size);
            if let NodeType::Dir = child_node.node_type {
                self.print_tree(&child, indent + 1);
            }
        }
    }

    fn calc_size(&mut self, root: &String) -> usize {
        let node = self.filemap.get(root).unwrap();
        if let NodeType::File = node.node_type {
            return node.size;
        }

        let mut total_size = 0;
        let childs: Vec<String> = self.filemap.get(root).unwrap().children.iter().cloned().collect();
        for child in childs.iter() {
            total_size += self.calc_size(child);
        }

        self.filemap.get_mut(root).unwrap().size = total_size;
        total_size
    }

}

fn part1(input: &InputType) -> String {
    let mut fs = Filesystem::new();
    fs.build(input);
    fs.calc_size(&"_/".to_string());
    // fs.print_tree(&"".to_string(), 0);

    fs.filemap.iter()
    .filter(|(_, v)| v.node_type == NodeType::Dir)
    .map(|(_, v)| v.size)
    .filter(|s| s <= &100_000)
    .sum::<usize>()
    .to_string()
}

fn part2(input: &InputType) -> String {
    let mut fs = Filesystem::new();
    fs.build(input);
    fs.calc_size(&"_/".to_string());

    let total_size = 70000000;
    let needed_size = 30000000;
    let used_size = fs.filemap.get("_/").unwrap().size;
    let curr_free_size = total_size - used_size;
    let need_to_free_size = needed_size - curr_free_size;

    fs.filemap.iter()
    .filter(|(_, v)| v.node_type == NodeType::Dir)
    .map(|(_, v)| v.size)
    .filter(|s| s >= &need_to_free_size)
    .sorted()
    .take(1)
    .exactly_one()
    .unwrap()
    .to_string()
}

type InputType = Vec<String>;
fn parse_input(raw_input: &[String]) -> InputType {
    raw_input.iter().map(|x| x.parse().unwrap()).collect()
}

pub fn solve(raw_input: &[String]) -> Solution {
    let input = parse_input(raw_input);

    use std::time::Instant;
    let now = Instant::now();
    let solution = (part1(&input), part2(&input));
    let elapsed = now.elapsed();
    (solution, elapsed)
}
