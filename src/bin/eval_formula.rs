use ready_set_boole::evaluate::*;

pub fn main() {
    for arg in std::env::args().skip(1) {
        println!("{}", arg);
        build_and_print_ast(&arg);
        println!("=> {}", eval_formula(&arg));
    }
}

