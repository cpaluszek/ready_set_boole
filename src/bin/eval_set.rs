use ready_set_boole::{build_and_print_expression, set::{eval_set, has_duplicate}};
use std::io::{self, Write};

pub fn main() {

    loop {
        if let Some(formula) = read_formula() {
            let sets = read_sets();
            if !sets.is_empty() {
                build_and_print_expression(&formula);
                println!("{:?}", eval_set(&formula, sets));
            }
        }
    }
}

fn read_formula() -> Option<String> {
    print!("Enter formula: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    match io::stdin().read_line(& mut input) {
        Ok(0) => None,
        Ok(_) => Some(input.trim().to_string()),
        Err(e) => {
            eprintln!("Error reading input: {}", e);
            None
        }
    }
}

fn read_sets() -> Vec<Vec<i32>> {
    println!("Enter one set per line, empty line to stop");
    let mut sets = Vec::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        let set: Vec<i32> = match io::stdin().read_line(& mut input) {
            Ok(0) => break,
            Ok(_) => {
                if input == "\n" {
                    break;
                }

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
                set.to_vec()
            },
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                continue;
            }
        };
        sets.push(set);
    }
    sets
}

