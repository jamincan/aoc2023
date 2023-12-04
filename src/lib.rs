pub type Solution = fn() -> anyhow::Result<String>;
pub type SolutionSet = (Solution, Option<Solution>);

macro_rules! solution {
    ($input:ident, $pt1:ident, $pt2:ident) => {
        pub const SOLUTION: crate::SolutionSet = (solution::part1, Some(solution::part2));
        mod solution {
            pub fn part1() -> anyhow::Result<String> {
                super::$pt1(super::$input).map(|res| res.to_string())
            }

            pub fn part2() -> anyhow::Result<String> {
                super::$pt2(super::$input).map(|res| res.to_string())
            }
        }
    };
    ($input:ident, $pt1:ident) => {
        pub const SOLUTION: crate::SolutionSet = (solution::part1, None);
        mod solution {
            pub fn part1() -> anyhow::Result<String> {
                super::$pt1(super::$input).map(|res| res.to_string())
            }
        }
    };
}

pub fn run_solution(day: u8, part: u8) {
    use std::time::Instant;
    let solution = match (part, SOLUTIONS[day as usize - 1]) {
        (1, (pt1, _)) => pt1,
        (2, (_, Some(pt2))) => pt2,
        (2, (_, None)) => {
            println!("Solution for part 2 not yet implemented");
            return;
        }
        (_, _) => panic!("part must be 1 or 2"),
    };

    let now = Instant::now();
    let result = solution();
    let elapsed = now.elapsed();
    match result {
        Ok(res) => println!("Solution for part {part} completed in {elapsed:.2?}:\n{res}"),
        Err(err) => println!("Solution for part {part} failed:\n{err}"),
    }
}

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

pub const SOLUTIONS: &[SolutionSet] = &[
    day1::SOLUTION,
    day2::SOLUTION,
    day3::SOLUTION,
    day4::SOLUTION,
];
