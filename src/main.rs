use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn stream_lines<P> (filename: P) -> io::Result<io::Lines<io::BufReader<File>>> 
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn find_largest_group(lines: io::Lines<io::BufReader<File>>) -> usize {
    let mut current_sum: usize = 0;
    let mut max_sum: usize = 0;
    for line in lines {
        match line {
            Ok(line) => {
                let number = line.parse::<usize>();
                match number {
                    Ok(number) => current_sum += number,
                    Err(_) => {
                        if current_sum > max_sum {
                            max_sum = current_sum;
                        };
                        current_sum = 0;
                    }
                }
            },
            Err(line) => println!("{line}")
        }
    }
    max_sum
}


fn main() {
    println!("Read file...");
    let sample_path = String::from("/home/sascha/Documents/AdventOfCode/2022/01/sample.txt");


    let lines = stream_lines(&sample_path);
    match lines {
        Ok(lines) => {
            let max_sum = find_largest_group(lines);
            println!("{max_sum}");
        },
        Err(_) => println!("File '{sample_path}' does not exist!")
    }
}
