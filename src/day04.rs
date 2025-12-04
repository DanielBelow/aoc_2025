use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use pathfinding::matrix::Matrix;

const ROLL_OF_PAPER: char = '@';

#[aoc_generator(day04)]
pub fn generate(s: &str) -> Option<Matrix<char>> {
    Matrix::from_rows(s.lines().map(|l| l.chars().collect_vec()).collect_vec()).ok()
}

#[aoc(day04, part1)]
pub fn part1(inp: &Matrix<char>) -> usize {
    inp.keys()
        .fold(0, |acc, pos| acc + usize::from(is_accessible(pos, inp)))
}

#[aoc(day04, part2)]
pub fn part2(inp: &Matrix<char>) -> usize {
    let mut removed = 0;

    let mut inp = inp.clone();

    loop {
        let mut had_changes = false;

        for pos in inp.keys() {
            if is_accessible(pos, &inp) {
                removed += 1;
                had_changes = true;
                inp[pos] = '.';
            }
        }

        if !had_changes {
            break;
        }
    }

    removed
}

fn is_accessible(center_pos: (usize, usize), grid: &Matrix<char>) -> bool {
    if grid[center_pos] != ROLL_OF_PAPER {
        return false;
    }

    let num_neighbours = grid
        .neighbours(center_pos, true)
        .filter(|&neighbour_pos| grid[neighbour_pos] == ROLL_OF_PAPER)
        .count();

    num_neighbours < 4
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "..@@.@@@@.\n\
                              @@@.@.@.@@\n\
                              @@@@@.@.@@\n\
                              @.@@@@..@.\n\
                              @@.@@@@.@@\n\
                              .@@@@@@@.@\n\
                              .@.@.@.@@@\n\
                              @.@@@.@@@@\n\
                              .@@@@@@@@.\n\
                              @.@.@@@.@.";

    #[test]
    fn test_p1() {
        let data = generate(TEST_INPUT).expect("valid input data");
        let res = part1(&data);
        assert_eq!(res, 13);
    }

    #[test]
    fn test_p2() {
        let data = generate(TEST_INPUT).expect("valid input data");
        let res = part2(&data);
        assert_eq!(res, 43);
    }
}
