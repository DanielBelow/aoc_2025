use aoc_runner_derive::{aoc, aoc_generator};
use geo::Contains;
use geo_types::{Coord, LineString, Polygon, Rect};
use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};

#[aoc_generator(day09)]
pub fn generate(s: &str) -> Option<Vec<(isize, isize)>> {
    let mut res = vec![];

    for line in s.lines() {
        let mut spl = line.split(',');
        let lhs = spl.next().and_then(|it| it.parse::<isize>().ok())?;
        let rhs = spl.next().and_then(|it| it.parse::<isize>().ok())?;

        res.push((lhs, rhs));
    }

    Some(res)
}

const fn area((lx, ly): (isize, isize), (rx, ry): (isize, isize)) -> usize {
    (lx.abs_diff(rx) + 1) * (ly.abs_diff(ry) + 1)
}

#[aoc(day09, part1)]
pub fn part1(inp: &[(isize, isize)]) -> Option<usize> {
    inp.iter()
        .combinations(2)
        .map(|it| area(*it[0], *it[1]))
        .max()
}

#[aoc(day09, part2)]
#[allow(clippy::cast_precision_loss)]
pub fn part2(inp: &[(isize, isize)]) -> Option<usize> {
    let coords = inp
        .iter()
        .map(|&(l, r)| Coord {
            x: l as f64,
            y: r as f64,
        })
        .collect_vec();

    let poly = Polygon::new(LineString(coords.clone()), vec![]);

    coords
        .iter()
        .combinations(2)
        .par_bridge()
        .filter(|it| {
            let rect = Rect::new(*it[0], *it[1]);
            poly.contains(&rect)
        })
        .map(|it| {
            let Coord { x: x1, y: y1 } = it[0];
            let Coord { x: x2, y: y2 } = it[1];
            area((*x1 as isize, *y1 as isize), (*x2 as isize, *y2 as isize))
        })
        .max()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "7,1\n\
                              11,1\n\
                              11,7\n\
                              9,7\n\
                              9,5\n\
                              2,5\n\
                              2,3\n\
                              7,3";

    #[test]
    fn test_p1() {
        let data = generate(TEST_INPUT).expect("valid input");
        let res = part1(&data);
        assert_eq!(res, Some(50));
    }

    #[test]
    fn test_p2() {
        let data = generate(TEST_INPUT).expect("valid input");
        let res = part2(&data);
        assert_eq!(res, Some(24));
    }
}
