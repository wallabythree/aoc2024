use crate::Solution;

use std::ops::Add;
use std::collections::HashSet;

pub const SOLUTION: Solution<usize, usize> = Solution { part1, part2 };

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Point(isize, isize);

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Debug)]
struct Robot {
    p: Point,
    v: Point,
}

impl Robot {
    fn advance(&mut self, width: isize, height: isize) {
        let mut next = self.p + self.v;

        next.0 = next.0.rem_euclid(width);
        next.1 = next.1.rem_euclid(height);

        self.p = next;
    }
}

#[derive(Debug)]
struct RobotMap {
    robots: Vec::<Robot>,
    width: isize,
    height: isize,
}

impl RobotMap {
    fn tick(&mut self) {
        for robot in self.robots.iter_mut() {
            robot.advance(self.width, self.height);
        }
    }

    fn safety_factor(&self) -> usize {
        let mut quadrants = [[0, 0],[0,0]];

        for r in &self.robots {
            if r.p.0 == self.width / 2 || r.p.1 == self.height / 2 {
                continue;
            }

            let x = (r.p.0 + self.width / 2) / self.width;
            let y = (r.p.1 + self.height / 2) / self.height;

            quadrants[y as usize][x as usize] += 1;
        }

        quadrants.iter().flatten().product()
    }

    fn contiguous(&self, p: Point, visited: &mut HashSet<Point>) -> usize {
        if visited.contains(&p) {
            return 0;
        }
        visited.insert(p);

        let dirs = [Point(0, -1), Point(1, 0), Point(0, 1), Point(-1, 0)];

        1 + self
            .robots
            .iter()
            .filter(|r| dirs.iter().any(|&d| r.p == p + d))
            .map(|r| self.contiguous(r.p, visited))
            .sum::<usize>()
    }

    fn as_str(&self) -> String {
        let mut map: Vec<Vec<_>> = (0..self.height)
            .map(|_| (0..self.width).map(|_| 0).collect())
            .collect();

        for robot in &self.robots {
            println!("{:?}", robot);
            let (x, y) = (robot.p.0 as usize, robot.p.1 as usize);
            map[y][x] += 1;
        }

        let mut s = String::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let n = map[y as usize][x as usize];

                if n == 0 {
                    s.push('.');
                } else {
                    s.push_str(&n.to_string());
                }
            }
            s.push('\n');
        }

        s
    }

    fn from(input: &str, width: isize, height: isize) -> Self {
        let robots = input
            .lines()
            .map(|l| l
                .split_whitespace()
                .map(|s| {
                    let (x, y) = s
                        .split_once('=')
                        .unwrap()
                        .1
                        .split_once(',')
                        .unwrap();

                    Point(x.parse().unwrap(), y.parse().unwrap())
                })
                .collect::<Vec<_>>()
            )
            .map(|r| Robot { p: r[0], v: r[1]  })
            .collect();

        Self { robots, width, height }
    }
}

fn part1(input: &str) -> usize {
    let mut map = RobotMap::from(input, 101, 103);

    for _ in 0..100 {
        map.tick();
    }

    map.safety_factor()
}

fn part2(input: &str) -> usize {
    let mut map = RobotMap::from(input, 101, 103);
    let mut ticks = 0;

    loop {
        let mut visited: HashSet<Point> = HashSet::new();
        let mut contiguous = 0;

        for robot in map.robots.iter() {
            let p = robot.p;

            if visited.contains(&p) {
                continue;
            }

            contiguous = contiguous.max(map.contiguous(p, &mut visited));
        }

        if contiguous >= 20 {
            break;
        }

        map.tick();
        ticks += 1;
    }

    println!("{}", map.as_str());

    ticks
}

#[cfg(test)]
mod tests {
    use super::RobotMap;

    const TEST_INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

    #[test]
    fn test_part1() {
        let mut map = RobotMap::from(TEST_INPUT, 11, 7);

        for _ in 0..100 {
            map.tick();
        }

        assert_eq!(map.safety_factor(), 12);
    }
}
