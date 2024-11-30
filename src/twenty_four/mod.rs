use crate::Solver;

mod day00;
mod day01;

pub fn get_solution(day: usize) -> &'static dyn Solver {
    match day {
        0 => &day00::SOLUTION,
        1 => &day01::SOLUTION,
        _ => unimplemented!(),
    }
}

