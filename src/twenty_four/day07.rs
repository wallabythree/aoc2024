use crate::Solution;
use std::ops::{ Add, Mul };

pub const SOLUTION: Solution<u64, u64> = Solution { part1, part2 };

fn concat(a: u64, b: u64) -> u64 {
    a * 10u64.pow(b.ilog10() + 1) + b
}

fn has_solution(
    ops: &[fn (u64, u64) -> u64],
    operands: &[u64],
    acc: u64,
    result: u64
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

fn parse_eqs(input: &str) -> Vec<(Vec<u64>, u64)> {
    input
            .lines()
            .map(|l| {
                let (n, ns) = l.split_once(':').unwrap();
                let result = n.parse().unwrap();
                let operands = ns
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect();

                (operands, result)
            })
            .collect()
}

fn part1(input: &str) -> u64 {
    let eqs = parse_eqs(input);
    let ops = [u64::add, u64::mul];

    eqs
        .iter()
        .filter(|(operands, result)|
            has_solution(&ops, &operands[1..], operands[0], *result)
        )
        .map(|(_, result)| result)
        .sum()
}

fn part2(input: &str) -> u64 {
    let eqs = parse_eqs(input);
    let ops = [u64::add, u64::mul, concat];

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
