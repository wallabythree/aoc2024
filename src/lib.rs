use std::fmt::Display;

mod twenty_four;

pub trait Solver {
    fn part1(&self, input: &str) -> Box<dyn Display>;
    fn part2(&self, input: &str) -> Box<dyn Display>;
}

pub struct Solution<T: Display, U: Display> {
    part1: fn(&str) -> T,
    part2: fn(&str) -> U,
}

impl<T: Display + 'static, U: Display + 'static> Solver for Solution<T, U> {
    fn part1(&self, input: &str) -> Box<dyn Display> {
        Box::new((self.part1)(input))
    }

    fn part2(&self, input: &str) -> Box<dyn Display> {
        Box::new((self.part2)(input))
    }
}

pub fn get_solution(year: usize, day: usize) -> &'static dyn Solver {
    match year {
        2024 => twenty_four::get_solution(day),
        _ => panic!(),
    }
}

