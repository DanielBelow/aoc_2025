use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use pathfinding::prelude::Matrix;

#[aoc_generator(day06)]
pub fn generate(s: &str) -> Option<Matrix<char>> {
    Matrix::from_rows(
        s.lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.chars().collect_vec())
            .collect_vec(),
    )
    .ok()
}

#[aoc(day06, part1)]
pub fn part1(inp: &Matrix<char>) -> usize {
    cephalopod_math(inp, inp.rows - 1, |col, operands, _, operation| {
        collect_operands(col, operation, |i, c| operands[i].push(c));
    })
}

#[aoc(day06, part2)]
pub fn part2(inp: &Matrix<char>) -> usize {
    cephalopod_math(inp, inp.columns, |col, operands, col_idx, operation| {
        collect_operands(col, operation, |_, c| operands[col_idx].push(c));
    })
}

fn cephalopod_math<F>(inp: &Matrix<char>, num_ops: usize, collect_operands: F) -> usize
where
    F: Fn(&[&char], &mut [String], usize, &mut Option<char>),
{
    let mut res = 0;
    let mut operands = vec![String::new(); num_ops];
    let mut operation = None;

    for (col_idx, col) in inp.column_iter().enumerate() {
        collect_operands(&col, &mut operands, col_idx, &mut operation);

        if col.iter().all(|it| it.is_ascii_whitespace()) || col_idx == inp.columns - 1 {
            res += finish_operands(operation, &operands);

            operation = None;
            operands = vec![String::new(); num_ops];
        }
    }

    res
}

fn collect_operands<F>(column: &[&char], operation: &mut Option<char>, mut append_to_operator: F)
where
    F: FnMut(usize, char),
{
    for (i, &&c) in column.iter().enumerate() {
        match c {
            '*' | '+' => *operation = Some(c),
            c if c.is_ascii_digit() => append_to_operator(i, c),
            _ => {}
        }
    }
}

fn finish_operands(operation: Option<char>, operands: &[String]) -> usize {
    let nums = operands.iter().filter_map(|it| it.parse::<usize>().ok());
    match operation {
        Some('*') => nums.product(),
        Some('+') => nums.sum(),
        _ => panic!("unknown operation"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\n
123 328  51 64 \n
 45 64  387 23 \n
  6 98  215 314\n
*   +   *   +  ";

    #[test]
    fn test_p1() {
        let data = generate(TEST_INPUT).expect("valid input");
        let res = part1(&data);
        assert_eq!(res, 4_277_556);
    }

    #[test]
    fn test_p2() {
        let data = generate(TEST_INPUT).expect("valid input");
        let res = part2(&data);
        assert_eq!(res, 3_263_827);
    }
}
