use std::str::Lines;

use anyhow::{bail, Context, Result};
use include_aoc::include_aoc;

static INPUT: &str = include_aoc!(2023, 5);

solution!(INPUT, pt1, pt2);

#[derive(Debug)]
struct Transformer {
    maps: Vec<Map>,
}

#[derive(Debug)]
struct Map {
    src_range: std::ops::Range<i64>,
    delta: i64,
}

impl Transformer {
    fn transform(&self, initial: i64) -> i64 {
        for map in self.maps.iter() {
            if map.src_range.contains(&initial) {
                return initial + map.delta;
            }
        }
        initial
    }
    fn reverse(&self, transformed: i64) -> i64 {
        for map in self.maps.iter() {
            let initial = transformed - map.delta;
            if map.src_range.contains(&initial) {
                return initial;
            }
        }
        return transformed;
    }
}

fn parse_transformer(input: &mut Lines) -> Result<Transformer> {
    let _header = input.next().context("input is empty")?;

    let maps: Vec<_> = input
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let nums: Vec<_> = line
                .split_whitespace()
                .map(|num| num.trim().parse::<i64>().ok())
                .collect::<Option<_>>()?;
            if nums.len() != 3 {
                return None;
            }

            let src_range = nums[1]..nums[1] + nums[2];
            let delta = nums[0] - nums[1];
            return Some(Map { src_range, delta });
        })
        .collect::<Option<_>>()
        .context("unable to parse maps")?;

    Ok(Transformer { maps })
}

fn parse_seed_section(input: &mut Lines) -> Result<Vec<i64>> {
    input
        .next()
        .context("input is empty")?
        .trim()
        .strip_prefix("seeds: ")
        .context("unable to strip seed prefix")?
        .split_whitespace()
        .map(|num| num.trim().parse::<i64>().context("unable to parse seeds"))
        .collect()
}

fn pt1(input: &str) -> Result<i64> {
    let mut lines = input.trim().lines();
    let mut seeds = parse_seed_section(&mut lines)?;

    let _empty = lines
        .next()
        .context("empty line between seeds and maps is missing")?;

    let mut transformers = Vec::new();
    while let Ok(transformer) = parse_transformer(&mut lines) {
        transformers.push(transformer);
    }

    for transformer in transformers {
        for seed in seeds.iter_mut() {
            *seed = transformer.transform(*seed);
        }
    }

    seeds.into_iter().min().context("no minimum found")
}

fn pt2(input: &str) -> Result<i64> {
    let mut lines = input.trim().lines();
    let seed_nums = parse_seed_section(&mut lines)?;
    let seed_ranges: Vec<_> = seed_nums
        .chunks_exact(2)
        .map(|pair| pair[0]..pair[0] + pair[1])
        .collect();

    let _empty = lines
        .next()
        .context("empty line between seeds and maps is missing")?;

    let mut transformers = Vec::new();
    while let Ok(transformer) = parse_transformer(&mut lines) {
        transformers.push(transformer);
    }

    for location in 0..i32::MAX {
        // Get original seed value from prospective location
        let mut transformed = location as i64;
        for transformer in transformers.iter().rev() {
            transformed = transformer.reverse(transformed);
        }

        for range in seed_ranges.iter() {
            if range.contains(&transformed) {
                return Ok(location as i64);
            }
        }
    }

    bail!("lowest value higher than {}", i32::MAX)
}

#[cfg(test)]
mod test {
    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn pt1() {
        assert_eq!(super::pt1(INPUT).unwrap(), 35);
    }

    #[test]
    fn pt2() {
        assert_eq!(super::pt2(INPUT).unwrap(), 46);
    }

    #[test]
    fn real_input() {
        assert_eq!(super::pt1(super::INPUT).unwrap(), 331445006);
        assert_eq!(super::pt2(super::INPUT).unwrap(), 6472060);
    }
}
