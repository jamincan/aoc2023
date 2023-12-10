use anyhow::{Context, Result};
use include_aoc::include_aoc;
use itertools::Itertools;

static INPUT: &str = include_aoc!(2023, 9);

solution!(INPUT, pt1, pt2);

fn next_level(value: &[i32]) -> Vec<i32> {
    value.iter().tuple_windows().map(|(a, b)| b - a).collect()
} 

fn extrapolate<const PART: u8>(history: &[i32]) -> i32 {
    assert!(PART >= 1 && PART <= 2);
    assert!(history.len() > 1);
    
    if history.iter().all_equal() {
        *history.first().unwrap()
    } else {
        let next_level = next_level(history);
        let result = extrapolate::<PART>(&next_level);
        if PART == 1 {
            result + history.last().unwrap()
        } else {
            history.first().unwrap() - result
        }
    }
}

fn parse_history(input: &str) -> Result<Vec<i32>> {
    let nums = input.trim().split_whitespace().map(|s| s.parse()).collect::<Result<_, _>>().with_context(|| format!("failed to parse history '{input}'"))?;
    Ok(nums)
}

fn pt1(input: &str) -> Result<i32> {
    let histories: Vec<_> = input.trim().lines().map(parse_history).collect::<Result<_>>()?;
    let next_values = histories.iter().map(|history| extrapolate::<1>(history));
    Ok(next_values.sum())
}

fn pt2(input: &str) -> Result<i32> {
    let histories: Vec<_> = input.trim().lines().map(parse_history).collect::<Result<_>>()?;
    let next_values = histories.iter().map(|history| extrapolate::<2>(history));
    Ok(next_values.sum())
}

#[cfg(test)]
mod test {
    const INPUT: &str = "0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45";

    #[test]
    fn extrapolate_pt1() {
        let input = vec![0,3,6,9,12,15];
        assert_eq!(super::extrapolate::<1>(&input), 18);
        let input = vec![1,3,6,10,15,21];
        assert_eq!(super::extrapolate::<1>(&input), 28);
        let input = vec![10,13,16,21,30,45];
        assert_eq!(super::extrapolate::<1>(&input), 68);
    }
    #[test]
    fn extrapolate_pt2() {
        let input = vec![10, 13, 16, 21, 30, 45];
        assert_eq!(super::extrapolate::<2>(&input), 5)
    }

    #[test]
    fn pt1() {
        assert_eq!(super::pt1(INPUT).unwrap(), 114);
    }

    #[test]
    fn pt2() {
        assert_eq!(super::pt2(INPUT).unwrap(), 2);
    }

    #[test]
    fn real_input() {
        assert_eq!(super::pt1(super::INPUT).unwrap(), 2098530125);
        assert_eq!(super::pt2(super::INPUT).unwrap(), 1016);
    }
}
