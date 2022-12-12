use std::collections::VecDeque;

use crate::common::Solution;

const START_VAL: u8 = 0;
const END_VAL: u8 = b'z' - b'a' + 2;

struct Point {
    x: usize,
    y: usize
}

impl Point {
    fn new(xx: usize, yy: usize) -> Self {
        Self {x:xx, y:yy}
    }
}

struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>
}

impl<T> Matrix<T> {
    fn get(&self, pos: &Point) -> Option<&T> {
        self.data.get(self.cols * pos.y + pos.x)
    }

    fn get_mut(&mut self, pos: &Point) -> Option<&mut T> {
        self.data.get_mut(self.cols * pos.y + pos.x)
    }

    fn get_neighbours(&self, p: &Point) -> Vec<Point> {
        let mut res = Vec::new();
        if p.y > 0 {
            res.push(Point::new(p.x, p.y - 1));
        }
        if p.x > 0 {
            res.push(Point::new(p.x - 1, p.y));
        }
        if p.x < self.cols - 1 {
            res.push(Point::new(p.x + 1, p.y));
        }
        if p.y < self.rows - 1 {
            res.push(Point::new(p.x, p.y + 1));
        }
        res
    }
}

struct State {
    steps: i32,
    pos: Point
}

impl State {
    fn new(steps: i32, pos: Point) -> Self {
        Self { steps, pos }
    }
}

fn part1(input: &InputType) -> String {
    let mut visited = Matrix { rows: input.rows, cols: input.cols, data: vec![false; input.data.len()] };

    let mut states: VecDeque<State> = input.data.iter().enumerate().filter(|&(_, x)| x == &START_VAL).map(|(idx, _)| {
        let y = idx / input.cols;
        let x = idx % input.cols;
        State::new(0, Point::new(x, y))
    }).collect();

    while let Some(state) = states.pop_front() {
        let was_visited = visited.get_mut(&state.pos).unwrap();
        if *was_visited {
            continue;
        }
        *was_visited = true;

        let curr_height = input.get(&state.pos).unwrap();
        if curr_height == &END_VAL {
            return state.steps.to_string();
        }

        let neighs = input.get_neighbours(&state.pos);

        for n in neighs.into_iter() {
            let neigh_height = input.get(&n).unwrap();
            let is_valid = neigh_height <= &(curr_height + 1);
            if is_valid {
                states.push_back(State::new(state.steps + 1, n));
            }
        }
    }
    "".to_string()
}

fn part2(input: &InputType) -> String {
    let mut visited = Matrix { rows: input.rows, cols: input.cols, data: vec![false; input.data.len()] };

    let mut states: VecDeque<State> = input.data.iter().enumerate().filter(|&(_, x)| x == &START_VAL || x == &(START_VAL + 1)).map(|(idx, _)| {
        let y = idx / input.cols;
        let x = idx % input.cols;
        State::new(0, Point::new(x, y))
    }).collect();

    while let Some(state) = states.pop_front() {
        let was_visited = visited.get_mut(&state.pos).unwrap();
        if *was_visited {
            continue;
        }
        *was_visited = true;

        let curr_height = input.get(&state.pos).unwrap();
        if curr_height == &END_VAL {
            return state.steps.to_string();
        }

        let neighs = input.get_neighbours(&state.pos);

        for n in neighs.into_iter() {
            let neigh_height = input.get(&n).unwrap();
            let is_valid = neigh_height <= &(curr_height + 1);
            if is_valid {
                states.push_back(State::new(state.steps + 1, n));
            }
        }
    }
    "".to_string()
}

type InputType = Matrix<u8>;
fn parse_input(raw_input: &[String]) -> InputType {
    let rows = raw_input.len();
    let cols = raw_input.get(0).unwrap().len();
    let data = raw_input.iter().flat_map(|x| x.bytes().map(|c| {
        match c {
            b'S' => START_VAL,
            b'a'..=b'z' => c - b'a' + 1,
            b'E' => END_VAL,
            _ => panic!()
        }
    })).collect();
    Matrix { rows, cols, data }
}

pub fn solve(raw_input: &[String]) -> Solution {
    let input = parse_input(raw_input);

    use std::time::Instant;
    let now = Instant::now();
    let solution = (part1(&input), part2(&input));
    let elapsed = now.elapsed();
    (solution, elapsed)
}
