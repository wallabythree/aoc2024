use crate::Solution;
use regex::Regex;

pub const SOLUTION: Solution<usize, usize> = Solution { part1, part2 };

fn part1(input: &str) -> usize {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    re
        .captures_iter(input)
        .map(|s| s
            .iter()
            .skip(1)
            .map(|n| n.unwrap().as_str().parse::<usize>().unwrap())
            .product::<usize>()
        )
        .sum()
}

fn part2(input: &str) -> usize {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    input
        .split("do()")
        .map(|g| g.split("don't()"))
        .flat_map(|i| i.take(1))
        .map(|do_block| {
            re
                .captures_iter(do_block)
                .map(|s| s
                    .iter()
                    .skip(1)
                    .map(|n| n.unwrap().as_str().parse::<usize>().unwrap())
                    .product::<usize>()
                )
                .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT_1: &str ="xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const TEST_INPUT_2: &str ="xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT_1), 161);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_2), 48);
    }
}
