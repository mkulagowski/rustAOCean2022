use std::{collections::HashSet, hash::Hash, cmp::max};

use crate::common::Solution;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

struct TreeSearcher {
    visible: HashSet<Point>
}

impl TreeSearcher {
    fn new() -> Self {
        Self { visible: HashSet::new() }
    }

    fn find_visible(&mut self, input: &Vec<Vec<u8>>) {
        let mut max_height;
        let row_size = input.get(0).unwrap().len();
        let col_size = input.len();

        // iterate each row 2 times
        for (y, row) in input.iter().enumerate().skip(1).take(col_size - 2) {
            max_height = 0;
            for (x, height) in row.iter().enumerate() {
                if height > &max_height {
                    // println!("Tree [{}, {}] with height {} visible, max height {}", x, y, height, max_height);
                    max_height = *height;
                    if x != 0 && x != (row_size - 1) {
                        self.visible.insert(Point::new(x, y));
                    }
                }
                if max_height == 9 {
                    break;
                }
            }
            max_height = 0;
            for (x, height) in row.iter().enumerate().rev() {
                if height > &max_height {
                    // println!("Tree [{}, {}] with height {} visible, max height {}", x, y, height, max_height);
                    max_height = *height;
                    if x != 0 && x != (row_size - 1) {
                        self.visible.insert(Point::new(x, y));
                    }
                }
                if max_height == 9 {
                    break;
                }
            }
        }

        for x in 1..(row_size - 2) {
            max_height = 0;
            for (y, row) in input.iter().enumerate() {
                let height = row.get(x).unwrap();
                if height > &max_height {
                    // println!("Tree [{}, {}] with height {} visible, max height {}", x, y, height, max_height);
                    max_height = *height;
                    if y != 0 && y != (col_size - 1) {
                        self.visible.insert(Point::new(x, y));
                    }
                }
                if max_height == 9 {
                    break;
                }
            }
            max_height = 0;
            for (y, row) in input.iter().enumerate().rev() {
                let height = row.get(x).unwrap();
                if height > &max_height {
                    // println!("Tree [{}, {}] with height {} visible, max height {}", x, y, height, max_height);
                    max_height = *height;
                    if y != 0 && y != (col_size - 1) {
                        self.visible.insert(Point::new(x, y));
                    }
                }
                if max_height == 9 {
                    break;
                }
            }
        }
    }

    fn print_finds(&self, input: &InputType) {
        let row_size = input.get(0).unwrap().len();
        let col_size = input.len();

        let mut visible_print = vec![vec![' '; row_size]; col_size];
        for Point{x, y} in self.visible.iter() {
            *visible_print.get_mut(*y).unwrap().get_mut(*x).unwrap() = char::from_digit(*input.get(*y).unwrap().get(*x).unwrap() as u32, 10).unwrap();
        }

        for r in visible_print.iter() {
            for c in r.iter() {
                print!("{} ", c);
            }
            println!("");
        }
    }

    fn get_val(x: usize, y: usize, input: &Vec<Vec<u8>>) -> u8 {
        *input.get(y).unwrap().get(x).unwrap()
    }

    fn get_scenic_score(x: usize, y: usize, input: &Vec<Vec<u8>>) -> usize {
        let mut total_score = 1;
        let row_size = input.get(0).unwrap().len() as i32;
        let col_size = input.len() as i32;
        let curr_height = Self::get_val(x, y, input);

        let mut xx = x as i32;
        let yy = y as i32;
        let mut score = 0;
        loop {
            xx -= 1;
            if xx < 0 || yy < 0 || xx == row_size || yy == col_size {
                break;
            }

            let height = Self::get_val(xx as usize, yy as usize, input);
            score += 1;
            if height >= curr_height {
                break;
            }
        }

        if score == 0 {
            return 0;
        }
        total_score *= score;
        score = 0;
        let mut xx = x as i32;
        loop {
            xx += 1;
            if xx < 0 || yy < 0 || xx == row_size || yy == col_size {
                break;
            }

            let height = Self::get_val(xx as usize, yy as usize, input);
            score += 1;
            if height >= curr_height {
                break;
            }
        }


        if score == 0 {
            return 0;
        }
        total_score *= score;
        score = 0;
        let xx = x as i32;
        let mut yy = y as i32;
        loop {
            yy -= 1;
            if xx < 0 || yy < 0 || xx == row_size || yy == col_size {
                break;
            }

            let height = Self::get_val(xx as usize, yy as usize, input);
            score += 1;
            if height >= curr_height {
                break;
            }
        }


        if score == 0 {
            return 0;
        }
        total_score *= score;
        score = 0;
        let mut yy = y as i32;
        loop {
            yy += 1;
            if xx < 0 || yy < 0 || xx == row_size || yy == col_size {
                break;
            }

            let height = Self::get_val(xx as usize, yy as usize, input);
            score += 1;
            if height >= curr_height {
                break;
            }
        }

        total_score * score
    }

    fn get_max_scenic_score(input: &Vec<Vec<u8>>) -> usize {
        let row_size = input.get(0).unwrap().len();
        let col_size = input.len();
        let mut max_score = 0;

        for x in 1..(row_size - 1) {
            for y in 1..(col_size - 1) {
                let score = Self::get_scenic_score(x, y, input);
                max_score = max(score, max_score);
            }
        }

        max_score
    }
}

fn part1(input: &InputType) -> String {
    let mut ts = TreeSearcher::new();
    ts.find_visible(input);
    // ts.print_finds(input);
    let row_size = input.get(0).unwrap().len();
    let col_size = input.len();
    (ts.visible.len() + row_size * 2 + col_size * 2 - 4).to_string()
}

fn part2(input: &InputType) -> String {
    TreeSearcher::get_max_scenic_score(input).to_string()
}

type InputType = Vec<Vec<u8>>;
fn parse_input(raw_input: &[String]) -> InputType {
    raw_input.iter().map(|x| x.chars().map(|y| y.to_digit(10).unwrap() as u8).collect()).collect()
}

pub fn solve(raw_input: &[String]) -> Solution {
    let input = parse_input(raw_input);

    use std::time::Instant;
    let now = Instant::now();
    let solution = (part1(&input), part2(&input));
    let elapsed = now.elapsed();
    (solution, elapsed)
}
