use std::{str::FromStr, collections::HashSet, fmt};

use itertools::Itertools;

use crate::common::Solution;
#[derive(Debug)]
enum Direction {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
    Diag(i32, i32)
}

impl Direction {
    fn val(&self) -> i32 {
        match self {
            Direction::Up(val) => *val,
            Direction::Down(val) => *val,
            Direction::Left(val) => *val,
            Direction::Right(val) => *val,
            _ => 0
        }
    }
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((x, y)) = s.split_once(" ") {
            let val = y.parse().unwrap();
            return match x {
                "U" => Ok(Direction::Up(val)),
                "D" => Ok(Direction::Down(val)),
                "L" => Ok(Direction::Left(val)),
                "R" => Ok(Direction::Right(val)),
                _ => Err(())
            };
        }
        Err(())
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{},{}]", self.x, self.y)
    }
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn is_in_line(&self, other: &Point) -> bool {
        self.x == other.x || self.y == other.y
    }

    fn make_move(&mut self, dir: &Direction) {
        match dir {
            Direction::Up(val) => self.y += 1,
            Direction::Down(val) => self.y -= 1,
            Direction::Left(val) => self.x -= 1,
            Direction::Right(val) => self.x += 1,
            Direction::Diag(x, y) => {self.x += x; self.y += y;}
        }
    }

    fn is_too_far(&self, other: &Point) -> bool {
        if self.is_in_line(other) {
            (other.x - self.x + other.y - self.y).abs() > 1
        } else {
            (other.x - self.x).abs() + (other.y - self.y).abs() > 2
        }
    }

    fn get_tail_move(head: &Point, tail: &Point) -> Option<Direction> {
        if head == tail || !head.is_too_far(tail) {
            return None;
        }

        // in line
        if head.is_in_line(tail) {
            if head.x > tail.x {
                return Some(Direction::Right(1));
            } else if head.x < tail.x {
                return Some(Direction::Left(1));
            } else if head.y > tail.y {
                return Some(Direction::Up(1));
            } else {
                return Some(Direction::Down(1));
            }
        }

        // diagonal
        let diag_x = if head.x > tail.x { 1 } else { -1 };
        let diag_y = if head.y > tail.y { 1 } else { -1 };
        Some(Direction::Diag(diag_x, diag_y))
    }
}


fn part1(input: &InputType) -> String {
    let mut head = Point::new(0, 0);
    let mut tail = Point::new(0, 0);
    let mut tail_moves = HashSet::new();
    tail_moves.insert(tail);
    for dir in input.iter() {
        let dist = match dir {
            Direction::Up(val) => val,
            Direction::Down(val) => val,
            Direction::Left(val) => val,
            Direction::Right(val) => val,
            _ => &0
        };
        for _ in 0..*dist {
            head.make_move(dir);
            if let Some(tail_dir) = Point::get_tail_move(&head, &tail) {
                tail.make_move(&tail_dir);
                tail_moves.insert(tail);
            }
        }

    }
    tail_moves.len().to_string()
}

fn part2(input: &InputType) -> String {
    let mut head = Point::new(0, 0);
    let mut body = vec![Point::new(0,0); 8];
    let mut tail = Point::new(0, 0);
    let mut tail_moves = HashSet::new();
    tail_moves.insert(tail);
    for dir in input.iter() {
        let dist = dir.val();
        for _ in 0..dist {
            head.make_move(dir);

            // connect head with body
            if let Some(tail_dir) = Point::get_tail_move(&head, &body.first().unwrap()) {
                body.first_mut().unwrap().make_move(&tail_dir);
            }

            for idx in 0..(body.len()-1) {
                let body_head = body.get(idx).unwrap();
                let body_tail = body.get(idx + 1).unwrap();
                if let Some(tail_dir) = Point::get_tail_move(body_head, body_tail) {
                    body.get_mut(idx + 1).unwrap().make_move(&tail_dir);
                }
            }

            // connect body with tail
            if let Some(tail_dir) = Point::get_tail_move(&body.last().unwrap(), &tail) {
                tail.make_move(&tail_dir);
                tail_moves.insert(tail);
            }
        }

    }
    tail_moves.len().to_string()
}

type InputType = Vec<Direction>;
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
