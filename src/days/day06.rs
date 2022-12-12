use std::{collections::HashMap};

use itertools::Itertools;

use crate::common::Solution;

struct LetterCounter {
    counter: HashMap<char, u8>,
    adds_performed: usize
}

impl LetterCounter {
    fn new() -> Self {
        Self { counter: HashMap::new(), adds_performed: 0 }
    }

    fn add(&mut self, letter: &char) {
        self.adds_performed += 1;
        *self.counter.entry(*letter).or_insert(0) += 1;
    }

    fn remove(&mut self, letter: &char) {
        let mut remove_entry = false;
        if let Some(entry) = self.counter.get_mut(letter) {
            remove_entry = *entry == 1;
            *entry -= 1;
        }

        if remove_entry {
            self.counter.remove(letter);
        }
    }

    fn is_valid(&self) -> bool {
        self.counter.values().all(|val| val <= &1)
    }
}

fn find_unique_quartet(input: &String) -> Option<usize> {
    if let Some(window) = input.chars()
    .enumerate()
    .tuple_windows()
    .find(|((_, a),(_, b),(_, c),(_, d))| a != b && a != c && a != d && b != c && b != d && c != d) {
        let last_idx = window.3.0;
        Some(last_idx + 1)
    } else {
        None
    }
}


fn find_unique_n(input: &String, n: usize) -> Option<usize> {
    let mut counter = LetterCounter::new();
    input.chars().take(n).for_each(|c| counter.add(&c));

    if counter.is_valid() {
        return Some(n);
    }

    input.chars()
    .zip(input.chars().skip(n))
    .find(|(prev, next)|  {
        counter.remove(prev);
        counter.add(next);
        counter.is_valid()
    }).and(Some(counter.adds_performed))
}

fn part1(input: &InputType) -> String {
    find_unique_n(input, 4).unwrap().to_string()
}

fn part2(input: &InputType) -> String {
    find_unique_n(input, 14).unwrap().to_string()
}

type InputType = String;
fn parse_input(raw_input: &[String]) -> InputType {
    raw_input.first().unwrap().to_owned()
}

pub fn solve(raw_input: &[String]) -> Solution {
    let input = parse_input(raw_input);

    use std::time::Instant;
    let now = Instant::now();
    let solution = (part1(&input), part2(&input));
    let elapsed = now.elapsed();
    (solution, elapsed)
}
