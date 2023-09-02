use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

use advent_of_code_2022_rust::top_n_counter::TopNGroupCounter;


fn stream_lines<P> (filename: P) -> io::Result<io::Lines<io::BufReader<File>>> 
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn apply_top_n_group_counter(line: String, counter: &mut TopNGroupCounter) {
    let number = line.parse::<usize>();
    match number {
        Ok(number) => counter.add(number),
        Err(_) => counter.complete()
    }
}


fn process_file(lines: io::Lines<io::BufReader<File>>, top_n: usize) -> Vec<usize> {
    let mut counter = TopNGroupCounter::new(top_n);
    for line in lines {
        match line {
            Ok(line) => apply_top_n_group_counter(line, &mut counter),
            Err(line) => println!("{line}")
        }
    }
    return counter.top_sums;
}


pub fn run_task(path: &PathBuf, top_n: usize) {
    let lines = stream_lines(&path);
    match lines {
        Ok(lines) => {
            let max_sums = process_file(lines, top_n);
            let mut total_sum: usize = 0;
            for _sum in max_sums {
                total_sum += _sum;
                println!("{_sum}");
            }
            println!("Total sum is: {total_sum}");
        },
        Err(_) => println!("File '{:?}' does not exist!", path.to_str())
    }
}