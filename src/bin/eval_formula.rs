use ready_set_boole::eval_formula::*;

pub fn main() {
    for arg in std::env::args().skip(1) {
        println!("{}", arg);
        build_and_print_ast(&arg);
        println!("=> {}", eval_formula(&arg));
    }
}

