use crate::Solution;
use std::collections::HashMap;

pub const SOLUTION: Solution<usize, usize> = Solution { part1, part2 };

fn decimals(n: usize) -> u32 {
    n.checked_ilog10().map_or(1, |e| e + 1)
}

fn split(n: usize) -> (usize, usize) {
    let len = 10usize.pow(decimals(n) / 2);

    (n / len, n % len)
}

fn count_splits(
    stone: usize,
    blinks: usize,
    cache: &mut HashMap<(usize,usize), usize>,
) -> usize {
    if blinks == 0 {
        return 0;
    }

    if let Some(&count) = cache.get(&(stone, blinks)) {
        return count;
    }

    let count;

    if decimals(stone) % 2 == 1 {
        count = count_splits((stone * 2024).max(1), blinks - 1, cache);
    } else {
        let (left, right) = split(stone);

        count = 1 + count_splits(left, blinks - 1, cache)
                  + count_splits(right, blinks - 1, cache);
    };

    cache.insert((stone, blinks), count);
    return count;
}

fn count_stones(stones: &[usize], blinks: usize) -> usize {
    let mut cache = HashMap::new();

    let splits: usize = stones
        .iter()
        .map(|&stone| count_splits(stone, blinks, &mut cache))
        .sum();

    stones.len() + splits
}

fn part1(input: &str) -> usize {
    let stones: Vec<_> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    count_stones(&stones, 25)
}

fn part2(input: &str) -> usize {
    let stones: Vec<_> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    count_stones(&stones, 75)
}

#[cfg(test)]
mod tests {
    use super::part1;

    const TEST_INPUT: &str = "125 17\n";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 55312);
    }
}
