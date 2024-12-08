use crate::Solution;
use std::hash::Hash;
use std::ops::{ Add, Neg, Sub };
use std::collections::{ HashMap, HashSet };

pub const SOLUTION: Solution<usize, usize> = Solution { part1, part2 };

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Point {
    x: isize,
    y: isize,
}

impl Neg for Point {
    type Output = Self;

    fn neg(self) -> Self {
        Self { x: -self.x, y: -self.y }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { x: self.x + other.x, y: self.y + other.y }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self { x: self.x - other.x, y: self.y - other.y }
    }
}

struct AntennaMap<T: Hash> {
    by_freq: HashMap<T, Vec<Point>>,
    width: isize,
    height: isize,
}

impl<T: Hash> AntennaMap<T> {
    fn in_bounds(&self, pos: Point) -> bool {
        pos.x >= 0 && pos.x < self.width && pos.y >= 0 && pos.y < self.height
    }
}

impl From<&str> for AntennaMap<char> {
    fn from(input: &str) -> Self {
        let mut by_freq: HashMap<char, Vec<Point>> = HashMap::new();

        let width = input.find('\n').unwrap() as isize;
        let height = input.len() as isize / width - 1;

        for (y, l) in input.lines().enumerate() {
            for (x, c) in l.chars().enumerate() {
                if c != '.' {
                    let pos = Point { x: x as isize, y: y as isize };
                    by_freq.entry(c).or_default().push(pos);
                }
            }
        }

        Self { by_freq, width, height }
    }
}

fn part1(input: &str) -> usize {
    let antenna_map = AntennaMap::from(input);
    let mut antinode_locations = HashSet::new();

    for (_, antennae) in antenna_map.by_freq.iter() {
        for (i, a) in antennae.iter().enumerate() {
            for b in &antennae[i + 1..] {
                let dir = *b - *a;

                for an in [*a - dir, *b + dir] {
                    if antenna_map.in_bounds(an) {
                        antinode_locations.insert(an);
                    }
                }
            }
        }
    }

    antinode_locations.len()
}

fn part2(input: &str) -> usize {
    let antenna_map = AntennaMap::from(input);
    let mut antinode_locations = HashSet::new();

    for (_, antennae) in antenna_map.by_freq.iter() {
        for (i, a) in antennae.iter().enumerate() {
            for b in &antennae[i + 1..] {
                let delta = *b - *a;

                for dir in [-delta, delta] {
                    let mut an = *a;

                    while antenna_map.in_bounds(an) {
                        antinode_locations.insert(an);
                        an = an + dir;
                    }
                }
            }
        }
    }

    antinode_locations.len()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 14);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 34);
    }
}
