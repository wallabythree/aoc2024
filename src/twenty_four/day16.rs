use crate::Solution;
use crate::util::point::{ Direction::*, Point };
use std::collections::{ BinaryHeap, HashMap, HashSet, VecDeque };
use std::cmp::Reverse;

pub const SOLUTION: Solution<Cost, usize> = Solution { part1, part2 };

type Cost = u64;

#[derive(Debug)]
struct Graph {
    nodes: HashSet<Point<i64>>,
    start: Point<i64>,
    end: Point<i64>,
}

impl Graph {
    fn neighbour(&self, pos: Point<i64>, dir: Point<i64>) -> Option<Point<i64>> {
        let neighbour = pos + dir;
        self.nodes.get(&neighbour).copied()
    }

    fn dijkstra(&self, start: Point<i64>) -> (HashMap<(Point<i64>, Point<i64>), Cost>, HashMap<(Point<i64>, Point<i64>), HashSet<(Point<i64>, Point<i64>)>>) {
        let mut queue = BinaryHeap::new();
        queue.push(Reverse((0, start, East.into())));

        let mut costs: HashMap<(Point<i64>, Point<i64>), Cost> = self
            .nodes
            .iter()
            .flat_map(|&node|
                [North.into(), East.into(), South.into(), West.into()]
                    .iter()
                    .map(|&dir| ((node, dir), Cost::MAX)).collect::<Vec<_>>()
            )
            .collect();

        let mut visited: HashSet<(Point<i64>, Point<i64>)> = HashSet::new();
        let mut prev: HashMap<(Point<i64>, Point<i64>), HashSet<(Point<i64>, Point<i64>)>> = HashMap::new();

        costs.insert((start, East.into()), 0);

        while let Some(Reverse((cost, node, dir))) = queue.pop() {
            if visited.contains(&(node, dir)) {
                continue;
            }
            visited.insert((node, dir));

            let edges = [
                (self.neighbour(node, dir), dir, 1),
                (Some(node), dir.rotate_left(), 1000),
                (Some(node), dir.rotate_right(), 1000)
            ];

            for (neighbour_opt, n_dir, n_cost) in edges {
                let new_cost = cost + n_cost;

                if let Some(neighbour) = neighbour_opt {
                    let &current_cost = costs.get(&(neighbour, n_dir)).unwrap();

                    if new_cost < current_cost {
                        costs.insert((neighbour, n_dir), new_cost);
                    }

                    if new_cost <= current_cost {
                        prev.entry((neighbour, n_dir)).or_default().insert((node, dir));
                    }

                    queue.push(Reverse((new_cost, neighbour, n_dir)));
                }
            }
        }

        (costs, prev)
    }

    fn shortest_path_len(&self) -> Cost {
        let (costs, _) = self.dijkstra(self.start);

        costs
            .iter()
            .filter(|(&(node, _), _)| node == self.end)
            .map(|(_, &cost)| cost)
            .min()
            .unwrap()
    }

    fn shortest_paths(&self) -> usize {
        let (costs, prevs) = self.dijkstra(self.start);

        let mut ends: Vec<_> = costs
            .iter()
            .filter(|(&(node, _), _)| node == self.end)
            .map(|((_, dir), cost)| (cost, dir))
            .collect();

        ends.sort();
        let shortest_path_len = *ends[0].0;

        let dirs: HashSet<Point<i64>> = ends
            .iter()
            .take_while(|(&cost, _)| cost == shortest_path_len)
            .map(|(_, &dir)| dir)
            .collect();

        let mut queue: VecDeque<(Point<i64>, Point<i64>)> = prevs
            .iter()
            .filter(|(&(node, dir), _)| node == self.end && dirs.contains(&dir))
            .flat_map(|(_, prev)| prev.clone())
            .collect();

        let mut edges = HashSet::new();

        while let Some((node, dir)) = queue.pop_front() {
            if edges.contains(&(node, dir)) {
                continue;
            }
            edges.insert((node, dir));

            if let Some(to_queue) = prevs.get(&(node, dir)) {
                for &prev in to_queue {
                    queue.push_back(prev);
                }
            }
        }

        let nodes: HashSet<Point<i64>> = edges
            .iter()
            .map(|&(node, _)| node)
            .collect();

        nodes.len() + 1
    }
}

impl TryFrom<&str> for Graph {
    type Error = Box<dyn std::error::Error>;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut nodes = HashSet::new();
        let mut start_opt = None;
        let mut end_opt = None;

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let x = x.try_into().map_err(|_| "Conversion error")?;
                let y = y.try_into().map_err(|_| "Conversion error")?;

                if c != '#' {
                    nodes.insert((x, y).into());
                }

                match c {
                    'S' => start_opt = Some(Point::from((x, y))),
                    'E' => end_opt = Some(Point::from((x, y))),
                    _   => (),
                }
            }
        }

        let start = start_opt.ok_or("Start not found")?;
        let end = end_opt.ok_or("End not found")?;

        Ok( Self { nodes, start, end } )
    }
}

fn part1(input: &str) -> Cost {
    let graph: Graph = Graph::try_from(input).unwrap();
    graph.shortest_path_len()
}

fn part2(input: &str) -> usize {
    let graph: Graph = Graph::try_from(input).unwrap();
    graph.shortest_paths()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT_1: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

    const TEST_INPUT_2: &str = "#################
 #...#...#...#..E#
 #.#.#.#.#.#.#.#.#
 #.#.#.#...#...#.#
 #.#.#.#.###.#.#.#
 #...#.#.#.....#.#
 #.#.#.#.#.#####.#
 #.#...#.#.#.....#
 #.#.#####.#.###.#
 #.#.#.......#...#
 #.#.###.#####.###
 #.#.#...#.....#.#
 #.#.#.#####.###.#
 #.#.#.........#.#
 #.#.#.#########.#
 #S#.............#
 #################
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT_1), 7036);
        assert_eq!(part1(TEST_INPUT_2), 11048);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_1), 45);
        assert_eq!(part2(TEST_INPUT_2), 64);
    }
}
