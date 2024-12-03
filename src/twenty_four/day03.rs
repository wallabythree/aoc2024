use crate::Solution;

pub const SOLUTION: Solution<usize, usize> = Solution { part1, part2 };

// Verbose but fast O(n) solution. Improves upon the initial O(n^2)
// implementation using regex capture groups by switching to fast `str.find`
// pattern matching.

// Parse and extract a usize integer at the start of a given ASCII bytestring.
// Returns the extracted integer and the offset of the integer's last byte in
// the bytestring.
fn extract_usize(s: &[u8]) -> (usize, usize) {
    let mut n = 0;
    let mut i = 0;

    while s[i].is_ascii_digit() {
        n *= 10;
        n += (s[i] - b'0') as usize;
        i += 1;
    }

    (n, i)
}

// Extract a pair of operands in the form "([0-9]+,[0-9]+)" that starts at a
// given substring, if any, and the offset of the closing parenthesis or first
// invalid character.
fn extract_operands(substr: &str) -> (Option<(usize, usize)>, usize) {
    let substr = substr.as_bytes();
    let mut i = 0;

    if substr[i] != b'(' {
        return (None, i);
    }
    i += 1;

    let (a, j) = extract_usize(&substr[i..]);
    i += j;

    if substr[i] != b',' {
        return (None, i);
    }
    i += 1;

    let (b, j) = extract_usize(&substr[i..]);
    i += j;

    if substr[i] != b')' {
        return (None, i);
    }

    (Some((a, b)), i)
}

// Return all operand pairs for a given opcode in a corrupted memory dump.
fn extract_all_operands(block: &str, op: &str) -> Vec<(usize, usize)> {
    let mut subblock = block;
    let mut ops = Vec::new();

    while let Some(i) = subblock.find(op) {
        let off = i + op.len();
        subblock = &subblock[off..];

        let (operands, len) = extract_operands(subblock);

        if let Some(pair) = operands {
            ops.push(pair);
        }

        subblock = &subblock[len..];
    }

    ops
}

fn part1(input: &str) -> usize {
    extract_all_operands(input, "mul")
        .iter()
        .map(|(a, b)| a * b)
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .split("do()")
        .map(|g| g.split("don't()"))
        .flat_map(|i| i.take(1))
        .map(|do_block| extract_all_operands(do_block, "mul")
            .iter()
            .map(|(a, b)| a * b)
            .sum::<usize>())
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
