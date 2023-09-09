use std::io::{self, BufRead, stdout};
use std::path::PathBuf;
use is_terminal::IsTerminal;

use advent_of_code_2022_rust::top_n_counter::TopNGroupCounter;
use advent_of_code_2022_rust::utils::file_utils::read_input;


fn apply_top_n_group_counter(line: String, counter: &mut TopNGroupCounter) {
    let number = line.parse::<usize>();
    match number {
        Ok(number) => counter.add(number),
        Err(_) => counter.complete()
    }
}

fn process_file(lines: io::Lines<Box<dyn BufRead>>, top_n: usize) -> Vec<usize> {
    let mut counter = TopNGroupCounter::new(top_n);
    for line in lines {
        match line {
            Ok(line) => apply_top_n_group_counter(line, &mut counter),
            Err(line) => eprintln!("{line}")
        }
    }
    return counter.top_sums;
}

fn update_total(total: &usize, value: &usize, index: &usize) -> usize {
    let updated_total = total + value;
    if stdout().is_terminal() {
        let offset_index: usize = index + 1;
        println!("Top {offset_index} elf: {value} [calories]");
    }
    updated_total
}

fn flush_result(total: &usize) {
    if stdout().is_terminal() {
        println!("Total: {total} [calories]");
    } else {
        println!("{total}")
    }
}

pub fn run_task(path: Option<&PathBuf>, top_n: usize) {
    let lines = read_input(&path).lines();

    let max_sums = process_file(lines, top_n);

    let mut total_sum: usize = 0;
    for (index, sum) in max_sums.iter().enumerate() {
        total_sum = update_total(&total_sum, &sum, &index);
    }

    flush_result(&total_sum);
}
