use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use num::Integer;

#[aoc_generator(day02)]
pub fn generate(s: &str) -> Vec<(isize, isize)> {
    s.lines()
        .flat_map(|it| {
            let spl = it.split(',');

            let mut v = vec![];

            for sp in spl {
                if sp.is_empty() {
                    continue;
                }

                let x = sp.split_once('-');
                let (lhs, rhs) = x.unwrap();
                v.push((lhs.parse::<isize>().unwrap(), rhs.parse::<isize>().unwrap()));
            }

            v
        })
        .collect()
}

fn is_invalid_id(num: &str, take: usize) -> bool {
    if !num.len().is_multiple_of(take) {
        return false;
    }

    for i in 0..take {
        if !num.chars().skip(i).step_by(take).all_equal() {
            return false;
        }
    }

    true
}

#[aoc(day02, part1)]
pub fn part1(inp: &[(isize, isize)]) -> isize {
    let mut invalid = 0;

    for (l, r) in inp {
        for id in *l..=*r {
            let id_str = id.to_string();
            let len = id_str.len();
            if len.is_odd() {
                continue;
            }

            if is_invalid_id(&id_str, len / 2) {
                invalid += id;
            }
        }
    }

    invalid
}

#[aoc(day02, part2)]
pub fn part2(inp: &[(isize, isize)]) -> isize {
    let mut invalid = 0;

    for (l, r) in inp {
        for id in *l..=*r {
            let id_str = id.to_string();
            let len = id_str.len();

            for l in 1..=(len / 2) {
                if is_invalid_id(&id_str, l) {
                    invalid += id;
                    break;
                }
            }
        }
    }

    invalid
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_p1() {
        let data = generate(TEST_INPUT);
        let res = part1(&data);
        assert_eq!(res, 1_227_775_554);
    }

    #[test]
    fn test_p2() {
        let data = generate(TEST_INPUT);
        let res = part2(&data);
        assert_eq!(res, 4_174_379_265);
    }
}
