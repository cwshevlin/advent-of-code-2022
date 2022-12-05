use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

pub fn read_lines<P>(filename: P) -> io::Result<Vec<String>> where P: AsRef<Path> {
    println!("{:?}", filename.as_ref());
    let file_result = File::open(filename)?;
    Ok(io::BufReader::new(file_result).lines().filter_map(|line| {
        println!("{:?}", line);
        line.ok()
    }).collect())
}