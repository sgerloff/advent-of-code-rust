pub mod file_utils {
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::PathBuf;

    pub fn read_input(path: &Option<&PathBuf>) -> Box<dyn BufRead> {
        match path {
            Some(path) => {
                let file = File::open(path);
                match file {
                    Ok(file) => Box::new(io::BufReader::new(file)),
                    Err(line) => panic!("File does not exist {line}!")
                }
            },
            None => Box::new(io::BufReader::new(io::stdin().lock())),
        }
    }
}