use std::str::FromStr;

use itertools::Itertools;

use crate::common::Solution;

#[derive(Debug, Clone, Copy)]
struct Move {
    num: u8,
    from: usize,
    to: usize
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(stripped) = s.strip_prefix("move ") {
            let (num, rest) = stripped.split_once(" from ").unwrap();
            let (from, to) = rest.split_once(" to ").unwrap();

            Ok(Move {
                num: num.parse().unwrap(),
                from: from.parse().unwrap(),
                to: to.parse().unwrap()
            })
        } else {
            Err(())
        }
    }
}

#[derive(Debug, Clone)]
struct StackData {
    stacks: Vec<Vec<char>>,
    moves: Vec<Move>
}

impl StackData {
    fn perform_moves(&mut self) {
        self.moves.iter()
        .flat_map(|m| std::iter::repeat((m.from, m.to)).take(m.num.into()))
        .for_each(|(from, to)| {
            if let Some(elem)  = self.stacks.get_mut(from - 1).unwrap().pop() {
                self.stacks.get_mut(to - 1).unwrap().push(elem);
            }
        });
    }

    fn perform_moves_batched(&mut self) {
        self.moves.iter()
        .for_each(|Move{num, from, to}| {
            let from  = self.stacks.get_mut(from - 1).unwrap();
            let mut from_elems = from.split_off(from.len() - *num as usize);
            self.stacks.get_mut(to - 1).unwrap().append(&mut from_elems);
        });
    }

    fn get_top_values(&self) -> String {
        self.stacks.iter()
        .filter_map(|stack| stack.last())
        .collect()
    }
}

fn part1(input: &InputType) -> String {
    let mut input_cloned = input.clone();
    input_cloned.perform_moves();
    input_cloned.get_top_values().to_string()
}

fn part2(input: &mut InputType) -> String {
    input.perform_moves_batched();
    input.get_top_values().to_string()
}

fn parse_stacks(input: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let num_row = input.last().unwrap();
    num_row.iter()
    .enumerate()
    .filter(|(_, r)| r.is_numeric())
    .map(|(i, _)|  input.iter()
        .rev()
        .skip(1)
        .map(move |row| row.get(i).unwrap())
        .take_while(|val| val.is_ascii_alphabetic())
        .copied()
        .collect_vec()
    )
    .collect_vec()
}

type InputType = StackData;
fn parse_input(raw_input: &[String]) -> InputType {
    let stack_info = raw_input.iter()
    .take_while(|line| !line.is_empty())
    .map(|line| line.chars().into_iter().collect_vec())
    .collect_vec();

    let stacks = parse_stacks(&stack_info);

    let moves = raw_input.iter()
    .filter_map(|line| line.parse().ok())
    .collect_vec();
    StackData { stacks, moves }
}

pub fn solve(raw_input: &[String]) -> Solution {
    let mut input = parse_input(raw_input);

    use std::time::Instant;
    let now = Instant::now();
    let solution = (part1(&input), part2(&mut input));
    let elapsed = now.elapsed();
    (solution, elapsed)
}
