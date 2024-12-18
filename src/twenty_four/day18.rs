use std::collections::{HashSet, VecDeque};
use std::error::Error;

use crate::Solution;
use crate::util::point::{ Direction::*, Point };

pub const SOLUTION: Solution<usize, String> = Solution { part1, part2 };

#[derive(Debug)]
struct Memory {
    fallen: HashSet<Point<u64>>,
    falling: VecDeque<Point<u64>>,
    size: Point<u64>,
}

impl Memory {
    fn tick(&mut self) {
        self.falling.pop_front().map(|f| self.fallen.insert(f));
    }

    fn advance(&mut self, ticks: usize) {
        for _ in 0..ticks {
            self.tick();
        }
    }

    fn in_bounds(&self, p: Point<u64>) -> bool {
        p.x <= self.size.x &&
        p.y <= self.size.y
    }

    fn neighbours(&self, p: Point<u64>) -> Vec<Point<u64>> {
        [North, East, South, West]
            .iter()
            .filter_map(|&d| p.checked_add::<i64>(d.into()))
            .filter(|&n_p| self.in_bounds(n_p))
            .filter(|n_p| !self.fallen.contains(n_p))
            .collect()
    }

    fn bfs(&self, start: Point<u64>, end: Point<u64>) -> Option<usize> {
        let mut level = VecDeque::new();
        let mut frontier = VecDeque::new();
        let mut visited = HashSet::new();

        level.push_back(start);

        let mut cost = 0;

        while let Some(node) = level.pop_front() {
            //println!("visiting: {:?}\t cost: {:?}", node, cost);

            visited.insert(node);

            if node == end {
                return Some(cost);
            }

            for neighbour in self.neighbours(node) {
                if visited.contains(&neighbour) || level.contains(&neighbour) || frontier.contains(&neighbour) {
                    continue;
                }

                frontier.push_back(neighbour);
            }

            if level.is_empty() {
                level = frontier;
                frontier = VecDeque::new();

                cost += 1;
            }
        }

        None
    }

    fn solve(&self) -> Option<usize> {
        self.bfs((0,0).into(), self.size)
    }

    fn solve2(&mut self) -> Point<u64> {
        let mut byte = None;

        while let Some(_) = self.solve() {
            byte = Some(self.falling.front().unwrap().clone());
            self.tick();
        }

        byte.unwrap()
    }
}

impl TryFrom<(&str, Point<u64>)> for Memory {
    type Error = Box<dyn Error>;

    fn try_from(
        (input, size): (&str, Point<u64>)
    ) -> Result<Self, Self::Error> {

        let falling: VecDeque<Point<_>> = input
            .lines()
            .map(|l| l
                .split_once(',')
                .map(|(x,y)| {
                    let pos = (x.parse()?, y.parse()?);
                    Ok::<_, Box<dyn Error>>(pos.into())
                })
                .ok_or("Parse error")?
            )
            .collect::<Result<VecDeque<_>, _>>()?;

        let memory = Memory {
            fallen: HashSet::new(),
            falling,
            size,
        };

        Ok(memory)
    }
}

fn part1(input: &str) -> usize {
    let mut memory = Memory::try_from((input, (70, 70).into())).unwrap();
    memory.advance(1024);
    memory.solve().unwrap()
}

fn part2(input: &str) -> String {
    let mut memory = Memory::try_from((input, (70, 70).into())).unwrap();
    let byte = memory.solve2();

    format!("{},{}", byte.x, byte.y)
}

#[cfg(test)]
mod tests {
    use super::Memory;

    const TEST_INPUT: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";

    #[test]
    fn test_part1() {
        let mut memory = Memory::try_from((TEST_INPUT, (6, 6).into())).unwrap();
        memory.advance(12);
        assert_eq!(memory.solve().unwrap(), 22);
    }

    #[test]
    fn test_part2() {
        let mut memory = Memory::try_from((TEST_INPUT, (6, 6).into())).unwrap();
        let byte = memory.solve2();

        assert_eq!(format!("{},{}", byte.x, byte.y), "6,1".to_string());
    }
}
