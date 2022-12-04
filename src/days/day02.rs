use itertools::Itertools;

use crate::common::Solution;

#[derive(PartialEq, Eq, Copy, Clone)]
enum Move {
    Rock = 0,
    Paper,
    Scissor,
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum Outcome {
    Loose = 0,
    Draw,
    Win,
}

impl From<u8> for Move {
    fn from(item: u8) -> Self {
       match item {
        0 => Move::Rock,
        1 => Move::Paper,
        2 => Move::Scissor,
        _ => panic!()
       }
    }
}

impl From<u8> for Outcome {
    fn from(item: u8) -> Self {
       match item {
        0 => Outcome::Loose,
        1 => Outcome::Draw,
        2 => Outcome::Win,
        _ => panic!()
       }
    }
}

fn get_outcome(player: &Move, opponent: &Move) -> Outcome {
    let player = *player as u8;
    let opponent = *opponent as u8;
    let opp_loosing_hand = (3 + player - 1) % 3;
    let opp_winning_hand = (3 + player + 1) % 3;

    if opponent == opp_winning_hand {
        Outcome::Loose
    } else if  opponent == opp_loosing_hand {
        Outcome::Win
    } else if opponent == player {
        Outcome::Draw
    } else {
        panic!()
    }
}

fn find_winning_move(opponent: &Move, outcome: &Outcome) -> Move {
    let opponent = *opponent as i32 + 3;
    let delta = *outcome as i32 - 1;
    let winning_move: u8 = ((opponent + delta) % 3).try_into().unwrap();
    Move::from(winning_move)
}

fn get_score(my_move: &Move, outcome: &Outcome) -> u8 {
    let move_val = *my_move as u8 + 1;
    let outcome_val = (*outcome  as u8) * 3;
    move_val + outcome_val

}

fn part1(input: &InputType) -> String {
    input.iter()
    .map(|(opp, player)| (Move::from(*opp), Move::from(*player)))
    .map(|(opp, player)| get_score(&player, &get_outcome(&player, &opp)) as u32)
    .sum::<u32>()
    .to_string()
}

fn part2(input: &InputType) -> String {
    input.iter()
    .map(|(opp, outcome)| (Move::from(*opp), Outcome::from(*outcome)))
    .map(|(opp, outcome)| get_score(&find_winning_move(&opp, &outcome), &outcome) as u32)
    .sum::<u32>()
    .to_string()
}

fn idx_from_str(s: &str) -> Result<u8, ()> {
    if let Ok(Some(c)) = s.chars().at_most_one() {
        match c {
            'A' | 'X' => Ok(0),
            'B' | 'Y' => Ok(1),
            'C' | 'Z' => Ok(2),
            _ => Err(()),
        }
    } else {
        Err(())
    }
}

type InputType = Vec<(u8, u8)>;
fn parse_input(raw_input: &[String]) -> InputType {
    raw_input.iter()
    .filter_map(|line| line.split_once(" "))
    .map(|(l, r)| (idx_from_str(l).unwrap(), idx_from_str(r).unwrap()))
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
