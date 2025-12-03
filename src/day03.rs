use aoc_runner_derive::{aoc, aoc_generator};
use rayon::prelude::*;

#[aoc_generator(day03)]
pub fn generate(s: &str) -> Vec<String> {
    s.lines().map(ToString::to_string).collect()
}

#[aoc(day03, part1)]
pub fn part1(inp: &[String]) -> usize {
    inp.par_iter()
        .map(|l| {
            let mut digits = str_to_digits(l);
            let (d1, p1) = find_max(&mut digits[0..l.len() - 1]);
            let (d2, _) = find_max(&mut digits[p1..]);

            d1 * 10 + d2
        })
        .sum()
}

#[aoc(day03, part2)]
pub fn part2(inp: &[String]) -> usize {
    inp.par_iter()
        .map(|l| {
            let mut digits = str_to_digits(l);

            let remaining_digits = digits.len() - 11;

            let mut result = 0;
            let mut offset = 0;

            for i in 0..12 {
                let (digit, pos) = find_max(&mut digits[offset..remaining_digits + i]);
                offset = pos;
                result = result * 10 + digit;
            }

            result
        })
        .sum()
}

fn str_to_digits(s: &str) -> Vec<(usize, u32)> {
    s.chars()
        .enumerate()
        .map(|(i, c)| (i, c.to_digit(10).expect("valid digit")))
        .collect()
}

fn find_max(digits: &mut [(usize, u32)]) -> (usize, usize) {
    let (_, (mp, md), _) =
        digits.select_nth_unstable_by(0, |(lp, ld), (rp, rd)| rd.cmp(ld).then(lp.cmp(rp)));

    (*md as usize, *mp + 1)
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
