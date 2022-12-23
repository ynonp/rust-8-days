use clap::{Parser};
use std::io::{BufReader, BufRead, stdin};
use std::fs;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    name: Option<String>,

    #[arg(short)]
    reverse: bool,

    #[arg(short='n')]
    lines: Option<String>,
}

fn tail_file(filename: Option<&str>, lines_count: usize, reverse: bool) {    
    let mut last_lines: Vec<String> = Vec::with_capacity(lines_count);

    let reader: Box<dyn BufRead> = match filename {
        None => Box::new(BufReader::new(stdin())),
        Some(filename) => Box::new(BufReader::new(fs::File::open(filename).unwrap()))
    };

    let mut idx = 0;
    for line in reader.lines() {        
        if idx < last_lines.len() {
            last_lines[idx] = line.unwrap();
        } else {
            last_lines.push(line.unwrap());
        }
        idx = (idx + 1) % lines_count;
    }

    for i in 0..lines_count {
        if reverse {
            let j = lines_count - (i + 1);
            println!("[{}] {}", j, last_lines[(idx + j) % lines_count]);
        } else {
            println!("[{}] {}", i, last_lines[(idx + i) % lines_count]);
        }
        
    }
}

fn main() {
    let cli = Cli::parse();

    // You can check the value provided by positional arguments, or option arguments    
    let lines: usize = cli.lines.unwrap_or(String::from("10")).parse().expect("n is not a number");

    println!("Value for reverse: {}", cli.reverse);
    println!("Value for n = {}", lines);
    println!("name: {:?}", cli.name.as_deref());

    tail_file(cli.name.as_deref(), lines, cli.reverse);
}




