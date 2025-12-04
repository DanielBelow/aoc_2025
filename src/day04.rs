use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use pathfinding::matrix::Matrix;

const ROLL_OF_PAPER: char = '@';

#[derive(Copy, Clone, Debug)]
pub struct Cell {
    kind: char,
    num_neighbours: usize,
}

#[aoc_generator(day04)]
pub fn generate(s: &str) -> Option<Matrix<Cell>> {
    let matrix =
        Matrix::from_rows(s.lines().map(|l| l.chars().collect_vec()).collect_vec()).ok()?;

    let mut cells = Matrix::new(
        matrix.rows,
        matrix.columns,
        Cell {
            kind: '.',
            num_neighbours: 0,
        },
    );

    for (pos, c) in matrix.items() {
        let num_neighbours = matrix
            .neighbours(pos, true)
            .filter(|p| matrix[p] == ROLL_OF_PAPER)
            .count();
        let cell_to_update = &mut cells[pos];
        cell_to_update.kind = *c;
        cell_to_update.num_neighbours = num_neighbours;
    }

    Some(cells)
}

#[aoc(day04, part1)]
pub fn part1(inp: &Matrix<Cell>) -> usize {
    inp.items().filter(|&(_, c)| is_accessible(c)).count()
}

#[aoc(day04, part2)]
pub fn part2(inp: &Matrix<Cell>) -> usize {
    let mut removed = 0;

    let mut inp = inp.clone();

    loop {
        let mut had_changes = false;

        for pos in inp.keys() {
            let c = &mut inp[pos];
            if !is_accessible(c) {
                continue;
            }

            removed += 1;
            c.kind = '.';

            for n in inp.neighbours(pos, true) {
                inp[n].num_neighbours -= 1;
            }

            had_changes = true;
        }

        if !had_changes {
            break;
        }
    }

    removed
}

const fn is_accessible(cell: &Cell) -> bool {
    cell.kind == ROLL_OF_PAPER && cell.num_neighbours < 4
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
