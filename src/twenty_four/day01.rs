use crate::Solution;
use std::collections::HashMap;
use std::iter::zip;

pub const SOLUTION: Solution<usize, usize> = Solution { part1, part2 };

fn part1(input: &str) -> usize {
    let mut l: Vec<usize> = Vec::new();
    let mut r: Vec<usize> = Vec::new();

    input.lines().for_each(|line| {
        let mut iter = line.split_whitespace().map(|s| s.parse().unwrap());

        let a = iter.next().unwrap();
        let b = iter.next().unwrap();

        l.push(a);
        r.push(b);
    });

    l.sort();
    r.sort();

    let pairs = zip(l, r);

    pairs.map(|(a, b)| a.abs_diff(b)).sum()
}

fn part2(input: &str) -> usize {
    let mut l: Vec<usize> = Vec::new();
    let mut r: HashMap<usize, usize> = HashMap::new();

    input.lines().for_each(|line| {
        let mut iter = line.split_whitespace().map(|s| s.parse().unwrap());

        let a = iter.next().unwrap();
        let b = iter.next().unwrap();

        l.push(a);

        if let Some(freq) = r.get_mut(&b) {
            *freq += 1;
        } else {
            r.insert(b, 1);
        }
    });

    l.iter().map(|a| a * r.get(a).unwrap_or(&0)).sum()
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
