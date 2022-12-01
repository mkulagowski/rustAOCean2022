use crate::common::Solution;
use itertools::Itertools;

fn get_top_n_calories(input: &InputType, n: usize) -> i32 {
    input.iter()
    .map(|x| x.iter().sum())
    .sorted()
    .rev()
    .take(n)
    .collect::<Vec<i32>>()
    .into_iter()
    .sum()
}

fn part1(input: &InputType) -> String {
    get_top_n_calories(input, 1).to_string()
}

fn part2(input: &InputType) -> String {
    get_top_n_calories(input, 3).to_string()
}

type InputType = Vec<Vec<i32>>;
fn parse_input(raw_input: &[String]) -> InputType {
    raw_input.split(|elem| elem.is_empty())
    .into_iter()
    .map(|subarray|
        subarray.into_iter()
        .map(|x| x.parse().expect(&format!("Could not parse value {}", x)))
        .collect()
    )
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
