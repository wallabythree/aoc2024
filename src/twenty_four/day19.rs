use crate::Solution;

use std::collections::HashMap;

pub const SOLUTION: Solution<usize, usize> = Solution { part1, part2 };

#[derive(Default, Debug)]
struct Trie {
    is_pattern: bool,
    children: HashMap<char, Trie>,
}

impl Trie {
    fn insert(&mut self, s: &[char]) {
        if s.is_empty() {
            return;
        }

        if s.len() == 1 {
            self.children.entry(s[0]).or_default().is_pattern = true;
        }

        self.children.entry(s[0]).or_default().insert(&s[1..]);
    }

    fn count_possible(
        &self,
        needle: &[char],
        root: &Trie,
        cache: &mut HashMap<usize, usize>
    ) -> usize {
        if needle.is_empty() {
            if self.is_pattern {
                return 1;
            } else {
                return 0;
            }
        }

        let child_opt = self.children.get(&needle[0]);

        if let Some(child) = child_opt {
            let same_branch = child.count_possible(&needle[1..], root, cache);

            if child.is_pattern {
                let cached = cache.get(&needle.len());

                let nested_branches = if let Some(&result) = cached {
                    result
                } else {
                    root.count_possible(&needle[1..], root, cache)
                };
                cache.insert(needle.len(), nested_branches);

                same_branch + nested_branches
            } else {
                same_branch
            }
        } else {
            0
        }
    }
}

fn parse_input(input: &str) -> (Trie, Vec<Vec<char>>) {
    let (patterns_str, designs_str) = input.split_once("\n\n").unwrap();

    let patterns: Vec<_> = patterns_str
        .split(", ")
        .map(|p| p.chars().collect::<Vec<_>>())
        .collect();

    let designs: Vec<_> = designs_str
        .lines()
        .map(|d| d.chars().collect::<Vec<_>>())
        .collect();

    let mut trie = Trie::default();

    for pattern in patterns {
        trie.insert(&pattern);
    }

    (trie, designs)
}

fn part1(input: &str) -> usize {
    let (trie, designs) = parse_input(input);

    designs
        .iter()
        .filter(|d| trie.count_possible(d, &trie, &mut HashMap::new()) > 0)
        .count()
}

fn part2(input: &str) -> usize {
    let (trie, designs) = parse_input(input);


    designs
        .iter()
        .map(|d| trie.count_possible(d, &trie, &mut HashMap::new()))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";

    const BEEP: &str = "gr, g, rgr, r, rg, g

rgrgr
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 6);
        assert_eq!(part1(BEEP), 1);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 16);
    }
}
