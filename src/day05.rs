use aoc_runner_derive::{aoc, aoc_generator};
use rayon::prelude::*;

pub struct Input {
    ranges: Vec<(usize, usize)>,
    ids: Vec<usize>,
}

#[aoc_generator(day05)]
pub fn generate(s: &str) -> Option<Input> {
    let (range_lines, id_lines) = s.split_once("\n\n")?;

    let mut ranges = vec![];
    for line in range_lines.lines() {
        let (from, to) = line.split_once('-')?;
        ranges.push((from.parse::<usize>().ok()?, to.parse::<usize>().ok()?));
    }

    let mut ids = vec![];
    for line in id_lines.lines() {
        ids.push(line.parse::<usize>().ok()?);
    }

    Some(Input { ranges, ids })
}

#[aoc(day05, part1)]
pub fn part1(inp: &Input) -> usize {
    inp.ids
        .par_iter()
        .map(|&id| {
            let is_fresh = inp.ranges.iter().any(|&(from, to)| id >= from && id <= to);
            usize::from(is_fresh)
        })
        .sum()
}

#[aoc(day05, part2)]
pub fn part2(inp: &Input) -> usize {
    let mut ranges = inp.ranges.clone();
    ranges.sort_by_key(|(from, _)| *from);

    let mut count = 0;
    let mut largest_fresh = 0;

    for (from, to) in ranges {
        let diff = if from > largest_fresh {
            Some(to - from + 1)
        } else {
            to.checked_sub(largest_fresh)
        };

        count += diff.unwrap_or_default();
        largest_fresh = diff.map_or(largest_fresh, |_| to);
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3-5\n\
                              10-14\n\
                              16-20\n\
                              12-18\n\
                              \n\
                              1\n\
                              5\n\
                              8\n\
                              11\n\
                              17\n\
                              32";

    #[test]
    fn test_p1() {
        let data = generate(TEST_INPUT).expect("valid test input");
        let res = part1(&data);
        assert_eq!(res, 3);
    }

    #[test]
    fn test_p2() {
        let data = generate(TEST_INPUT).expect("valid test input");
        let res = part2(&data);
        assert_eq!(res, 14);
    }
}
