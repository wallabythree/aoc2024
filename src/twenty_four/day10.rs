use crate::Solution;
use std::collections::HashSet;

pub const SOLUTION: Solution<usize, usize> = Solution { part1, part2 };

type Point = (usize, usize);
type Dir = (isize, isize);

struct TopoMap {
    grid: Vec<Vec<usize>>,
    trailheads: Vec<(usize, usize)>,
}

impl TopoMap {
    fn height(&self) -> usize {
        self.grid.len()
    }

    fn width(&self) -> usize {
        match self.height() {
            0 => 0,
            _ => self.grid[0].len(),
        }
    }

    fn validate_move(&self, pos: Point, dir: Dir) -> Option<(usize, usize)> {
        let (x, y) = pos;
        let (dx, dy) = dir;

        x
            .checked_add_signed(dx)
            .and_then(|x| (x < self.width()).then_some(x))
            .and_then(|x| y
                .checked_add_signed(dy)
                .and_then(|y| (y < self.height()).then_some((x, y)))
            )
    }

    fn is_gentle_climb(&self, src: Point, dst: Point) -> bool {
        self
            .grid[dst.1][dst.0]
            .checked_sub(self.grid[src.1][src.0])
            .is_some_and(|diff| diff == 1)
    }

    fn dfs(&self, pos: Point, visited: &mut HashSet<(usize, usize)>) -> usize {
        if visited.contains(&pos) {
            return 0;
        }

        let (x, y) = pos;
        let height = self.grid[y][x];

        if height == 9 {
            visited.insert(pos);
            return 1;
        }

        let dirs = [(0, -1), (1, 0), (0, 1), (-1, 0)];

        dirs
            .iter()
            .filter_map(|&dir| self.validate_move(pos, dir))
            .filter(|&next_pos| self.is_gentle_climb(pos, next_pos))
            .map(|next_pos| self.dfs(next_pos, visited))
            .sum()
    }

    fn dfs2(&self, pos: Point) -> usize {
        let (x, y) = pos;
        let height = self.grid[y][x];

        if height == 9 {
            return 1;
        }

        let dirs = [(0, -1), (1, 0), (0, 1), (-1, 0)];

        dirs
            .iter()
            .filter_map(|&dir| self.validate_move(pos, dir))
            .filter(|&next_pos| self.is_gentle_climb(pos, next_pos))
            .map(|next_pos| self.dfs2(next_pos))
            .sum()
    }

    fn from(input: &str) -> Self {
        let grid: Vec<Vec<_>> = input
            .trim()
            .lines()
            .map(|l| l
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect())
            .collect();

        let trailheads: Vec<_> = grid
            .iter()
            .enumerate()
            .flat_map(|(y, r)| r
                .iter()
                .enumerate()
                .filter_map(|(x, h)| (*h == 0).then_some((x, y)))
                .collect::<Vec<_>>()
            )
            .collect();

        Self { grid, trailheads }
    }
}

fn part1(input: &str) -> usize {
    let topo_map = TopoMap::from(input);

    topo_map
        .trailheads
        .iter()
        .map(|&pos| topo_map.dfs(pos, &mut HashSet::new()))
        .sum()
}

fn part2(input: &str) -> usize {
    let topo_map = TopoMap::from(input);

    topo_map
        .trailheads
        .iter()
        .map(|&pos| topo_map.dfs2(pos))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 36);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 81);
    }
}
