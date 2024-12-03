use crate::Solver;

mod day00;
mod day01;
mod day02;
mod day03;

pub fn get_solution(day: usize) -> &'static dyn Solver {
    match day {
        0 => &day00::SOLUTION,
        1 => &day01::SOLUTION,
        2 => &day02::SOLUTION,
        3 => &day03::SOLUTION,
        _ => unimplemented!(),
    }
}
