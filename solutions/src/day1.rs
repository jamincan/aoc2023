use anyhow::{Context, Result};
use include_aoc::include_aoc;

static INPUT: &str = include_aoc!(2023, 1);

solution!(INPUT, pt1, pt2);

fn pt1_parse(value: &str) -> Result<u32> {
    let mut chars = value.chars();
    let first = chars
        .find_map(|c| c.to_digit(10))
        .with_context(|| format!("no first digit found in '{value}'"))?;
    let last = chars.rev().find_map(|c| c.to_digit(10)).unwrap_or(first);
    Ok(first * 10 + last)
}

fn pt1(input: &str) -> Result<u32> {
    input
        .lines()
        .map(pt1_parse)
        .try_fold(0, |sum, c| c.map(|c| sum + c))
}

fn pt2_parse(input: &str) -> Result<u32> {
    use aho_corasick::AhoCorasick;
    use once_cell::sync::Lazy;

    static DIGITS: Lazy<AhoCorasick> = Lazy::new(|| {
        AhoCorasick::new(&[
            r"1", r"2", r"3", r"4", r"5", r"6", r"7", r"8", r"9", r"one", r"two", r"three",
            r"four", r"five", r"six", r"seven", r"eight", r"nine",
        ])
        .unwrap()
    });
    let mut digits = DIGITS.find_overlapping_iter(input);
    let first = digits
        .next()
        .map(|m| m.pattern().as_u32() % 9 + 1)
        .with_context(|| format!("no digit found in '{input}'"))?;
    let last = digits
        .last()
        .map(|m| m.pattern().as_u32() % 9 + 1)
        .unwrap_or(first);
    Ok((10 * first + last) as u32)
}

fn pt2(input: &str) -> Result<u32> {
    input
        .lines()
        .map(pt2_parse)
        .try_fold(0, |sum, c| c.map(|c| sum + c))
}

#[cfg(test)]
mod test {
    const INPUT1: &str = "1abc2
                        pqr3stu8vwx
                        a1b2c3d4e5f
                        treb7uchet";

    const INPUT2: &str = "two1nine
                        eightwothree
                        abcone2threexyz
                        xtwone3four
                        4nineeightseven2
                        zoneight234
                        7pqrstsixteen";

    #[test]
    fn pt1() {
        assert_eq!(super::pt1(INPUT1).unwrap(), 142);
    }

    #[test]
    fn pt2() {
        assert_eq!(super::pt2(INPUT2).unwrap(), 281);
    }
}
