use std::process;

use std::io::{self, Write};


pub fn clear() {
    print!("\x1B[2J\x1B[H");
}


pub fn read_input(prompt: &str) -> String {
    let mut buffer = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}


pub fn continue_run(){
    print!("Premi invio per continuare..");

    let mut buffer = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();

    if buffer.trim().is_empty(){
        return
    }
}

