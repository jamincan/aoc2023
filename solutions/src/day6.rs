use anyhow::{Context, Result};
use include_aoc::include_aoc;

static INPUT: &str = include_aoc!(2023, 6);

solution!(INPUT, pt1, pt2);

fn pt1(input: &str) -> Result<i64> {
    let mut lines = input.trim().lines();
    let times = lines.next().context("missing first line")?;
    let times = parse_numbers_with_prefix(times, "Time:")?;
    let distances = lines.next().context("missing second line")?;
    let distances = parse_numbers_with_prefix(distances, "Distance:")?;
    let races = times.into_iter().zip(distances.into_iter());
    let intervals = races.map(winning_time_interval);
    let ways_to_win_count = intervals.map(|(t1, t2)| t2 - t1 + 1);
    Ok(ways_to_win_count.product())
}

fn parse_numbers_with_prefix(input: &str, prefix: &str) -> Result<Vec<f64>> {
    let input = input
        .trim()
        .strip_prefix(prefix)
        .with_context(|| format!("missing prefix '{prefix}'"))?;
    let times = input
        .trim()
        .split_whitespace()
        .map(str::parse)
        .collect::<Result<_, _>>()?;
    Ok(times)
}

fn roots(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let base = b.powi(2) - 4. * a * c;
    if base < 0. {
        return None;
    }
    let root = base.powf(0.5);
    let x1 = (-b + root) / (2. * a);
    let x2 = (-b - root) / (2. * a);
    Some((x1.min(x2), x1.max(x2)))
}

fn winning_time_interval(race: (f64, f64)) -> (i64, i64) {
    // initial velocity = vi; wait = w; distance = d; total time = t;
    // vi = w
    // d = vi(t - w)
    // w^2 - tw + d = 0
    let (t, d) = race;
    let a = 1.;
    let b = -t;
    let c = d;
    let (t1, t2) = roots(a, b, c).expect("no roots");

    // Use floor + 1 and ceil - 1 to ensure that interval exceeds record matching times when those times are exact integers
    (t1.floor() as i64 + 1, t2.ceil() as i64 - 1)
}

fn parse_number_with_prefix(input: &str, prefix: &str) -> Result<f64> {
    let input = input
        .trim()
        .strip_prefix(prefix)
        .with_context(|| format!("missing prefix '{prefix}'"))?;
    let number: String = input.trim().chars().filter(char::is_ascii_digit).collect();
    let number = number.parse()?;
    Ok(number)
}

fn pt2(input: &str) -> Result<i64> {
    let mut lines = input.trim().lines();
    let time = lines.next().context("missing first line")?;
    let time = parse_number_with_prefix(time, "Time:")?;
    let distance = lines.next().context("missing second line")?;
    let distance = parse_number_with_prefix(distance, "Distance:")?;
    let (t1, t2) = winning_time_interval((time, distance));
    Ok(t2 - t1 + 1)
}

#[cfg(test)]
mod test {
    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn winning_time_interval() {
        use super::winning_time_interval as wti;
        assert_eq!(wti((7., 9.)), (2, 5));
        assert_eq!(wti((15., 40.)), (4, 11));
        assert_eq!(wti((30., 200.)), (11, 19));
        assert_eq!(wti((71530., 940200.)), (14, 71516));
    }

    #[test]
    fn pt1() {
        assert_eq!(super::pt1(INPUT).unwrap(), 288);
    }

    #[test]
    fn pt2() {
        assert_eq!(super::pt2(INPUT).unwrap(), 71503);
    }

    #[test]
    fn real_input() {
        assert_eq!(super::pt1(super::INPUT).unwrap(), 2344708);
        assert_eq!(super::pt2(super::INPUT).unwrap(), 30125202);
    }
}
