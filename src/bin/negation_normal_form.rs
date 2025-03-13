use ready_set_boole::negation_normal_form::negation_normal_form;

pub fn main() {
    for arg in std::env::args().skip(1) {
        println!("{}", arg);
        let nnf = negation_normal_form(&arg);
        println!("=> {nnf}");
    }
}

