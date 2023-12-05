use aoc2023::{run_solution, SOLUTIONS};
use clap::{arg, command, value_parser, ArgAction};

fn main() {
    let matches = command!()
        .arg(
            arg!(<DAY>)
                .help("Which days solution to run")
                .value_parser(value_parser!(u8).range(1..=(SOLUTIONS.len() as i64)))
                .required(true),
        )
        .arg(
            arg!(--part)
                .short('p')
                .help("Which part of the solution to run")
                .default_values(["1", "2"])
                .value_parser(value_parser!(u8).range(1..=2))
                .action(ArgAction::Append),
        )
        .get_matches();

    let dir = include_aoc::cache_dir!();
    println!("Dir: {dir}");

    let day = *matches.get_one::<u8>("DAY").unwrap();
    let parts = matches
        .get_many::<u8>("part")
        .unwrap_or_default()
        .copied()
        .collect::<Vec<_>>();

    for part in parts {
        run_solution(day, part)
    }
}
