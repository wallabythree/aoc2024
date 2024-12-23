use crate::Solution;
use crate::util::point::Point;
use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;

pub const SOLUTION: Solution<usize, usize> = Solution { part1, part2 };

impl Point<i64> {
    fn gps(&self) -> i64 {
        self.y * 100 + self.x
    }
}

impl TryFrom<char> for Point<i64> {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '^' => Ok((0, -1).into()),
            '>' => Ok((1, 0).into()),
            'v' => Ok((0, 1).into()),
            '<' => Ok((-1, 0).into()),
            _ => Err(format!("Invalid direction: '{}'", c)),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum BoxType {
    Single,
    Left,
    Right,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Obstacle {
    Wall,
    Box(BoxType),
}

impl TryFrom<char> for Obstacle {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '#' => Ok(Obstacle::Wall),
            'O' => Ok(Obstacle::Box(BoxType::Single)),
            '[' => Ok(Obstacle::Box(BoxType::Left)),
            ']' => Ok(Obstacle::Box(BoxType::Right)),
            _ => Err(()),
        }
    }
}

struct Warehouse {
    robot: Point<i64>,
    obstacles: HashMap<i64, HashMap<i64, Obstacle>>,
    width: usize,
    height: usize,
}

impl Warehouse {
    fn attempt_move(&mut self, dir: Point<i64>) {
        let can_move = self.can_move(self.robot, dir);

        if can_move {
            self.execute_move(self.robot, dir);
            self.robot = self.robot + dir;
        }
    }

    fn can_move(&self, pos: Point<i64>, dir: Point<i64>) -> bool {
        let next_pos = pos + dir;
        let next = self
            .obstacles
            .get(&next_pos.y)
            .and_then(|xs| xs.get(&next_pos.x));

        match next {
            None => true,
            Some(Obstacle::Wall) => false,
            _ if dir.y == 0 => self.can_move(next_pos, dir),
            Some(Obstacle::Box(BoxType::Left)) => {
                let left = self.can_move(next_pos, dir);
                let right = self.can_move(next_pos + (1, 0).into(), dir);

                left && right
            },
            Some(Obstacle::Box(BoxType::Right)) => {
                let left = self.can_move(next_pos + (-1, 0).into(), dir);
                let right = self.can_move(next_pos, dir);

                left && right
            },
            _ => self.can_move(next_pos, dir),
        }
    }

    fn execute_move(&mut self, pos: Point<i64>, dir: Point<i64>) {
        let next_pos = pos + dir;
        let next = self
            .obstacles
            .get(&next_pos.y)
            .and_then(|xs| xs.get(&next_pos.x));

        match next {
            None => (),
            Some(Obstacle::Wall) => return,
            _ if dir.y == 0 => self.execute_move(next_pos, dir),
            Some(Obstacle::Box(BoxType::Left)) => {
                self.execute_move(next_pos, dir);
                self.execute_move(next_pos + (1,0).into(), dir);
            },
            Some(Obstacle::Box(BoxType::Right)) => {
                self.execute_move(next_pos, dir);
                self.execute_move(next_pos + (-1,0).into(), dir);
            },
            _ => self.execute_move(next_pos, dir),
        }

        self
            .obstacles
            .get_mut(&pos.y)
            .and_then(|xs| xs.remove(&pos.x))
            .and_then(|o| self
                .obstacles
                .entry(next_pos.y)
                .or_default()
                .insert(next_pos.x, o)
            );
    }

    fn gps_sum(&self) -> i64 {
        self
            .obstacles
            .iter()
            .flat_map(|(y, xs)| xs
                .iter()
                .filter_map(|(x, o)| match o {
                    Obstacle::Box(BoxType::Single) =>
                        Some(Point { x: *x, y: *y }.gps()),
                    Obstacle::Box(BoxType::Left) =>
                        Some(Point { x: *x, y: *y }.gps()),
                    _ => None,
                })
            )
            .sum()
    }
}

impl TryFrom<&str> for Warehouse {
    type Error = Box<dyn std::error::Error>;

    fn try_from(map_str: &str) -> Result<Self, Self::Error> {
        let width = map_str.find('\n').ok_or("Invalid map format")?;
        let height = map_str.len() / width;

        let mut robot_pos = None;

        let obstacles: HashMap<i64, HashMap<i64, Obstacle>> = map_str
            .lines()
            .enumerate()
            .map(|(y, l)| {
                let row = l
                    .chars()
                    .enumerate()
                    .filter_map(|(x, c)| {
                        let x = x.try_into().unwrap();
                        let y = y.try_into().unwrap();

                        if let Ok(o) = Obstacle::try_from(c) {
                            Some((x, o))
                        } else {
                            if c == '@' {
                                robot_pos = Some((x, y).into())
                            }

                            None
                        }
                    })
                    .collect();

                (y.try_into().unwrap(), row)
            })
            .collect();

        let robot = robot_pos.ok_or("Robot not found")?;

        Ok(Self { robot, obstacles, width, height })
    }
}

impl Debug for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut v = vec![vec!['.'; self.width]; self.height];

        for (&y, xs) in &self.obstacles {
            for (&x, o) in xs {
                let c = match o {
                        Obstacle::Wall => '#',
                        Obstacle::Box(BoxType::Single) => 'O',
                        Obstacle::Box(BoxType::Left) => '[',
                        Obstacle::Box(BoxType::Right) => ']',
                };

                v[y as usize][x as usize] = c;
            }
        }

        let (r_x, r_y): (usize, usize) = self.robot.usized().unwrap();
        v[r_y][r_x] = '@';

        let mut s = String::new();

        for row in v.iter() {
            s.push_str(&row.iter().collect::<String>());
            s.push('\n');
        }

        write!(f, "{}", s.trim_end())
    }
}

fn part1(input: &str) -> usize {
    let (map_str, moves_str) = input.split_once("\n\n").unwrap();
    let mut warehouse = Warehouse::try_from(map_str).unwrap();
    let moves: VecDeque<Point<i64>> = moves_str
        .chars()
        .filter(|&c| c != '\n')
        .map(Point::try_from)
        .collect::<Result<_, _>>()
        .unwrap();

    for &m in moves.iter() {
        warehouse.attempt_move(m);
    }

    warehouse.gps_sum().try_into().unwrap()
}

fn part2(input: &str) -> usize {
    let (map_str, moves_str) = input.split_once("\n\n").unwrap();
    let widened: String = map_str
        .chars()
        .map(|c| match c {
            '#' => "##",
            'O' => "[]",
            '.' => "..",
            '@' => "@,",
            '\n' => "\n",
            _ => panic!(),
        })
        .collect();

    let mut warehouse = Warehouse::try_from(widened.as_str()).unwrap();
    let moves: VecDeque<Point<i64>> = moves_str
        .chars()
        .filter(|&c| c != '\n')
        .map(Point::try_from)
        .collect::<Result<_, _>>()
        .unwrap();

    for &m in moves.iter() {
        warehouse.attempt_move(m);
    }

    warehouse.gps_sum().try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT_SMALL: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";

    const TEST_INPUT_LARGE: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT_SMALL), 2028);
        assert_eq!(part1(TEST_INPUT_LARGE), 10092);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_LARGE), 9021);
    }
}
