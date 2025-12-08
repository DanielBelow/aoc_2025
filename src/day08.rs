use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use parse_display_derive::{Display, FromStr};
use std::collections::{HashMap, HashSet};

#[derive(Display, FromStr, Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[display("{x},{y},{z}")]
pub struct JunctionBox {
    x: isize,
    y: isize,
    z: isize,
}

impl JunctionBox {
    pub const fn distance_to(&self, target: &Self) -> isize {
        (self.x - target.x).pow(2) + (self.y - target.y).pow(2) + (self.z - target.z).pow(2)
    }
}

#[aoc_generator(day08)]
pub fn generate(s: &str) -> Vec<JunctionBox> {
    s.lines()
        .filter_map(|l| l.parse::<JunctionBox>().ok())
        .collect_vec()
}

#[aoc(day08, part1)]
pub fn part1(inp: &[JunctionBox]) -> usize {
    let junction_pairs = generate_pairs(inp);
    solve_p1(inp, &junction_pairs[..1000])
}

#[aoc(day08, part2)]
pub fn part2(inp: &[JunctionBox]) -> Option<isize> {
    let junction_pairs = generate_pairs(inp);

    let mut circuits = HashMap::new();
    let mut circuit_id = 1;
    for (first, other) in &junction_pairs {
        if circuits.contains_key(first) && circuits.contains_key(other) {
            // both circuits already connected to something
            // update all of "other" to same circuit as "first"
            let first_id = *circuits.get(first)?;
            let other_id = *circuits.get(other)?;

            for circ in circuits.values_mut() {
                if *circ == other_id {
                    *circ = first_id;
                }
            }
        } else {
            // at least one of the boxes is not connected yet
            assert!(!circuits.contains_key(first) || !circuits.contains_key(other));

            let num = *circuits
                .get(first)
                .or_else(|| circuits.get(other))
                .unwrap_or_else(|| {
                    circuit_id += 1;
                    &circuit_id
                });

            *circuits.entry(*first).or_default() = num;
            *circuits.entry(*other).or_default() = num;
        }

        if circuits.values().all_equal() && circuits.len() == inp.len() {
            // all nodes inserted and all part of the same circuit
            return Some(first.x * other.x);
        }
    }

    unreachable!("no solution found")
}

fn generate_pairs(inp: &[JunctionBox]) -> Vec<(JunctionBox, JunctionBox)> {
    inp.iter()
        .combinations(2)
        .sorted_by_key(|a| a[0].distance_to(a[1]))
        .map(|a| (*a[0], *a[1]))
        .collect_vec()
}

fn solve_p1(inp: &[JunctionBox], junction_pairs: &[(JunctionBox, JunctionBox)]) -> usize {
    let connected_components = pathfinding::prelude::connected_components(inp, |junction_box| {
        junction_pairs
            .iter()
            .filter_map(|(l, r)| {
                if *l == *junction_box {
                    Some(*r)
                } else if *r == *junction_box {
                    Some(*l)
                } else {
                    None
                }
            })
            .collect_vec()
    });

    connected_components
        .iter()
        .map(HashSet::len)
        .k_largest(3)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "162,817,812\n\
                              57,618,57\n\
                              906,360,560\n\
                              592,479,940\n\
                              352,342,300\n\
                              466,668,158\n\
                              542,29,236\n\
                              431,825,988\n\
                              739,650,466\n\
                              52,470,668\n\
                              216,146,977\n\
                              819,987,18\n\
                              117,168,530\n\
                              805,96,715\n\
                              346,949,466\n\
                              970,615,88\n\
                              941,993,340\n\
                              862,61,35\n\
                              984,92,344\n\
                              425,690,689";

    #[test]
    fn test_p1() {
        let data = generate(TEST_INPUT);
        let junction_pairs = generate_pairs(&data);
        let res = solve_p1(&data, &junction_pairs[..10]);
        assert_eq!(res, 40);
    }

    #[test]
    fn test_p2() {
        let data = generate(TEST_INPUT);
        let res = part2(&data);
        assert_eq!(res, Some(25_272));
    }
}
