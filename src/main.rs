use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


// fn iterate_lines(corpus: &String) -> Vec<u32> {
//     let bytes = corpus.as_bytes();

//     let mut sums = Vec::<u32>::new();

//     let mut start_id = 0;
//     let mut current_sum: u32 = 0;
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b'\n' {
//             let line = &corpus[start_id..i];
//             let parse_result = line.to_string().parse::<u32>();
//             match parse_result {
//                 Ok(parse_result) => {
//                     current_sum = current_sum + parse_result;
//                 },
//                 Err(_) => {
//                     sums.push(current_sum);
//                     current_sum = 0;
//                 }              
//             };
//             start_id = i + 1 
//         }
//     }
//     sums
// }


fn stream_lines<P> (filename: P) -> io::Result<io::Lines<io::BufReader<File>>> 
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn main() {
    println!("Read file...");
    let sample_path = String::from("/home/sascha/Documents/AdventOfCode/2022/01/sample.txt");
    let mut max_sum: usize = 0;
    let mut current_sum: usize = 0;
    if let Ok(lines) = stream_lines(sample_path) {
        for line in lines {
            if let Ok(line) = line {
                let number = line.parse::<usize>();
                match number {
                    Ok(number) => {
                        current_sum += number;
                    },
                    Err(_) => {
                        if current_sum > max_sum {
                            max_sum = current_sum;
                        };
                        current_sum = 0;
                    }
                };
            };
        };
    };
    println!("{max_sum}");
}
