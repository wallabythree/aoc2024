use crate::Solver;

mod day01;
mod day02;
mod day03;
mod day04;

pub fn get_solution(day: usize) -> &'static dyn Solver {
    match day {
        1 => &day01::SOLUTION,
        2 => &day02::SOLUTION,
        3 => &day03::SOLUTION,
        4 => &day04::SOLUTION,
        _ => unimplemented!(),
    }
}
