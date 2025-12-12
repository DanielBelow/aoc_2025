use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{Itertools, iproduct};
use memoize::memoize;
use pathfinding::prelude::Matrix;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;

#[derive(Clone, Debug)]
pub struct Region {
    width: usize,
    height: usize,
    quantity: Vec<usize>, // element refers to index in Input.shapes
}

#[derive(Clone, Debug)]
pub struct Input {
    shapes: Vec<Matrix<char>>,
    regions: Vec<Region>,
}

#[aoc_generator(day12)]
pub fn generate(s: &str) -> Option<Input> {
    let mut shapes = vec![];
    let mut regions = vec![];

    let spl = s.split("\n\n").collect_vec();

    for block in spl.iter().take(spl.len() - 1) {
        let rows = block
            .lines()
            .dropping(1)
            .map(|it| it.chars().collect_vec())
            .collect_vec();
        shapes.push(Matrix::from_rows(rows).ok()?);
    }

    let regions_block = spl.last()?;
    for line in regions_block.lines() {
        let (wnh, quants) = line.split_once(": ")?;
        let (w, h) = wnh.split_once('x')?;
        let quants = quants
            .split_ascii_whitespace()
            .filter_map(|it| it.parse::<usize>().ok())
            .collect_vec();

        regions.push(Region {
            width: w.parse::<usize>().ok()?,
            height: h.parse::<usize>().ok()?,
            quantity: quants,
        });
    }

    Some(Input { shapes, regions })
}

fn can_fit_shapes(region: &Region, shapes: &[Matrix<char>]) -> bool {
    let mut shapes_to_place = vec![];
    for (idx, &num) in region.quantity.iter().enumerate() {
        for _ in 0..num {
            shapes_to_place.push(shapes[idx].clone());
        }
    }

    let num_hashes = shapes_to_place
        .iter()
        .map(|it| it.items().filter(|(_, i)| **i == '#').count())
        .sum::<usize>();
    if num_hashes > region.height * region.width {
        return false;
    }

    let place_region = Matrix::new(region.height, region.width, '.');

    can_place_all_shapes(place_region, shapes_to_place)
}

fn orientations(mat: Matrix<char>) -> Vec<Matrix<char>> {
    let mut orientations = Vec::with_capacity(8);

    let flipped = mat.flipped_lr();
    for rot in 1..4 {
        orientations.push(mat.rotated_ccw(rot));
        orientations.push(flipped.rotated_ccw(rot));
    }

    orientations.push(flipped);
    orientations.push(mat);

    orientations
}

#[memoize]
#[allow(clippy::needless_pass_by_value)]
fn can_place_all_shapes(place_region: Matrix<char>, mut shapes: Vec<Matrix<char>>) -> bool {
    if shapes.is_empty() {
        return true;
    }

    let mat = shapes.pop().expect("non empty");
    let orientations = orientations(mat);

    for (row, col) in iproduct!(0..place_region.rows, 0..place_region.columns) {
        for attempt in &orientations {
            if can_place_shape(row, col, &place_region, attempt) {
                let mut new_region = place_region.clone();
                // place shape
                new_region.set_slice((row, col), attempt);

                // check remaining
                if can_place_all_shapes(new_region, shapes.clone()) {
                    return true;
                }
            }
        }
    }

    false
}

fn can_place_shape(
    row: usize,
    col: usize,
    place_region: &Matrix<char>,
    shape: &Matrix<char>,
) -> bool {
    for (shape_r, shape_c) in iproduct!(0..shape.rows, 0..shape.columns) {
        match place_region.get((row + shape_r, col + shape_c)) {
            None => return false,
            Some(c) => {
                if *c == '#' && shape[(shape_r, shape_c)] == '#' {
                    return false;
                }
            }
        }
    }

    true
}

#[aoc(day12, part1)]
pub fn part1(inp: &Input) -> usize {
    inp.regions
        .par_iter()
        .filter(|region| can_fit_shapes(region, &inp.shapes))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "0:\n\
                              ###\n\
                              ##.\n\
                              ##.\n\
                              \n\
                              1:\n\
                              ###\n\
                              ##.\n\
                              .##\n\
                              \n\
                              2:\n\
                              .##\n\
                              ###\n\
                              ##.\n\
                              \n\
                              3:\n\
                              ##.\n\
                              ###\n\
                              ##.\n\
                              \n\
                              4:\n\
                              ###\n\
                              #..\n\
                              ###\n\
                              \n\
                              5:\n\
                              ###\n\
                              .#.\n\
                              ###\n\
                              \n\
                              4x4: 0 0 0 0 2 0\n\
                              12x5: 1 0 1 0 2 2\n\
                              12x5: 1 0 1 0 3 2";

    #[test]
    fn test_p1() {
        let data = generate(TEST_INPUT).expect("valid input");
        let res = part1(&data);
        assert_eq!(res, 2);
    }
}
