use std::cmp::Ordering;

use itertools::Itertools;
use serde_json::Value;

use crate::common::Solution;

#[derive(Debug, Clone)]
enum PacketValue {
    Value(i32),
    List(Vec<PacketValue>),
}

impl Ord for PacketValue {
    fn cmp(&self, other: &Self) -> Ordering {
        // println!("called cmp!");
        // println!("comparing {:?} and {:?} = ?", self, other);
        let ord = match (self, other) {
            (PacketValue::Value(left), PacketValue::Value(right)) => left.cmp(right),
            (PacketValue::List(left), PacketValue::List(right)) => left.cmp(right),
            (PacketValue::List(left), right) => left.cmp(&vec![right.clone()]),
            (left, PacketValue::List(right)) => vec![left.clone()].cmp(right),
        };
        // println!(
        //     "\tcomparing {} and {} = {:?}",
        //     self.print(),
        //     other.print(),
        //     ord
        // );
        ord
    }
}

impl PartialOrd for PacketValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PacketValue {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl Eq for PacketValue {}

impl PacketValue {
    fn from(val: &Value) -> PacketValue {
        match val {
            Value::Number(v) => PacketValue::Value(v.as_i64().unwrap() as i32),
            Value::Array(l) => {
                PacketValue::List(l.iter().map(|vv| PacketValue::from(vv)).collect())
            }
            _ => panic!("Value other than Number or Array!"),
        }
    }

    fn print(&self) -> String {
        match self {
            PacketValue::Value(x) => format!("{}", x),
            PacketValue::List(l) => {
                let ss: Vec<String> = l.iter().map(|p| p.print()).collect();
                format!("[{}]", ss.join(","))
            }
        }
    }
}

fn part1(input: &InputType) -> String {
    input
        .iter()
        .tuples()
        .enumerate()
        .filter(|(_, (a, b))| a.cmp(b).is_lt())
        .map(|(idx, _)| idx + 1)
        .sum::<usize>()
        .to_string()
}

fn part2(input: &InputType) -> String {
    let dividers: [PacketValue; 2] = [
        PacketValue::List(vec![PacketValue::List(vec![PacketValue::Value(2)])]),
        PacketValue::List(vec![PacketValue::List(vec![PacketValue::Value(6)])]),
    ];

    input
        .iter()
        .chain(dividers.iter())
        .sorted()
        .into_iter()
        .enumerate()
        .filter(|(_, x)| dividers.contains(x))
        .map(|(i, _)| i + 1)
        .product::<usize>()
        .to_string()
}

type InputType = Vec<PacketValue>;
fn parse_input(raw_input: &[String]) -> InputType {
    raw_input
        .iter()
        .filter(|x| !x.is_empty())
        .map(|x| PacketValue::from(&serde_json::from_str(x).unwrap()))
        .collect()
}

pub fn solve(raw_input: &[String]) -> Solution {
    let input = parse_input(raw_input);

    use std::time::Instant;
    let now = Instant::now();
    let solution = (part1(&input), part2(&input));
    let elapsed = now.elapsed();
    (solution, elapsed)
}
