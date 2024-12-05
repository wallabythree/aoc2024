use crate::Solution;
use std::hash::Hash;
use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;

pub const SOLUTION: Solution<usize, usize> = Solution { part1, part2 };

struct SleighSafetyManual<T: Eq + Hash> {
    // maps each page to the set of pages that must not come after it
    rule_map: HashMap<T, HashSet<T>>,
}

impl<T: Eq + Hash + Clone + Copy> SleighSafetyManual<T> {
    fn from_rules(rules: &[(T, T)]) -> Self {
        let mut rule_map = HashMap::new();

        for (a,b) in rules {
            rule_map.entry(*b).or_insert(HashSet::new()).insert(*a);
        }

        Self { rule_map }
    }

    fn validate(&self, update: &[T]) -> bool {
        update.is_sorted_by(|a, b|
            !self.rule_map.get(a).is_some_and(|v| v.contains(b))
        )
    }

    fn repair(&self, update: &[T]) -> Vec<T> {
        let mut sorted: Vec<_> = update.to_vec();

        sorted.sort_by(|a, b| {
            if self.rule_map.get(a).is_some_and(|v| v.contains(b)) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        sorted
    }
}

fn parse_rules(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|l| l
            .split_once('|')
            .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
            .unwrap()
        )
        .collect()
}

fn parse_updates(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|l| l.split(',').map(|n| n.parse().unwrap()).collect())
        .collect()
}

fn parse_input(input: &str) -> (Vec<(usize,usize)>, Vec<Vec<usize>>) {
    let (rules_str, updates_str) = input.split_once("\n\n").unwrap();

    (parse_rules(rules_str), parse_updates(updates_str))
}

fn part1(input: &str) -> usize {
    let (rules, updates) = parse_input(input);
    let manual = SleighSafetyManual::from_rules(&rules);

    updates
        .iter()
        .filter(|update| manual.validate(update))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn part2(input: &str) -> usize {
    let (rules, updates) = parse_input(input);
    let manual = SleighSafetyManual::from_rules(&rules);

    updates
        .iter()
        .filter(|update| !manual.validate(update))
        .map(|update| manual.repair(update))
        .map(|update| update[update.len() / 2])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 143);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 123);
    }
}
