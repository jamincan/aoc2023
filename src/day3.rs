use anyhow::{Context, Result};
use itertools::Itertools;

pub const INPUT: &str = include_str!("input/day3.txt");

solution!(INPUT, pt1, pt2);

fn neighbour_indices(current: usize, width: usize, num_len: usize) -> impl Iterator<Item = usize> {
    let current = current as i32;
    let width = width as i32;
    let num_len = num_len as i32;

    let previous_row = (-width - 1)..=(-width + num_len);
    let current_row = [-1, num_len].into_iter();
    let next_row = (width - 1)..=(width + num_len);

    previous_row
        .chain(current_row)
        .chain(next_row)
        .map(move |offset| current + offset)
        // Ignore indices < 0
        .filter(|index| *index >= 0)
        // Ignore indices corresponding to a line break
        .filter(move |index| index % width != width - 1)
        .map(|index| index as usize)
}

fn pt1(input: &str) -> Result<u32> {
    // Normalize line breaks; if we just join the lines without the break, we might end up reading numbers across lines
    let input = input.lines().join("\n");

    let digit = regex::Regex::new(r"\d+").unwrap();
    let width = input.lines().next().context("input is empty")?.len() + 1; // Include newline
    let chars = input.chars().collect_vec();
    let mut valid = Vec::new();
    'outer: for m in digit.find_iter(&input) {
        let neighbours = neighbour_indices(m.start(), width, m.len());
        for neighbour in neighbours {
            if let Some('.' | '0'..='9') | None = chars.get(neighbour) {
                continue;
            } else {
                let num = m.as_str().parse::<u32>()?;
                valid.push(num);
                continue 'outer;
            }
        }
    }
    Ok(valid.iter().sum::<u32>())
}

fn pt2(input: &str) -> Result<u32> {
    // Normalize line breaks
    let input = input.lines().join("\n");

    let digit = regex::Regex::new(r"\d+").unwrap();
    let width = input.lines().next().context("input is empty")?.len() + 1; // Include newline
    let chars = input.chars().collect_vec();

    // Find all numbers adjacent to '*'
    let mut possible_gears = std::collections::BTreeMap::<usize, Vec<u32>>::new();
    for m in digit.find_iter(&input) {
        let neighbours = neighbour_indices(m.start(), width, m.len());
        for neighbour in neighbours {
            if let Some('*') = chars.get(neighbour) {
                let num = m.as_str().parse::<u32>()?;
                possible_gears
                    .entry(neighbour)
                    .and_modify(|gears| gears.push(num))
                    .or_insert(vec![num]);
            }
        }
    }

    Ok(possible_gears
        .values()
        .filter(|gears| gears.len() == 2)
        .map(|gears| gears.iter().product::<u32>())
        .sum::<u32>())
}

#[cfg(test)]
mod test {
    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn neighbours() {
        use super::neighbour_indices;
        use itertools::Itertools;
        // Example
        //  0  1  2  3  4  (5)
        //  6  7  8  9 10 (11)
        // 12 13 14 15 16 (17)

        let indices = neighbour_indices(0, 6, 3).collect_vec();
        assert_eq!(indices, vec![3, 6, 7, 8, 9]);
        let indices = neighbour_indices(1, 6, 3).collect_vec();
        assert_eq!(indices, vec![0, 4, 6, 7, 8, 9, 10]);
        let indices = neighbour_indices(6, 6, 1).collect_vec();
        assert_eq!(indices, vec![0, 1, 7, 12, 13]);
        let indices = neighbour_indices(10, 6, 1).collect_vec();
        assert_eq!(indices, vec![3, 4, 9, 15, 16]);
        let indices = neighbour_indices(15, 6, 2).collect_vec();
        assert_eq!(indices, vec![8, 9, 10, 14, 20, 21, 22]);
    }

    #[test]
    fn pt1() {
        assert_eq!(super::pt1(INPUT).unwrap(), 4361);
        assert_eq!(super::pt1(super::INPUT).unwrap(), 544433);
    }

    #[test]
    fn pt2() {
        assert_eq!(super::pt2(INPUT).unwrap(), 467835);
        assert_eq!(super::pt2(super::INPUT).unwrap(), 76314915);
    }
}
