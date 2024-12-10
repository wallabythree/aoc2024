use crate::Solution;
use std::collections::{ HashSet, LinkedList };

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

    let mut chunks: LinkedList<_> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>()
        .chunks(2)
        .collect::<Vec<_>>()
        .iter()
        .enumerate()
        .map(|(id, pair)| Chunk { id, size: pair[0], padding: pair[1] })
        .collect();

    let mut in_order = LinkedList::new();
    let mut moved = HashSet::new();

    while let Some(mut chunk) = chunks.pop_back() {
        if chunks.is_empty() {
            chunks.push_back(chunk);
            break;
        }

        if moved.contains(&chunk.id) {
            in_order.push_front(chunk);
            continue;
        }

        let mut cursor = chunks.cursor_front_mut();

        while let Some(candidate) = cursor.current() {
            if chunk.size <= candidate.padding {
                let orig_chunk_padding = chunk.padding;

                chunk.padding = candidate.padding - chunk.size;
                candidate.padding = 0;

                cursor.insert_after(chunk);
                moved.insert(chunk.id);

                if cursor.peek_next().is_some() {
                    let prev = cursor.back_mut().unwrap();
                    prev.padding += orig_chunk_padding + chunk.size;
                }

                break;
            }

            cursor.move_next();
        }

        if !moved.contains(&chunk.id) {
            in_order.push_front(chunk);
        }
    }

    chunks.append(&mut in_order);

    let mut i = 0;
    let mut checksum = 0;

    for chunk in chunks {
        let n = i + chunk.size;
        let m = (n - i) * (n + i - 1) / 2;
        checksum += m * chunk.id;

        i += chunk.size + chunk.padding;
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
