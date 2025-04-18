use std::io::{self, Write};

use ready_set_boole::curve::reverse_map;

pub fn main() {
    loop {
        let z = match get_f64_input("Z") {
            Some(value) => value,
            None => continue,
        };

        let (x, y) = reverse_map(z);
        println!("Reverse Z-order mapping for {}: ({}, {})", z, x, y);
    }
}

fn get_f64_input(prompt: &str) -> Option<f64> {
    let mut input = String::new();

    print!("{}: ", prompt);
    io::stdout().flush().unwrap();

    match io::stdin().read_line(&mut input) {
        Ok(0) => return None, // EOF
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error reading input: {}", e);
            return None;
        }
    }

    match input.trim().parse::<f64>() {
        Ok(value) => Some(value),
        Err(err) => {
            eprintln!("Error while parsing number for {}: {err:?}", prompt);
            None
        }
    }
}
