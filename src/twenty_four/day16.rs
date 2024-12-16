use crate::Solution;
use crate::util::point::{ Direction::*, Point };
use std::collections::{ BinaryHeap, HashMap, HashSet };
use std::cmp::Reverse;

pub const SOLUTION: Solution<Cost, Cost> = Solution { part1, part2 };

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

        if self.nodes.contains(&neighbour) {
            Some(neighbour)
        } else {
            None
        }
    }

    fn dijkstra(&self, start: Point<i64>) -> HashMap<Point<i64>, Cost> {
        let mut queue = BinaryHeap::new();
        queue.push(Reverse((0, start, East.into())));

        let mut d: HashMap<Point<i64>, Cost> = self
            .nodes
            .iter()
            .map(|&node| (node, Cost::MAX))
            .collect();

        let mut f: HashMap<(Point<i64>, Point<i64>), Cost> = HashMap::new();

        d.insert(start, 0);
        f.insert((start, East.into()), 0);

        while let Some(Reverse((cost, node, dir))) = queue.pop() {
            println!("{:?}", node);

            f.insert((node, dir), cost);

            let edges = [
                (self.neighbour(node, dir), dir, 1),
                (Some(node), dir.rotate_left(), 1000),
                (Some(node), dir.rotate_right(), 1000)
            ];

            for (neighbour_opt, n_dir, n_cost) in edges {
                let new_cost = cost + n_cost;

                if let Some(neighbour) = neighbour_opt {
                    if f.contains_key(&(neighbour, n_dir)) {
                        continue;
                    }

                    if let Some(&current_cost) = d.get(&neighbour) {
                        if current_cost > new_cost {
                            d.insert(neighbour, new_cost);
                        }

                        queue.retain(|&Reverse((_, node, dir))| {
                            node != neighbour || dir != n_dir
                        });
                        queue.push(Reverse((new_cost, neighbour, n_dir)));
                    }
                }
            }
        }

        d
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
        let end = end_opt.ok_or("Start not found")?;

        Ok( Self { nodes, start, end } )
    }
}

fn part1(input: &str) -> Cost {
    let graph: Graph = Graph::try_from(input).unwrap();
    println!("{:?}", graph);

    let result = graph.dijkstra(graph.start);
    let end_cost = result.get(&graph.end);
    println!("dijkstra: {:?}", end_cost);

    *end_cost.unwrap()
}

fn part2(input: &str) -> Cost {
    0
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
        assert_eq!(part2(TEST_INPUT_1), 0);
    }
}
