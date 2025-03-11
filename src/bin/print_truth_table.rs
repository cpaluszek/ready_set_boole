use ready_set_boole::{evaluate::build_and_print_ast, truth_table::print_truth_table};

pub fn main() {
    for arg in std::env::args().skip(1) {
        build_and_print_ast(&arg);
        print_truth_table(&arg);
    }
}

