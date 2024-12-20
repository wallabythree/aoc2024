use crate::Solution;
use crate::util::point::{ Direction::*, Grid, Point };
use std::collections::{ BTreeMap, HashMap, VecDeque };

pub const SOLUTION: Solution<usize, usize> = Solution { part1, part2 };

#[derive(Debug)]
struct Racetrack {
    grid: Grid<char>,
    start: Point<i64>,
    end: Point<i64>,
}

impl Racetrack {
    fn neighbours(&self, pos: Point<i64>) -> Vec<Point<i64>> {
        [North, East, South, West]
            .iter()
            .map(|&dir| pos + dir.into())
            .filter(|&pos| self.grid.in_bounds(pos))
            .collect()
    }

    fn bfs(
        &self,
        start: Point<i64>,
        goal: Point<i64>,
        max_depth: Option<usize>,
        cheat: bool,
    ) -> Option<HashMap<Point<i64>, usize>> {
        let mut level = VecDeque::new();
        let mut frontier = VecDeque::new();
        let mut visited = HashMap::new();
        let mut depth = 0;

        level.push_back(start);

        while let Some(node) = level.pop_front() {
            visited.insert(node, depth);

            if node == goal {
                return Some(visited);
            }

            let neighbours: Vec<Point<i64>> = self
                .neighbours(node)
                .iter()
                .copied()
                .filter(|&pos| {
                    let &c = self.grid.get(pos).unwrap();
                    pos == goal || (if cheat { c == '#' } else { c != '#' })
                })
                .collect();

            for neighbour in neighbours {
                if visited.contains_key(&neighbour) {
                    continue;
                }

                frontier.push_back(neighbour);
            }

            if level.is_empty() {
                if max_depth.is_some_and(|max_depth| depth == max_depth) {
                    return None;
                }

                level = frontier;
                frontier = VecDeque::new();
                depth += 1;
            }
        }

        None
    }

    fn find_cheats(
        &self,
        cheat_depth: usize
    ) -> Option<BTreeMap<usize, usize>> {
        let path = self.bfs(self.start, self.end, None, false)?;
        let full_cost = path.get(&self.end).copied()?;

        let mut cheats: BTreeMap<usize, usize> =
            BTreeMap::new();

        for (&cheat_start, &start_depth) in &path {
            for (&cheat_end, &end_depth) in &path {
                if end_depth <= start_depth {
                    continue;
                }

                let manhattan = cheat_start.manhattan(cheat_end) as usize;

                if manhattan <= cheat_depth {
                    let cheat_cost = full_cost
                        - end_depth
                        + manhattan
                        + start_depth;

                    if cheat_cost < full_cost {
                        *cheats.entry(full_cost - cheat_cost).or_default() += 1;
                    }
                }
            }
        }
        Some(cheats)
    }

    fn count_cheats(
        &self,
        cheat_depth: usize,
        minimum_savings: usize
    ) -> Option<usize> {
        let cheats = self.find_cheats(cheat_depth)?;

        let count = cheats
            .range(minimum_savings..)
            .map(|(_, cheats)| cheats)
            .sum();

        Some(count)
    }
}

impl TryFrom<&str> for Racetrack {
    type Error = Box<dyn std::error::Error>;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut start_opt = None;
        let mut end_opt = None;

        let tiles = input
            .lines()
            .enumerate()
            .map(|(y, row)| row
                .chars()
                .enumerate()
                .map(|(x, c)| {
                    match c {
                        'S' => start_opt = Some((x as i64, y as i64).into()),
                        'E' => end_opt = Some((x as i64, y as i64).into()),
                        _   => (),
                    }

                    c
                })
                .collect::<Vec<_>>()
            )
            .collect::<Vec<Vec<_>>>();

        let grid = Grid { tiles };
        let start = start_opt.ok_or("Start not found")?;
        let end = end_opt.ok_or("End not found")?;

        Ok(Self { grid, start, end })
    }
}

fn part1(input: &str) -> usize {
    let track = Racetrack::try_from(input).unwrap();
    track.count_cheats(2, 100).unwrap()
}

fn part2(input: &str) -> usize {
    let track = Racetrack::try_from(input).unwrap();
    track.count_cheats(20, 100).unwrap()
}

#[cfg(test)]
mod tests {
    use super::Racetrack;

    const TEST_INPUT: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

    #[test]
    fn test_part1() {
        let track = Racetrack::try_from(TEST_INPUT).unwrap();
        assert_eq!(track.count_cheats(2, 64).unwrap(), 1);
    }

    #[test]
    fn test_part2() {
        let track = Racetrack::try_from(TEST_INPUT).unwrap();
        assert_eq!(track.count_cheats(20, 76).unwrap(), 3);
    }
}
