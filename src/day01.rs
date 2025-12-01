use aoc_runner_derive::{aoc, aoc_generator};
use num::Integer;

#[aoc_generator(day01)]
pub fn generate(s: &str) -> Vec<isize> {
    s.lines()
        .filter_map(|line| {
            let mut chr = line.chars();

            let letter = chr.next()?;
            let num = chr.as_str().parse::<isize>().ok()?;
            Some(if letter == 'R' { num } else { -num })
        })
        .collect()
}

const NUM_DIALS: isize = 100;

#[aoc(day01, part1)]
pub fn part1(inp: &[isize]) -> isize {
    let (_, num_zero) = inp.iter().fold((50, 0), |(cur, num_zero), elem| {
        let cur = (cur + elem).rem_euclid(NUM_DIALS);
        (cur, num_zero + isize::from(cur == 0))
    });

    num_zero
}

#[aoc(day01, part2)]
pub fn part2(inp: &[isize]) -> isize {
    let (_, num_zero) = inp.iter().fold((50, 0), |(cur, num_zero), elem| {
        let (full_rotations, remainder) = elem.div_rem(&NUM_DIALS);

        let next_pos = cur + remainder;
        let next_mod = next_pos.rem_euclid(NUM_DIALS);

        let flipped_sign = isize::from(cur != 0 && (next_mod == 0 || next_pos != next_mod));
        (next_mod, num_zero + full_rotations.abs() + flipped_sign)
    });

    num_zero
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "L68\n\
                              L30\n\
                              R48\n\
                              L5\n\
                              R60\n\
                              L55\n\
                              L1\n\
                              L99\n\
                              R14\n\
                              L82";

    #[test]
    fn test_p1() {
        let data = generate(TEST_INPUT);
        let res = part1(&data);
        assert_eq!(res, 3);
    }

    #[test]
    fn test_p2() {
        let data = generate(TEST_INPUT);
        let res = part2(&data);
        assert_eq!(res, 6);
    }

    #[test]
    fn test_double_rotation_from_zero() {
        let data = generate(
            "L50\n\
                R200",
        );
        let res = part2(&data);
        assert_eq!(res, 3);
    }

    #[test]
    fn test_not_counting_move_from_zero() {
        let data = generate(
            "L50\n\
                L1",
        );
        let res = part2(&data);
        assert_eq!(res, 1);
    }
}
