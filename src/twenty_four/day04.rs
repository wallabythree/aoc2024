use crate::Solution;
use std::ops::{ Add, Mul };

pub const SOLUTION: Solution<usize, usize> = Solution { part1, part2 };

#[derive(Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul<isize> for Point {
    type Output = Self;

    fn mul(self, other: isize) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

fn grid_get<'a, T>(grid: &[&'a [T]], pos: Point) -> Option<&'a T> {
    let height = grid.len().try_into().expect("Grid height overflow");
    let width = grid[0].len().try_into().expect("Grid height overflow");

    if pos.x >= 0 && pos.x < width && pos.y >= 0 && pos.y < height {
        Some(&grid[pos.y as usize][pos.x as usize])
    } else {
        None
    }

}

fn directed_search<T: Eq>(
    grid: &[&[T]],
    pattern: &[T],
    pos: Point,
    delta: Point,
) -> bool {
    if pattern.is_empty() {
        return true;
    }

    match grid_get(grid, pos) {
        Some(e) if *e == pattern[0] => {
            directed_search(grid, &pattern[1..], pos + delta, delta) // recurse
        },
        _ => false
    }
}

fn search<T: Eq>(grid: &[&[T]], pattern: &[T], start: Point) -> usize {
    let north = Point { x: 0, y: -1 };
    let east  = Point { x: 1, y: 0 };
    let south = Point { x: 0, y: 1 };
    let west  = Point { x: -1, y: 0 };

    let directions = [
        directed_search(grid, pattern, start, north),
        directed_search(grid, pattern, start, north + east),
        directed_search(grid, pattern, start, east),
        directed_search(grid, pattern, start, east + south),
        directed_search(grid, pattern, start, south),
        directed_search(grid, pattern, start, south + west),
        directed_search(grid, pattern, start, west),
        directed_search(grid, pattern, start, west + north),
    ];

    directions.iter().filter(|d| **d).count()
}

fn is_cross<T: Eq>(grid: &[&[T]], pattern: &[T], start: Point) -> bool {
    let north_east = Point { x: 1, y: -1 };
    let south_east = Point { x: 1, y: 1 };
    let south_west = Point { x: -1, y: 1 };
    let north_west = Point { x: -1, y: -1 };

    let mid: isize = pattern.len() as isize / 2;

    let directions = [
        directed_search(grid, pattern, start + (south_west * mid), north_east),
        directed_search(grid, pattern, start + (north_west * mid), south_east),
        directed_search(grid, pattern, start + (north_east * mid), south_west),
        directed_search(grid, pattern, start + (south_east * mid), north_west),
    ];

    (directions[0] || directions[2]) && (directions[1] || directions[3])
}

fn parse_grid(input: &str) -> Vec<&[u8]> {
    input
        .lines()
        .map(|l| l.as_bytes())
        .collect()
}

fn part1(input: &str) -> usize {
    let grid = parse_grid(input);
    let mut count = 0;

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let pos = Point { x: x as isize, y: y as isize };

            count += search(&grid, "XMAS".as_bytes(), pos);
        }
    }

    count
}

fn part2(input: &str) -> usize {
    let grid = parse_grid(input);
    let mut count = 0;

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let pos = Point { x: x as isize, y: y as isize };

            if is_cross(&grid, "MAS".as_bytes(), pos) {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_search() {
        let grid = parse_grid(TEST_INPUT);
        let count = search(&grid, "XMAS".as_bytes(), Point { x: 4, y: 1 });

        assert_eq!(count, 1);
    }

    #[test]
    fn test_search_part2() {
        let grid = parse_grid(TEST_INPUT);
        assert!(is_cross(&grid, "MAS".as_bytes(), Point { x: 7, y: 2 }));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 9);
    }
}
