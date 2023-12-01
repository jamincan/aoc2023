pub type Solution = fn() -> Result<String, ()>;

macro_rules! solution {
    ($input:ident, $pt1:ident, $pt2:ident) => {
        pub const SOLUTION: [crate::Solution; 2] = [solution::part1, solution::part2];
        mod solution {
            pub fn part1() -> Result<String, ()> {
                super::$pt1(super::$input)
            }

            pub fn part2() -> Result<String, ()> {
                super::$pt2(super::$input)
            }
        }
    };
    ($input:ident, $pt1:ident) => {
        pub const SOLUTION: [Solution; 2] = [solution::part1, solution::part2];
        mod solution {
            fn part1() -> Result<String, ()> {
                super::$pt1(super::$input)
            }

            fn part2() -> Result<String, ()> {
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
    if let Ok(res) = result {
        println!("Solution for part {part} completed in {elapsed:.2?}:\n{res}");
    } else {
        println!("Solution for part {part} failed");
    }
}

pub mod day1;

pub const SOLUTIONS: &[[Solution; 2]] = &[day1::SOLUTION];
