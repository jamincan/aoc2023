pub type Solution = fn() -> Option<String>;

macro_rules! solution {
    ($input:ident, $pt1:ident, $pt2:ident) => {
        pub const SOLUTION: [crate::Solution; 2] = [solution::part1, solution::part2];
        mod solution {
            pub fn part1() -> Option<String> {
                super::$pt1(super::$input).map(|res| res.to_string())
            }

            pub fn part2() -> Option<String> {
                super::$pt2(super::$input).map(|res| res.to_string())
            }
        }
    };
    ($input:ident, $pt1:ident) => {
        pub const SOLUTION: [crate::Solution; 2] = [solution::part1, solution::part2];
        mod solution {
            pub fn part1() -> Option<String> {
                super::$pt1(super::$input).map(|res| res.to_string())
            }

            pub fn part2() -> Option<String> {
                todo!()
            }
        }
    };
}

pub fn run_solution(solution: [Solution; 2], part: u8) {
    use std::time::Instant;
    let solution = solution[part as usize - 1];

    let now = Instant::now();
    let result = solution();
    let elapsed = now.elapsed();
    if let Some(res) = result {
        println!("Solution for part {part} completed in {elapsed:.2?}:\n{res}");
    } else {
        println!("Solution for part {part} failed");
    }
}

pub mod day1;
pub mod day2;

pub const SOLUTIONS: &[[Solution; 2]] = &[day1::SOLUTION, day2::SOLUTION];
