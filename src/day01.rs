use aoc_runner_derive::{aoc, aoc_generator};

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
    let mut current = 50;
    let mut hit_zero = 0;

    for &rot in inp {
        current += rot;
        hit_zero += isize::from((current % NUM_DIALS) == 0);
    }

    hit_zero
}

#[aoc(day01, part2)]
pub fn part2(inp: &[isize]) -> isize {
    let mut current = 50;
    let mut num_zero = 0;

    for rot in inp {
        let mut rot = *rot;
        while rot >= NUM_DIALS {
            num_zero += 1;
            rot -= NUM_DIALS;
        }

        while rot <= -NUM_DIALS {
            num_zero += 1;
            rot += NUM_DIALS;
        }

        let was_zero = current == 0;
        current += rot;

        num_zero += isize::from(!was_zero && (current <= 0 || current >= NUM_DIALS));

        if current < 0 {
            current += NUM_DIALS;
        } else if current >= NUM_DIALS {
            current -= NUM_DIALS;
        }
    }

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
