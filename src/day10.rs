use aoc_runner_derive::{aoc, aoc_generator};
use good_lp::{
    Expression, IntoAffineExpression, Solution, SolverModel, highs, variable, variables,
};
use itertools::Itertools;

#[derive(Clone, Default, Debug)]
pub struct MachineDescription {
    light_diagram: Vec<bool>,
    schematics: Vec<Vec<usize>>,
    joltage_req: Vec<usize>,
}

#[aoc_generator(day10)]
pub fn generate(s: &str) -> Option<Vec<MachineDescription>> {
    let mut res = vec![];

    for line in s.lines() {
        let mut desc = MachineDescription::default();

        let mut spl = line.split_ascii_whitespace().peekable();

        let diag = spl.next()?;
        let diag = diag.strip_prefix('[')?.strip_suffix(']')?;
        desc.light_diagram = diag.chars().map(|it| it == '#').collect_vec();

        while let Some(schematic) = spl.peek()
            && schematic.starts_with('(')
        {
            desc.schematics.push(
                spl.next()?
                    .strip_prefix('(')?
                    .strip_suffix(')')?
                    .split(',')
                    .filter_map(|it| it.parse::<usize>().ok())
                    .collect_vec(),
            );
        }

        let joltage = spl.next()?;
        desc.joltage_req = joltage
            .strip_prefix('{')?
            .strip_suffix('}')?
            .split(',')
            .filter_map(|it| it.parse::<usize>().ok())
            .collect_vec();

        res.push(desc);
    }

    Some(res)
}

#[aoc(day10, part1)]
pub fn part1(inp: &[MachineDescription]) -> usize {
    let mut button_presses = 0;

    for desc in inp {
        let initial_state = vec![false; desc.light_diagram.len()];
        if let Some(path) = pathfinding::prelude::bfs(
            &initial_state,
            |state| {
                let mut succs = vec![];

                for cfg in &desc.schematics {
                    let mut new_state = state.clone();
                    for &idx in cfg {
                        new_state[idx] = !state[idx];
                    }

                    succs.push(new_state);
                }

                succs
            },
            |state| state.eq(&desc.light_diagram),
        ) {
            button_presses += path.len() - 1;
        }
    }

    button_presses
}

#[aoc(day10, part2)]
#[allow(clippy::cast_precision_loss)]
pub fn part2(inp: &[MachineDescription]) -> Option<usize> {
    let mut result = 0;

    for desc in inp {
        // Variables for button presses:
        // index 0 presses button represented by schematic 0, etc
        let mut vars = variables!();
        let mut button_presses = vec![];
        for _ in 0..desc.schematics.len() {
            let variable = vars.add(variable().min(0).integer());
            button_presses.push(variable);
        }

        // minimise the number of total button presses
        let mut problem = highs(vars.minimise(button_presses.iter().sum::<Expression>()));

        // update the joltage on each index based on the number of button presses on each schematic
        let mut joltage_sum = vec![0.into_expression(); desc.joltage_req.len()];
        for (i, schematic) in desc.schematics.iter().enumerate() {
            for &x in schematic {
                joltage_sum[x] += button_presses[i];
            }
        }

        // add constraint so that each of the generated joltages equal the requirement
        for (e, &j) in joltage_sum.into_iter().zip(&desc.joltage_req) {
            problem.add_constraint(e.eq(j as f64));
        }

        // solve and sum button presses
        let sol = problem.solve().ok()?;
        result += button_presses.iter().map(|&v| sol.value(v)).sum::<f64>() as usize;
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}\n\
                              [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}\n\
                              [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_p1() {
        let data = generate(TEST_INPUT).expect("valid input");
        let res = part1(&data);
        assert_eq!(res, 7);
    }

    #[test]
    fn test_p2() {
        let data = generate(TEST_INPUT).expect("valid input");
        let res = part2(&data);
        assert_eq!(res, Some(33));
    }
}
