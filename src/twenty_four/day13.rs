use crate::Solution;

pub const SOLUTION: Solution<i64, i64> = Solution { part1, part2 };

fn transpose<T: Copy>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let rows = matrix.len();
    let cols = if matrix.is_empty() { 0 } else { matrix[0].len() };

    (0..cols)
        .map(|c| (0..rows).map(|r| matrix[r][c]).collect())
        .collect()
}

fn solve(eqs: &[Vec<i64>]) -> Option<Vec<i64>> {
    let a = &eqs[0];
    let b = &eqs[1];

    let r: Vec<_> = a
        .iter()
        .zip(b)
        .map(|(e_a, e_b)| e_a * b[0] - e_b * a[0])
        .collect();

    if r[2] % r[1] != 0 {
        return None;
    }
    let y = r[2] / r[1];

    if (a[2] - a[1] * y) % a[0] != 0 {
        return None;
    }
    let x = (a[2] - a[1] * y) / a[0];

    Some(vec![x, y])
}

fn part1(input: &str) -> i64 {
    let machines: Vec<Vec<_>> = input
        .trim()
        .split("\n\n")
        .map(|machine| machine
            .lines()
            .map(|l| l
                .split(',')
                .map(|e| e.split_once(['+', '=']).unwrap().1.parse().unwrap())
                .collect()
            )
            .collect()
        )
        .map(transpose)
        .collect();

    machines
        .iter()
        .filter_map(|m| solve(m))
        .map(|coeffs| coeffs[0] * 3 + coeffs[1])
        .sum()
}

fn part2(input: &str) -> i64 {
    let machines: Vec<Vec<_>> = input
        .trim()
        .split("\n\n")
        .map(|machine| machine
            .lines()
            .map(|l| l
                .split(',')
                .map(|e| e.split_once(['+', '=']).unwrap().1.parse().unwrap())
                .collect()
            )
            .collect()
        )
        .map(|m: Vec<Vec<_>>| {
            let mut eqs = m.clone();
            let last_row = eqs.len() - 1;
            let cols = eqs[last_row].len();

            for i in 0..cols {
                eqs[last_row][i] += 10000000000000;
            }

            eqs
        })
        .map(transpose)
        .collect();

    machines
        .iter()
        .filter_map(|m| solve(m))
        .map(|coeffs| coeffs[0] * 3 + coeffs[1])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::part1;

    const TEST_INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 480);
    }
}
