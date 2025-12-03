use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day03)]
pub fn generate(s: &str) -> Vec<String> {
    s.lines().map(ToString::to_string).collect()
}

#[aoc(day03, part1)]
pub fn part1(inp: &[String]) -> usize {
    let mut res = 0;

    for l in inp {
        let (d1, p1) = find_max(l, 0, l.len() - 1);
        let (d2, _) = find_max(l, p1, l.len() - p1);

        res += d1 * 10 + d2;
    }

    res
}

fn find_max(v: &str, skip: usize, up_to_index: usize) -> (usize, usize) {
    let digits = v
        .chars()
        .enumerate()
        .skip(skip)
        .take(up_to_index)
        .map(|(i, c)| (i, c.to_digit(10).unwrap() as usize))
        .collect_vec();

    let (_, md) = digits.iter().max_by_key(|(_, n)| *n).unwrap();

    let (max_pos, max_digit) = digits.iter().find(|(_, n)| *n == *md).unwrap();
    (*max_digit, *max_pos + 1)
}

#[aoc(day03, part2)]
pub fn part2(inp: &[String]) -> usize {
    let mut res = 0;

    for l in inp {
        let (d1, p1) = find_max(l, 0, l.len() - 11);
        let (d2, p2) = find_max(l, p1, l.len() - 10 - p1);
        let (d3, p3) = find_max(l, p2, l.len() - 9 - p2);
        let (d4, p4) = find_max(l, p3, l.len() - 8 - p3);
        let (d5, p5) = find_max(l, p4, l.len() - 7 - p4);
        let (d6, p6) = find_max(l, p5, l.len() - 6 - p5);
        let (d7, p7) = find_max(l, p6, l.len() - 5 - p6);
        let (d8, p8) = find_max(l, p7, l.len() - 4 - p7);
        let (d9, p9) = find_max(l, p8, l.len() - 3 - p8);
        let (d10, p10) = find_max(l, p9, l.len() - 2 - p9);
        let (d11, p11) = find_max(l, p10, l.len() - 1 - p10);
        let (d12, _) = find_max(l, p11, l.len() - p11);

        let as_str = format!("{d1}{d2}{d3}{d4}{d5}{d6}{d7}{d8}{d9}{d10}{d11}{d12}");
        // println!("{l} -> {as_str}");

        res += as_str.parse::<usize>().unwrap();
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "987654321111111\n\
                              811111111111119\n\
                              234234234234278\n\
                              818181911112111";

    #[test]
    fn test_p1() {
        let data = generate(TEST_INPUT);
        let res = part1(&data);
        assert_eq!(res, 357);
    }

    #[test]
    fn test_p2() {
        let data = generate(TEST_INPUT);
        let res = part2(&data);
        assert_eq!(res, 3_121_910_778_619);
    }
}
