use crate::Solution;

pub const SOLUTION: Solution<usize, usize> = Solution { part1, part2 };

fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| l
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect()
        )
        .collect()
}

fn is_safe(report: &[u8]) -> bool {
    (report.is_sorted_by(|a, b| a < b) || report.is_sorted_by(|a, b| b < a)) &&
    // check if the distance between consecutive levels lies in the range 1..4
    report.windows(2).all(|w| (1..4).contains(&w[0].abs_diff(w[1])))
}

fn part1(input: &str) -> usize {
    let reports = parse(input);

    reports
        .iter()
        .filter(|r| is_safe(r))
        .count()
}

fn part2(input: &str) -> usize {
    let reports = parse(input);

    reports
        .iter()
        .map(|report| {
            // brute force: consider original report
            // and all n-choose-(n-1) subsequences
            let mut combinations = Vec::with_capacity(report.len() + 1);
            combinations.push(report.clone());

            for i in 0..report.len() {
                let mut combination = report.clone();
                combination.remove(i);
                combinations.push(combination);
            }

            combinations
        })
        .filter(|rs| rs.iter().any(|r| is_safe(r)))
        .count()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 4);
    }
}
