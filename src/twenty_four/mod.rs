use crate::Solver;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;

pub fn get_solution(day: usize) -> &'static dyn Solver {
    match day {
        1 => &day01::SOLUTION,
        2 => &day02::SOLUTION,
        3 => &day03::SOLUTION,
        4 => &day04::SOLUTION,
        5 => &day05::SOLUTION,
        7 => &day07::SOLUTION,
        8 => &day08::SOLUTION,
        9 => &day09::SOLUTION,
        10 => &day10::SOLUTION,
        11 => &day11::SOLUTION,
        12 => &day12::SOLUTION,
        13 => &day13::SOLUTION,
        _ => unimplemented!(),
    }
}
