use crate::Solution;
use std::collections::{ HashSet, VecDeque };

pub const SOLUTION: Solution<usize, usize> = Solution { part1, part2 };

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Point(usize, usize);

impl Point {
    fn checked_add(&self, dir: Dir) -> Option<Self> {
        let (x, y) = (self.0, self.1);
        let (dx, dy) = (dir.0, dir.1);

        x
            .checked_add_signed(dx)
            .and_then(|x| y
                .checked_add_signed(dy)
                .map(|y| Self(x, y)))
    }
}

type Dir = (isize, isize);

const VERTICE_DIRS: [Dir; 4] = [(-1, -1), (1, -1), (1, 1), (-1, 1)];

#[derive(Debug)]
struct Garden {
    grid: Vec<Vec<char>>
}

impl Garden {
    fn from(input: &str) -> Self {
        let grid = input
            .lines()
            .map(|l| l.chars().collect())
            .collect();

        Self { grid }
    }

    fn height(&self) -> usize {
        self.grid.len()
    }

    fn width(&self) -> usize {
        if self.height() == 0 { 0 } else { self.grid[0].len() }
    }

    fn try_move(&self, pos: Point, dir: Dir) -> Option<Point> {
        pos
            .checked_add(dir)
            .filter(|pos| pos.0 < self.width() && pos.1 < self.height())
    }

    fn neighbours(&self, pos: Point) -> Vec<Point> {
        let dirs = [(0, -1), (1, 0), (0, 1), (-1, 0)];

        dirs.iter().filter_map(|&dir| self.try_move(pos, dir)).collect()
    }

    fn is_vertex(&self, pos: Point, dir: Dir) -> bool {
        let plant = self.grid[pos.1][pos.0];

        let opposite = self
            .try_move(pos, dir)
            .map(|opposite_pos| self.grid[opposite_pos.1][opposite_pos.0]);

        let adjacent_x = self
            .try_move(pos, (dir.0, 0))
            .map(|opposite_pos| self.grid[opposite_pos.1][opposite_pos.0]);

        let adjacent_y = self
            .try_move(pos, (0, dir.1))
            .map(|opposite_pos| self.grid[opposite_pos.1][opposite_pos.0]);

        if let Some(opposite) = opposite {
            let adjacent_x = adjacent_x.unwrap();
            let adjacent_y = adjacent_y.unwrap();

            if opposite == plant {
                adjacent_x != plant && adjacent_y != plant
            } else {
                (adjacent_x == plant && adjacent_y == plant) ||
                (adjacent_x != plant && adjacent_y != plant)
            }
        } else if let Some(adjacent_x) = adjacent_x {
            adjacent_x != plant
        } else if let Some(adjacent_y) = adjacent_y {
                adjacent_y != plant
        } else {
            true
        }
    }

    fn bfs(
        &self,
        start: Point,
        visited: &mut HashSet<Point>
    ) -> (usize, usize, usize) {
        let mut area = 0;
        let mut perimeter = 0;

        let mut vertices = 0;

        let mut queue = VecDeque::new();
        queue.push_back(start);

        while let Some(pos) = queue.pop_front() {
            visited.insert(pos);

            for &v in VERTICE_DIRS.iter() {
                if self.is_vertex(pos, v) {
                    vertices += 1;
                }
            }

            let (x, y) = (pos.0, pos.1);
            let plant = self.grid[y][x];

            area += 1;

            let neighbours: Vec<Point> = self
                .neighbours(pos)
                .iter()
                .filter(|n| self.grid[n.1][n.0] ==  plant)
                .copied()
                .collect();

            perimeter += 4 - neighbours.len();

            for n in neighbours {
                if !visited.contains(&n) && !queue.contains(&n) {
                    queue.push_back(n);
                }
            }
        }

        (area, perimeter, vertices)
    }

    fn total_fence_price(&self) -> (usize, usize) {
        let mut visited = HashSet::new();
        let mut perimeter_cost = 0;
        let mut sides_cost = 0;

        for y in 0..self.height() {
            for x in 0..self.width() {
                let pos = Point(x, y);

                if !visited.contains(&pos) {
                    let (area, perimeter, sides) = self.bfs(pos, &mut visited);
                    perimeter_cost += area * perimeter;
                    sides_cost += area * sides;
                }
            }
        }

        (perimeter_cost, sides_cost)
    }
}

fn part1(input: &str) -> usize {
    let garden = Garden::from(input);

    garden.total_fence_price().0
}

fn part2(input: &str) -> usize {
    let garden = Garden::from(input);

    garden.total_fence_price().1
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT_SMALL: &str = "AAAA
BBCD
BBCC
EEEC
";

    const TEST_INPUT_XO: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
";

    const TEST_INPUT_E_SHAPED: &str = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
";

    const TEST_INPUT_AB: &str = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
";

    const TEST_INPUT_LARGE: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT_SMALL), 140);
        assert_eq!(part1(TEST_INPUT_XO), 772);
        assert_eq!(part1(TEST_INPUT_LARGE), 1930);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_SMALL), 80);
        assert_eq!(part2(TEST_INPUT_XO), 436);
        assert_eq!(part2(TEST_INPUT_E_SHAPED), 236);
        assert_eq!(part2(TEST_INPUT_AB), 368);
        assert_eq!(part2(TEST_INPUT_LARGE), 1206);
    }
}
