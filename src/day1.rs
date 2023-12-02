pub const INPUT: &str = include_str!("input/day1.txt");

solution!(INPUT, pt1, pt2);

fn pt1_parse(value: &str) -> Option<u32> {
    // Get iterator of decimal digits in the string
    let first = value.chars().find_map(|c| c.to_digit(10))?;
    let last = value.chars().rev().find_map(|c| c.to_digit(10))?;
    Some(first * 10 + last)
}

fn pt1(input: &str) -> Option<u32> {
    input
        .lines()
        .map(pt1_parse)
        .try_fold(0, |sum, c| c.map(|c| sum + c))
}

fn parse_digit(input: &str, reverse: bool) -> Option<u32> {
    let test_fn = if !reverse {
        str::starts_with
    } else {
        str::ends_with
    };
    if test_fn(input, "one") || test_fn(input, "1") {
        Some(1)
    } else if test_fn(input, "two") || test_fn(input, "2") {
        Some(2)
    } else if test_fn(input, "three") || test_fn(input, "3") {
        Some(3)
    } else if test_fn(input, "four") || test_fn(input, "4") {
        Some(4)
    } else if test_fn(input, "five") || test_fn(input, "5") {
        Some(5)
    } else if test_fn(input, "six") || test_fn(input, "6") {
        Some(6)
    } else if test_fn(input, "seven") || test_fn(input, "7") {
        Some(7)
    } else if test_fn(input, "eight") || test_fn(input, "8") {
        Some(8)
    } else if test_fn(input, "nine") || test_fn(input, "9") {
        Some(9)
    } else {
        None
    }
}

fn pt2_parse(input: &str) -> Option<u32> {
    let mut start = 0;
    let mut end = input.len();
    let mut first: Option<u32> = None;
    while end > 0 || start < input.len() {
        if let Some(f) = first {
            if let Some(l) = parse_digit(&input[0..end], true) {
                return Some(10 * f + l);
            }
            end -= 1;
        } else {
            first = parse_digit(&input[start..], false);
            start += 1;
        }
    }
    return None;
}

fn pt2(input: &str) -> Option<u32> {
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
        assert_eq!(super::pt1(INPUT1), Some(142));
    }

    #[test]
    fn pt2() {
        assert_eq!(super::pt2(INPUT2), Some(281));
    }
}
