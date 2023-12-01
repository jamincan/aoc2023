pub const INPUT: &str = include_str!("input/day1.txt");

solution!(INPUT, pt1, pt2);

fn pt1_parse(value: &str) -> Option<u32> {
    // Get iterator of decimal digits in the string
    let mut digits = value.chars().filter_map(|c| c.to_digit(10));
    let first = digits.next()?;
    let last = digits.last().unwrap_or(first);
    Some(first * 10 + last)
}

fn pt1(input: &str) -> Result<String, ()> {
    let sum = input
        .lines()
        .map(pt1_parse)
        .try_fold(0, |sum, c| c.map(|c| sum + c))
        .ok_or(())?;
    Ok(format!("{sum}"))
}

fn parse_digit(input: &str) -> Option<u32> {
    let mut chars = input.chars();
    if let Some(digit) = chars.next()?.to_digit(10) {
        return Some(digit);
    }
    if input.starts_with("one") {
        Some(1)
    } else if input.starts_with("two") {
        Some(2)
    } else if input.starts_with("three") {
        Some(3)
    } else if input.starts_with("four") {
        Some(4)
    } else if input.starts_with("five") {
        Some(5)
    } else if input.starts_with("six") {
        Some(6)
    } else if input.starts_with("seven") {
        Some(7)
    } else if input.starts_with("eight") {
        Some(8)
    } else if input.starts_with("nine") {
        Some(9)
    } else {
        None
    }
}

fn pt2_parse(value: &str) -> Option<u32> {
    let mut input = value;
    let mut digits = vec![];
    while !input.is_empty() {
        if let Some(digit) = parse_digit(input) {
            digits.push(digit);
        }
        input = &input[1..];
    }
    let first = digits.first()?;
    let last = digits.last().expect("first exists so last does too");
    Some(10 * *first + *last)
}

fn pt2(input: &str) -> Result<String, ()> {
    let sum = input
        .lines()
        .map(pt2_parse)
        .try_fold(0, |sum, c| c.map(|c| sum + c))
        .ok_or(())?;
    Ok(format!("{sum}"))
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
        assert_eq!(super::pt1(INPUT1), Ok(String::from("142")));
    }

    #[test]
    fn pt2() {
        assert_eq!(super::pt2(INPUT2), Ok(String::from("281")));
    }
}
