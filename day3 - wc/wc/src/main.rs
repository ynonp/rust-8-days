use std::env;
use std::fs;
use std::io::{self, BufRead, BufReader};

fn main() {
    let input = env::args().nth(1);
    let reader: Box<dyn BufRead> = match input {
        None => Box::new(BufReader::new(io::stdin())),
        Some(filename) => Box::new(BufReader::new(fs::File::open(filename).unwrap()))
    };

    let mut line_count: usize = 0;
    let mut char_count: usize = 0;
    let mut words_count: usize = 0;

    for line in reader.lines() {        
        let the_line = line.unwrap();
        line_count += 1;
        words_count += the_line.split_whitespace().count();
        char_count += the_line.chars().count() + 1; // +1 because of the newline
    }

    println!("{} {} {}", line_count, words_count, char_count);
}
