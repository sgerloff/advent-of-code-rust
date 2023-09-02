use std::path::PathBuf;

use clap::{arg, command, value_parser, Command};

mod task_01;


fn main() {
    let matches = command!()
    .subcommand(
        Command::new("task-01")
        .about("Advent of Code 2022 - Task 01")
        .arg(
            arg!(-f --file <FILE> "Path to input file")
            .required(true)
            .value_parser(value_parser!(PathBuf))
        )
        .arg(
            arg!(-n --topn "Number of largest groups to sum")
            .required(false)
            .value_parser(value_parser!(usize))
            .default_value("1")
        )
    )
    .get_matches();

    match matches.subcommand() {
        Some(("task-01", sub_matches)) => {
            task_01::run_task(
                sub_matches.get_one::<PathBuf>("file").unwrap(), 
                *sub_matches.get_one::<usize>("topn").unwrap()
            )
        },
        _ => println!("Should not be reached!")
    }
}