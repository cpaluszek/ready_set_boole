use ready_set_boole::{build_and_print_expression, truth_table::print_truth_table};
use std::io::{self, Write};

pub fn main() {
    let mut input = String::new();

    loop {
        print!("Enter formula: ");
        io::stdout().flush().unwrap();

        match io::stdin().read_line(& mut input) {
            Ok(0) => break,
            Ok(_) => {
                let formula = input.trim();
                build_and_print_expression(formula);
                print_truth_table(formula);
                println!("");
                input.clear();
            },
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }
    }
}

