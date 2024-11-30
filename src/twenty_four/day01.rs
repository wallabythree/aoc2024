use crate::Solution;

pub const SOLUTION: Solution<usize, usize> = Solution { part1, part2 };

fn part1(input: &str) -> usize {
    input.len()
}

fn part2(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT: &str = "";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 0);
    }
    
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), TEST_INPUT.len());
    }
}

