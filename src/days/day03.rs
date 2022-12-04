use std::str::FromStr;

use itertools::Itertools;

use crate::common::Solution;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
struct Itemtype(char);

impl Itemtype {
    fn get_rank(&self) -> u8 {
        if self.0.is_ascii_lowercase() {
            1 + self.0 as u8 - ('a' as u8)
        } else {
            27 + self.0 as u8 - ('A' as u8)
        }
    }
}

#[derive(Clone, Copy)]
struct ItemtypeCounter {
    data: [u32; 26*2]
}

impl ItemtypeCounter {
    fn new() -> Self {
        Self { data : [0; 26*2] }
    }

    fn get_char(i: usize) -> char {
        let prefix = if i > 25 { 'A' } else { 'a' } as u8;
        ((i % 26) as u8 + prefix) as char
    }

    fn get_idx(c: &Itemtype) -> usize {
        if c.0.is_ascii_lowercase() {
            c.0 as u8 - ('a' as u8)
        } else {
            c.0 as u8 - ('A' as u8) + 26
        }.into()
    }

    fn add(&mut self, c: &Itemtype) {
        let idx = ItemtypeCounter::get_idx(c);
        let entry = self.data.get_mut(idx).unwrap();
        *entry = *entry + 1;
    }

    fn add_sack(&mut self, r: &Rucksack) {
        r.compartments.0.iter().chain(r.compartments.1.iter()).for_each(|c| self.add(c))
    }

    fn find_first_same(&self, other: &ItemtypeCounter) -> Option<Itemtype> {
        for (i, (a, b)) in self.data.iter().zip(other.data.iter()).enumerate() {
            if *a > 0 && *b > 0 {
                return Some(Itemtype(ItemtypeCounter::get_char(i)));
            }
        }
        None
    }

    fn find_first_same3(&self, other1: &ItemtypeCounter, other2: &ItemtypeCounter) -> Option<Itemtype> {
        for (i, ((a, b), c)) in self.data.iter().zip(other1.data.iter()).zip(other2.data.iter()).enumerate() {
            if *a > 0 && *b > 0 && *c > 0 {
                return Some(Itemtype(ItemtypeCounter::get_char(i)));
            }
        }
        None
    }

}

#[derive(PartialEq, Eq, Clone)]
struct Rucksack {
    compartments: (Vec<Itemtype>, Vec<Itemtype>)
}

impl Rucksack {
    fn get_error_item(&self) -> Option<Itemtype> {
        let mut counter1 = ItemtypeCounter::new();
        let mut counter2 = ItemtypeCounter::new();
        self.compartments.0.iter().for_each(|c| counter1.add(&c));
        self.compartments.1.iter().for_each(|c| counter2.add(&c));
        counter1.find_first_same(&counter2)
    }

    fn find_badge(group: &Vec<&Rucksack>) -> Option<Itemtype> {
        let mut counters = vec![ItemtypeCounter::new(); group.len()];
        group.iter().zip(counters.iter_mut()).for_each(|(r, c)| c.add_sack(r));

        let c1 = counters.get(0).unwrap();
        let c2 = counters.get(1).unwrap();
        let c3 = counters.get(2).unwrap();
        c1.find_first_same3(c2, c3)
    }
}

impl FromStr for Rucksack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_at(s.len() / 2);
        let compartments: (Vec<Itemtype>, Vec<Itemtype>) = [x, y].into_iter()
            .map(|comp| comp.chars()
            .map(|c| Itemtype(c))
            .collect()
        ).collect_tuple().unwrap();
        Ok(Rucksack { compartments })
    }
}

fn part1(input: &InputType) -> String {
    input.iter()
    .filter_map(Rucksack::get_error_item)
    .map(|i| i.get_rank() as u32)
    .sum::<u32>()
    .to_string()
}

fn part2(input: &InputType) -> String {
    input.chunks(3)
    .filter_map(|chunk| {
        let vv = chunk.into_iter().collect_vec();
        Rucksack::find_badge(&vv)
    })
    .map(|i| i.get_rank() as u32)
    .sum::<u32>()
    .to_string()
}


type InputType = Vec<Rucksack>;
fn parse_input(raw_input: &[String]) -> InputType {
    raw_input.iter()
    .map(|line| line.parse().unwrap())
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
