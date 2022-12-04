use std::str::FromStr;

use crate::common::Solution;

#[derive(Debug)]
struct Assignment {
    from: i32,
    to: i32
}

impl Assignment {
    fn contains(&self, other: &Assignment) -> bool {
        self.from <= other.from && self.to >= other.to
    }

    fn overlaps_start_of(&self, other: &Assignment) -> bool {
        self.from <= other.from && self.to >= other.from
    }
}

impl FromStr for Assignment {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((from, to)) = s.split_once('-') {
            let (from, to) = (from.parse().unwrap(), to.parse().unwrap());
            Ok(Assignment { from, to })
        } else {
            Err(())
        }
    }
}

impl FromStr for Pair {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((first, second)) = s.split_once(',') {
            let (first, second) = (first.parse().unwrap(), second.parse().unwrap());
            Ok(Pair(first, second))
        } else {
            Err(())
        }
    }
}

#[derive(Debug)]
struct Pair (Assignment, Assignment);

impl Pair {
    fn is_contained(&self) -> bool {
        self.0.contains(&self.1) || self.1.contains(&self.0)
    }

    fn is_overlapped(&self) -> bool {
        self.0.overlaps_start_of(&self.1) || self.1.overlaps_start_of(&self.0)
    }
}

fn part1(input: &InputType) -> String {
    input.iter()
    .filter(|p| p.is_contained())
    .count()
    .to_string()
}

fn part2(input: &InputType) -> String {
    input.iter()
    .filter(|p| p.is_overlapped())
    .count()
    .to_string()
}

type InputType = Vec<Pair>;
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
