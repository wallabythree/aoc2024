use crate::Solution;

pub const SOLUTION: Solution<usize, usize> = Solution { part1, part2 };

#[derive(Clone, Copy, Debug)]
struct Chunk {
    id: usize,
    size: usize,
    padding: usize,
}

fn part1(input: &str) -> usize {
    let mut list: Vec<Option<usize>> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .enumerate()
        .flat_map(|(i, size)| {
            let node = if i % 2 == 0 { Some(i / 2) } else { None };
            (0..size).map(|_| node).collect::<Vec<_>>()
        })
        .collect();

    let mut i = 0;
    let mut j = list.len() - 1;

    while i < list.len() {
        if list[i].is_some() {
            i += 1;
            continue;
        }

        while list[j].is_none() && j > 0 {
            j -= 1;
        }

        if i > j {
            break;
        }

        list.swap(i, j);
        i += 1;
    }

    list
        .iter()
        .enumerate()
        .filter_map(|(i, chunk)| chunk.map(|id| i * id))
        .sum()
}

fn part2(input: &str) -> usize {
    let mut input = input.trim().to_string();
    input.push('0');

    let mut allocated: Vec<_> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>()
        .chunks(2)
        .collect::<Vec<_>>()
        .iter()
        .enumerate()
        .map(|(id, pair)| Chunk { id, size: pair[0], padding: pair[1] })
        .collect();

    let mut i = allocated.len() - 1;

    while i > 1 {
        for j in 0..allocated.len() {
            if i <= j {
                break;
            }

            if allocated[i].size <= allocated[j].padding {
                allocated[i - 1].padding += allocated[i].size + allocated[i].padding;
                allocated[i].padding = allocated[j].padding - allocated[i].size;
                allocated[j].padding = 0;

                let removed = allocated.remove(i);
                allocated.insert(j + 1, removed);
                i += 1;
                break;
            }
        }

        i -= 1;
    }

    let mut i = 0;
    let mut checksum = 0;

    for chunk in allocated {
        for j in 0..chunk.size {
            checksum += (i + j) * chunk.id;
        }

        i += chunk.size + chunk.padding
    }

    checksum
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT: &str = "2333133121414131402";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 1928);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 2858);
    }
}
