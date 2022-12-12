use std::collections::VecDeque;

use itertools::Itertools;

use crate::common::Solution;

#[derive(Clone, PartialEq, Eq, Debug)]
struct Monkey {
    items: Vec<u64>,
    test_num: u64,
    test_passed_idx: usize,
    test_failed_idx: usize,
    inspections: u64,
    op_component: Comp,
    op_is_mult: bool,
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum Comp {
    Old,
    Val(u64)
}

impl Monkey {

    fn operation(&self, item: u64) -> u64 {
        match self.op_component {
            Comp::Old => if self.op_is_mult { item * item } else { item + item },
            Comp::Val(v) => if self.op_is_mult { item * v } else { item + v },
        }
    }

    fn from(s: &[String]) -> Result<Self, ()> {
        if s.len() != 7 {
            println!("Err while parsing! Got {} lines instead of 7!", s.len());
            return Err(());
        }

        let items = s.get(1).unwrap().trim_start().strip_prefix("Starting items: ").unwrap().split(", ").into_iter().map(|s| s.parse().unwrap()).collect();
        let test_num = s.get(3).unwrap().trim_start().strip_prefix("Test: divisible by ").unwrap().parse().unwrap();
        let test_passed_idx = s.get(4).unwrap().trim_start().strip_prefix("If true: throw to monkey ").unwrap().parse().unwrap();
        let test_failed_idx = s.get(5).unwrap().trim_start().strip_prefix("If false: throw to monkey ").unwrap().parse().unwrap();
        let op_str = s.get(2).unwrap().trim_start().strip_prefix("Operation: new = old ").unwrap();
        let op_is_mult = op_str.contains("*");
        let delim = if op_is_mult {"* "} else {"+ "};
        let right = op_str.strip_prefix(delim).unwrap();
        let op_component = match right {
            "old" => Comp::Old,
            v => Comp::Val(v.parse().unwrap())
        };
        Ok(Self { items, test_num, test_passed_idx, test_failed_idx, inspections: 0, op_component, op_is_mult})
    }

    fn inspect(&mut self, item: u64, drop_worry: bool) -> (usize, u64) {
        self.inspections += 1;
        // inspect
        let item_op = self.operation(item);
        // get bored
        let new_item = if drop_worry { item_op / 3 } else { item_op};
        // test
        if new_item % self.test_num == 0 {
            (self.test_passed_idx, new_item)
        } else {
            (self.test_failed_idx, new_item)
        }
    }

}

fn part1(input: &InputType) -> String {
    let mut monkes = input.clone();
    let mut current_items: VecDeque<(usize, u64)> = input.iter()
    .enumerate()
    .flat_map(|(idx, m)| m.items.iter().map(move |item| (idx, *item)))
    .collect();
    let mut future_items: VecDeque<(usize, u64)> = VecDeque::new();
    for _ in 0..20 {
        while let Some((idx, item)) = current_items.pop_front() {
            let monke = monkes.get_mut(idx).unwrap();

            let (next_idx, next_item) = monke.inspect(item, true);

            if next_idx > idx {
                current_items.push_back((next_idx, next_item));
            } else {
                future_items.push_back((next_idx, next_item));
            }
        }
        current_items = future_items.drain(..).collect();
    }

    monkes.iter()
    .map(|m| m.inspections)
    .sorted()
    .rev()
    .take(2)
    .product::<u64>()
    .to_string()
}

fn part2(input: &InputType) -> String {
    let mut monkes = input.clone();
    let mut current_items: VecDeque<(usize, u64)> = input.iter().enumerate().flat_map(|(idx, m)| m.items.iter().map(move |item| (idx, *item))).collect();
    let mut future_items: VecDeque<(usize, u64)> = VecDeque::new();
    let max_item_val: u64 = input.iter().map(|monke| monke.test_num).product();
    for _ in 0..10000 {
        while let Some((idx, item)) = current_items.pop_front() {
            let monke = monkes.get_mut(idx).unwrap();

            let (next_idx, next_item) = monke.inspect(item, false);
            let next_item = next_item % max_item_val;

            if next_idx > idx {
                current_items.push_back((next_idx, next_item));
            } else {
                future_items.push_back((next_idx, next_item));
            }
        }
        current_items = future_items.drain(..).collect();
    }

    monkes.iter()
    .map(|m| m.inspections)
    .sorted()
    .rev()
    .take(2)
    .product::<u64>()
    .to_string()
}

type InputType = Vec<Monkey>;
fn parse_input(raw_input: &[String]) -> InputType {
    let mut result = Vec::new();
    for i in (0..raw_input.len()).step_by(7) {
        if let Ok(monke) = Monkey::from(&raw_input[i..i+7]) {
            result.push(monke);
        }
    }
    result
}

pub fn solve(raw_input: &[String]) -> Solution {
    let input = parse_input(raw_input);

    use std::time::Instant;
    let now = Instant::now();
    let solution = (part1(&input), part2(&input));
    let elapsed = now.elapsed();
    (solution, elapsed)
}
