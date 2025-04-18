use std::io::{self, Write};

use ready_set_boole::set::{has_duplicate, powerset};

pub fn main() {
    let mut input = String::new();

    loop {
        print!("Enter set: ");
        io::stdout().flush().unwrap();

        match io::stdin().read_line(& mut input) {
            Ok(0) => break,
            Ok(_) => {
                let set: Vec<i32> = match input.split_whitespace().map(|n| n.parse()).collect() {
                    Ok(numbers) => numbers,
                    Err(err) => {
                        eprintln!("Error while parsing numbers: {err:?}");
                        input.clear();
                        continue;
                    }
                };

                if has_duplicate(&set) {
                    eprintln!("Invalid set: duplicate");
                    continue;
                }

                for subset in powerset(set) {
                    println!("{subset:?}");
                }
                input.clear();
            },
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }
    }
}
