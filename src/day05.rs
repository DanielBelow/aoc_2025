use aoc_runner_derive::{aoc, aoc_generator};

pub struct Input {
    ranges: Vec<(usize, usize)>,
    ids: Vec<usize>,
}

#[aoc_generator(day05)]
pub fn generate(s: &str) -> Option<Input> {
    let (ranges, ids) = s.split_once("\n\n")?;

    let ranges = ranges.lines().fold(vec![], |mut acc, line| {
        let (from, to) = line.split_once("-").unwrap();
        acc.push((from.parse::<usize>().unwrap(), to.parse::<usize>().unwrap()));
        acc
    });

    let ids = ids.lines().fold(vec![], |mut acc, line| {
        acc.push(line.parse::<usize>().unwrap());
        acc
    });

    Some(Input { ranges, ids })
}

#[aoc(day05, part1)]
pub fn part1(inp: &Input) -> usize {
    let ranges = merge_overlapping(&mut inp.ranges.clone());

    inp.ids.iter().fold(0, |acc, &id| {
        let is_fresh = ranges.iter().any(|r| id >= r[0] && id <= r[1]);
        acc + usize::from(is_fresh)
    })
}

#[aoc(day05, part2)]
pub fn part2(inp: &Input) -> usize {
    let ranges = merge_overlapping(&mut inp.ranges.clone());
    ranges.iter().map(|r| r[1] - r[0] + 1).sum()
}

fn merge_overlapping(ranges: &mut [(usize, usize)]) -> Vec<Vec<usize>> {
    ranges.sort_by(|(lhs, _), (rhs, _)| lhs.cmp(rhs));

    let mut result = vec![];
    result.push(vec![ranges[0].0, ranges[0].1]);

    for cur in ranges.iter().skip(1) {
        let j = result.len() - 1;

        if (result[j][0]..=result[j][1]).contains(&cur.0) {
            result[j][1] = cur.1.max(result[j][1]);
        } else {
            result.push(vec![cur.0, cur.1]);
        }
    }

    result
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
        let data = generate(TEST_INPUT).unwrap();
        let res = part1(&data);
        assert_eq!(res, 3);
    }

    #[test]
    fn test_p2() {
        let data = generate(TEST_INPUT).unwrap();
        let res = part2(&data);
        assert_eq!(res, 14);
    }
}
