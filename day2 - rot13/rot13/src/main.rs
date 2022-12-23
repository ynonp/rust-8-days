mod utils;
use std::env;
use utils::rot13;

fn main() {
    let input_word = env::args().nth(1).expect("Usage: rot13 text-string");    
    let output = rot13(&input_word);

    println!("{}", output);
}
