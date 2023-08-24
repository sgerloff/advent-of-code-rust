use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use top_n_counter::TopNGroupCounter;

pub mod top_n_counter;


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

fn main() {
    println!("Read file...");
    let sample_path = String::from("/home/sascha/Documents/AdventOfCode/2022/01/sample.txt");
    let lines = stream_lines(&sample_path);
    match lines {
        Ok(lines) => {
            let max_sums = process_file(lines, 3);
            for _sum in max_sums {
                println!("{_sum}");
            }
        },
        Err(_) => println!("File '{sample_path}' does not exist!")
    }
}
