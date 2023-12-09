use std::{collections::HashMap, str::Lines};

use anyhow::{Context, Result};

use include_aoc::include_aoc;

static INPUT: &str = include_aoc!(2023, 8);

solution!(INPUT, pt1, pt2);

#[derive(Clone, Copy)]
enum Instruction {
    Left,
    Right
}

fn parse_instructions(input: &str) -> Result<Vec<Instruction>> {
    let instructions: Vec<Instruction> = input.trim().chars().map(|char| {
        match char {
            'L' => Ok(Instruction::Left),
            'R' => Ok(Instruction::Right),
            _ => Err(anyhow::format_err!("invalid instruction '{char}'"))
        }
    }).collect::<Result<_>>()?;

    Ok(instructions)
}

fn parse_nodes(input: Lines) -> Result<HashMap<&str, (&str, &str)>> {
    let mut nodes = HashMap::new();
    for line in input {
        let (id, next_elements) = line.split_once('=').context("unable to separate id from elements")?;
        let id = id.trim();
        let next_elements = next_elements.trim().strip_prefix('(').context("no opening parenthesis")?;
        let next_elements = next_elements.strip_suffix(')').context("no closing parenthesis")?;
        let (a, b) = next_elements.split_once(", ").context("failed to split elements")?;
        nodes.insert(id, (a, b));
    }
    Ok(nodes)
}

fn calculate_steps(nodes: &HashMap<&str, (&str, &str)>, mut instructions: impl Iterator<Item = Instruction>, start: &str, end_test: fn(&str) -> bool) -> Result<u64> {
    let mut current = start;
    let mut steps = 0;
    while !end_test(current) {
        steps += 1;
        let (left, right) = nodes.get(&current).with_context(|| format!("node '{current}' not found"))?;
        match instructions.next().expect("instructions should cycle forever") {
            Instruction::Left => current = *left,
            Instruction::Right => current = *right,
        }
    }
    Ok(steps)
}

fn pt1(input: &str) -> Result<u64> {
    let mut lines = input.trim().lines();
    let instructions = parse_instructions(lines.next().context("empty input")?)?;
    let instructions = instructions.into_iter().cycle();
    let _empty_line = lines.next().context("no empty line separating instructions")?;
    let nodes = parse_nodes(lines)?;

    calculate_steps(&nodes, instructions, "AAA", |id| id == "ZZZ")
}

fn pt2(input: &str) -> Result<u64> {
    let mut lines = input.trim().lines();
    let instructions = parse_instructions(lines.next().context("empty input")?)?;
    let instructions = instructions.into_iter().cycle();
    let _empty_line = lines.next().context("no empty line separating instructions")?;
    let nodes = parse_nodes(lines)?;

    let starting_node_steps: Vec<_> = nodes
        .keys()
        .filter(|id| id.ends_with('A'))
        .map(|node| {
            calculate_steps(&nodes, instructions.clone(), *node, |node| node.ends_with('Z'))
        })
        .collect::<Result<_>>()?;

    let lcm = starting_node_steps.into_iter().reduce(|lcm, steps| num::Integer::lcm(&lcm, &steps)).context("no starting nodes")?;
    Ok(lcm)
}

#[cfg(test)]
mod test {
    const INPUT1: &str = "RL

    AAA = (BBB, CCC)
    BBB = (DDD, EEE)
    CCC = (ZZZ, GGG)
    DDD = (DDD, DDD)
    EEE = (EEE, EEE)
    GGG = (GGG, GGG)
    ZZZ = (ZZZ, ZZZ)";

    const INPUT2: &str = "LLR

    AAA = (BBB, BBB)
    BBB = (AAA, ZZZ)
    ZZZ = (ZZZ, ZZZ)";

    const INPUT3: &str = "LR

    11A = (11B, XXX)
    11B = (XXX, 11Z)
    11Z = (11B, XXX)
    22A = (22B, XXX)
    22B = (22C, 22C)
    22C = (22Z, 22Z)
    22Z = (22B, 22B)
    XXX = (XXX, XXX)";

    #[test]
    fn pt1() {
        assert_eq!(super::pt1(INPUT1).unwrap(), 2);
        assert_eq!(super::pt1(INPUT2).unwrap(), 6);
    }

    #[test]
    fn pt2() {
        assert_eq!(super::pt2(INPUT3).unwrap(), 6);
    }

    #[test]
    fn real_input() {
        assert_eq!(super::pt1(super::INPUT).unwrap(), 19631);
        assert_eq!(super::pt2(super::INPUT).unwrap(), 21003205388413);
    }
}
