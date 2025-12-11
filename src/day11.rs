use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashMap;

#[aoc_generator(day11)]
pub fn generate(s: &str) -> Option<HashMap<String, Vec<String>>> {
    let mut result = HashMap::new();

    for line in s.lines() {
        let (lhs, rhs) = line.split_once(':')?;
        let rhs = rhs
            .split_ascii_whitespace()
            .map(str::to_string)
            .collect_vec();

        result.entry(lhs.to_string()).insert_entry(rhs);
    }

    Some(result)
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct SearchState {
    node: String,
    seen_fft: bool,
    seen_dac: bool,
}

impl SearchState {
    fn get_successors(&self, mapping: &HashMap<String, Vec<String>>) -> Vec<Self> {
        mapping.get(&self.node).map_or_else(Vec::new, |res| {
            res.iter()
                .map(|s| Self {
                    node: s.clone(),
                    seen_fft: self.seen_fft || s == "fft",
                    seen_dac: self.seen_dac || s == "dac",
                })
                .collect_vec()
        })
    }

    fn success(&self) -> bool {
        self.node == "out" && self.seen_fft && self.seen_dac
    }
}

#[aoc(day11, part1)]
pub fn part1(inp: &HashMap<String, Vec<String>>) -> usize {
    let state = SearchState {
        node: "you".to_string(),
        // we don't care about these two flags in p1
        seen_fft: true,
        seen_dac: true,
    };

    count_paths(state, inp)
}

#[aoc(day11, part2)]
pub fn part2(inp: &HashMap<String, Vec<String>>) -> usize {
    let state = SearchState {
        node: "svr".to_string(),
        seen_fft: false,
        seen_dac: false,
    };

    count_paths(state, inp)
}

fn count_paths(initial_state: SearchState, mapping: &HashMap<String, Vec<String>>) -> usize {
    pathfinding::prelude::count_paths(
        initial_state,
        |state| state.get_successors(mapping),
        SearchState::success,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "aaa: you hhh\n\
                              you: bbb ccc\n\
                              bbb: ddd eee\n\
                              ccc: ddd eee fff\n\
                              ddd: ggg\n\
                              eee: out\n\
                              fff: out\n\
                              ggg: out\n\
                              hhh: ccc fff iii\n\
                              iii: out";

    const TEST_INPUT_P2: &str = "svr: aaa bbb\n\
                                 aaa: fft\n\
                                 fft: ccc\n\
                                 bbb: tty\n\
                                 tty: ccc\n\
                                 ccc: ddd eee\n\
                                 ddd: hub\n\
                                 hub: fff\n\
                                 eee: dac\n\
                                 dac: fff\n\
                                 fff: ggg hhh\n\
                                 ggg: out\n\
                                 hhh: out";

    #[test]
    fn test_p1() {
        let data = generate(TEST_INPUT).expect("valid input");
        let res = part1(&data);
        assert_eq!(res, 5);
    }

    #[test]
    fn test_p2() {
        let data = generate(TEST_INPUT_P2).expect("valid input");
        let res = part2(&data);
        assert_eq!(res, 2);
    }
}
