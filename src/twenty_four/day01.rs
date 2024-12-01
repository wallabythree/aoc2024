use crate::Solution;
use std::collections::HashMap;
use std::iter::zip;

pub const SOLUTION: Solution<usize, usize> = Solution { part1, part2 };

fn parse(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut left: Vec<usize> = Vec::new();
    let mut right: Vec<usize> = Vec::new();

    input.lines().for_each(|line| {
        let mut iter = line.split_whitespace().map(|s| s.parse().unwrap());

        let a = iter.next().unwrap();
        let b = iter.next().unwrap();

        left.push(a);
        right.push(b);
    });

    (left, right)
}

fn part1(input: &str) -> usize {
    let (mut left, mut right) = parse(input);

    left.sort();
    right.sort();

    let pairs = zip(left, right);

    pairs.map(|(a, b)| a.abs_diff(b)).sum()
}

fn part2(input: &str) -> usize {
    let (left, right) = parse(input);
    let mut freqs: HashMap<usize, usize> = HashMap::new();

    for e in right {
        freqs.entry(e).and_modify(|v| *v += 1).or_insert(1);
    }

    left.iter().map(|e| e * freqs.get(e).unwrap_or(&0)).sum()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT: &str = "3   4
    4   3
    2   5
    1   3
    3   9
    3   3";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 31);
    }
}
