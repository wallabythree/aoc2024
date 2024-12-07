use crate::Solver;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day07;

pub fn get_solution(day: usize) -> &'static dyn Solver {
    match day {
        1 => &day01::SOLUTION,
        2 => &day02::SOLUTION,
        3 => &day03::SOLUTION,
        4 => &day04::SOLUTION,
        5 => &day05::SOLUTION,
        7 => &day07::SOLUTION,
        _ => unimplemented!(),
    }
}
