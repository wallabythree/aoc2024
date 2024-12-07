use crate::Solution;
use std::ops::{ Add, Mul };

pub const SOLUTION: Solution<usize, usize> = Solution { part1, part2 };

fn concat(a: usize, b: usize) -> usize {
    a * 10usize.pow(b.ilog10() + 1) + b
}

fn has_solution(
    ops: &[fn (usize, usize) -> usize],
    operands: &[usize],
    acc: usize,
    result: usize
) -> bool {
    if operands.is_empty() || acc > result {
        return acc == result;
    }

    ops
        .iter()
        .any(|op| {
            let acc = op(acc, operands[0]);
            has_solution(ops, &operands[1..], acc, result)
        })
}

fn parse_eqs(input: &str) -> Vec<(Vec<usize>, usize)> {
    input
            .lines()
            .map(|l| {
                let mut ns = l
                    .split(|c: char| !c.is_ascii_digit())
                    .filter(|s| !s.is_empty())
                    .map(|n| n.parse().unwrap());

                let result = ns.next().unwrap();
                let operands = ns.collect();
                (operands, result)
            })
            .collect()
}

fn part1(input: &str) -> usize {
    let eqs = parse_eqs(input);
    let ops = [usize::add, usize::mul];

    eqs
        .iter()
        .filter(|(operands, result)|
            has_solution(&ops, &operands[1..], operands[0], *result)
        )
        .map(|(_, result)| result)
        .sum()
}

fn part2(input: &str) -> usize {
    let eqs = parse_eqs(input);
    let ops = [usize::add, usize::mul, concat];

    eqs
        .iter()
        .filter(|(operands, result)|
            has_solution(&ops, &operands[1..], operands[0], *result)
        )
        .map(|(_, result)| result)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 3749);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 11387);
    }
}
