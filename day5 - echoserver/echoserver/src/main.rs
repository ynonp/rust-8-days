use std::io::{Write, BufReader, BufRead, BufWriter};
use std::net::TcpListener;
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9123").unwrap();
    println!("listening started, ready to accept");
    for stream in listener.incoming() {
        thread::spawn(|| {            
            let stream = match stream {
                Ok(s) => s,
                Err(_) => return,
            };

            let mut reader = BufReader::new(&stream);
            let mut writer = BufWriter::new(&stream);

            loop {
                let mut line = String::new();
                match reader.read_line(&mut line) {
                    Ok(0) => return,
                    Ok(l) => l,
                    Err(_) => return,
                };
    
                let output = format!("You Said: {}", &line);
                
                if let Err(_) = writer.write(output.as_bytes()) {
                    return;
                }
                if let Err(_) = writer.flush() {
                    return;
                }
            }
        });
    }
}