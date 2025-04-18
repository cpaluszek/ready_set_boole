use std::io::{self, Write};

use ready_set_boole::curve::map;

pub fn main() {
    loop {
        let x = match get_u16_input("X") {
            Some(value) => value,
            None => continue,
        };

        let y = match get_u16_input("Y") {
            Some(value) => value,
            None => continue,
        };

        let z = map(x, y);
        println!("Z-order mapping for ({}, {}): {}", x, y, z);
    }
}

fn get_u16_input(prompt: &str) -> Option<u16> {
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

    match input.trim().parse::<u16>() {
        Ok(value) => Some(value),
        Err(err) => {
            eprintln!("Error while parsing number for {}: {err:?}", prompt);
            None
        }
    }
}
