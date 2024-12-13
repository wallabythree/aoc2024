use crate::Solution;
use std::collections::HashMap;

pub const SOLUTION: Solution<u64, u64> = Solution { part1, part2 };

fn decimals(n: u64) -> u32 {
    n.checked_ilog10().map_or(1, |e| e + 1)
}

fn split(n: u64) -> (u64, u64) {
    let len = 10u64.pow(decimals(n) / 2);

    (n / len, n % len)
}

fn count_splits(
    stone: u64,
    blinks: u64,
    cache: &mut HashMap<(u64,u64), u64>,
) -> u64 {
    if blinks == 0 {
        return 0;
    }

    if let Some(&count) = cache.get(&(stone, blinks)) {
        return count;
    }

    let count = if decimals(stone) % 2 == 1 {
        count_splits((stone * 2024).max(1), blinks - 1, cache)
    } else {
        let (left, right) = split(stone);

        1 + count_splits(left, blinks - 1, cache)
          + count_splits(right, blinks - 1, cache)
    };

    cache.insert((stone, blinks), count);
    count
}

fn count_stones(stones: &[u64], blinks: u64) -> u64 {
    let mut cache = HashMap::new();

    let splits: u64 = stones
        .iter()
        .map(|&stone| count_splits(stone, blinks, &mut cache))
        .sum();

    let stone_count: u64 = stones.len().try_into().unwrap();

    stone_count + splits
}

fn part1(input: &str) -> u64 {
    let stones: Vec<_> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    count_stones(&stones, 25)
}

fn part2(input: &str) -> u64 {
    let stones: Vec<_> = input
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
