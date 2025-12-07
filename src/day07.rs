use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use pathfinding::matrix::Matrix;
use std::collections::BTreeSet;

const SPLITTER: char = '^';

#[aoc_generator(day07)]
pub fn generate(s: &str) -> Option<Matrix<char>> {
    Matrix::from_rows(s.lines().map(|l| l.chars().collect_vec()).collect_vec()).ok()
}

#[aoc(day07, part1)]
pub fn part1(inp: &Matrix<char>) -> usize {
    let start_pos = find_start(inp);

    let mut num_splits = 0;

    let mut queue = BTreeSet::new();
    queue.insert(start_pos);

    while let Some(p) = queue.pop_first() {
        num_splits += usize::from(inp[p] == SPLITTER);
        for succ in successors(inp, p) {
            queue.insert(succ);
        }
    }

    num_splits
}

fn successors(matrix: &Matrix<char>, (row, col): (usize, usize)) -> Vec<(usize, usize)> {
    let mut succs = vec![];

    if matrix[(row, col)] == SPLITTER {
        if col > 0 {
            succs.push((row, col - 1));
        }

        if col < matrix.columns - 1 {
            succs.push((row, col + 1));
        }
    } else if row < matrix.rows - 1 {
        succs.push((row + 1, col));
    }

    succs
}

#[aoc(day07, part2)]
pub fn part2(inp: &Matrix<char>) -> usize {
    let start_pos = find_start(inp);

    pathfinding::prelude::count_paths(
        start_pos,
        |&p| successors(inp, p),
        |&(r, _)| r == inp.rows - 1,
    )
}

fn find_start(inp: &Matrix<char>) -> (usize, usize) {
    inp.items()
        .find_map(|(p, c)| if *c == 'S' { Some(p) } else { None })
        .expect("'S' position in input")
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = ".......S.......\n\
                              ...............\n\
                              .......^.......\n\
                              ...............\n\
                              ......^.^......\n\
                              ...............\n\
                              .....^.^.^.....\n\
                              ...............\n\
                              ....^.^...^....\n\
                              ...............\n\
                              ...^.^...^.^...\n\
                              ...............\n\
                              ..^...^.....^..\n\
                              ...............\n\
                              .^.^.^.^.^...^.\n\
                              ...............";

    #[test]
    fn test_p1() {
        let data = generate(TEST_INPUT).expect("valid input");
        let res = part1(&data);
        assert_eq!(res, 21);
    }

    #[test]
    fn test_p2() {
        let data = generate(TEST_INPUT).expect("valid input");
        let res = part2(&data);
        assert_eq!(res, 40);
    }
}
