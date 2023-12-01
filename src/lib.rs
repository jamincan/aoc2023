macro_rules! solution {
    ($input:ident, $pt1:ident, $pt2:ident) => {
        pub const SOLUTION: Day = Day;
        pub struct Day;

        impl super::Solution for Day {
            fn pt1(&self) -> Result<String, ()> {
                $pt1($input)
            }

            fn pt2(&self) -> Result<String, ()> {
                $pt2($input)
            }
        }
    };
    ($input:ident, $pt1:ident) => {
        pub const SOLUTION: Day = Day;
        pub struct Day;

        impl super::Solution for Day {
            fn pt1(&self) -> Result<String, ()> {
                $pt1($input)
            }

            fn pt2(&self) -> Result<String, ()> {
                todo!()
            }
        }
    };
}

pub mod day1;

pub trait Solution {
    fn pt1(&self) -> Result<String, ()>;
    fn pt2(&self) -> Result<String, ()>;
}

pub const SOLUTIONS: &[&dyn Solution] = &[&day1::SOLUTION];

pub fn run_solution(solution: &dyn Solution, part: u8) {
    use std::time::Instant;

    let now = Instant::now();
    let result = match part {
        1 => solution.pt1(),
        2 => solution.pt2(),
        _ => unimplemented!("part must be 1 or 2"),
    };
    let elapsed = now.elapsed();
    if let Ok(res) = result {
        println!("Solution for part {part} completed in {elapsed:.2?}:\n{res}");
    } else {
        println!("Solution for part {part} failed");
    }
}
