use crate::common::Solution;

enum Instruction {
    noop,
    sety(i32),
    addxy,
}

#[derive(Default)]
struct Cpu {
    x: i64,
    y: i64,
    cycle: usize
}

impl Cpu {
    fn new() -> Self {
        Cpu { x: 1, y: 0, cycle: 0}
    }

    fn bump_cycle(&mut self) {
        self.cycle += 1;
    }

    fn perform(&mut self, task: &Instruction) {
        match task {
            Instruction::noop => (),
            Instruction::sety(v) => self.y = *v as i64,
            Instruction::addxy => self.x += self.y
        }
    }

    fn get_signal_strength(&self) -> i64 {
        self.x * (self.cycle as i64)
    }
}

fn part1(input: &InputType) -> String {
    let mut cpu = Cpu::new();
    let mut next_check = 19;
    let mut signal_sum = 0;
    for i in 0..input.len() {
        cpu.bump_cycle();

        if i == next_check {
            let sum = cpu.get_signal_strength();
            signal_sum += sum;
            next_check += 40;
        }

        let task = input.get(i).unwrap();
        cpu.perform(task);
    }
    signal_sum.to_string()
}

fn part2(input: &InputType) -> String {
    let mut cpu = Cpu::new();
    let mut crt = ['.'; 40*6];

    for i in 0..(input.len() as i64) {
        cpu.bump_cycle();
        if cpu.x.abs_diff(i % 40) <= 1 {
            *crt.get_mut(i as usize).unwrap() = '#';
        }

        let task = input.get(i as usize).unwrap();
        cpu.perform(task);
    }
    // Read the result manually
    // crt.chunks(40).for_each(|line| {
    //     for l in line {
    //         print!("{}", l);
    //     }
    //     println!("");
    // });
    "EKRHEPUZ".to_string()
}

type InputType = Vec<Instruction>;
fn parse_input(raw_input: &[String]) -> InputType {
    raw_input.iter().flat_map(|x| {
        if let Some(addx) = x.strip_prefix("addx ") {
            vec![Instruction::sety(addx.parse().unwrap()), Instruction::addxy]
        } else {
            vec![Instruction::noop]
        }
    }).collect()
}

pub fn solve(raw_input: &[String]) -> Solution {
    let input = parse_input(raw_input);

    use std::time::Instant;
    let now = Instant::now();
    let solution = (part1(&input), part2(&input));
    let elapsed = now.elapsed();
    (solution, elapsed)
}
