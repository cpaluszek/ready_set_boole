use ready_set_boole::adder;

pub fn main() {
    let mut sum: u32 = 0;
    for arg in std::env::args().skip(1) {
        let val: u32 = arg.parse().expect("Invalid number argument");
        sum = adder(sum, val);
    }
    println!("Sum: {sum}");
}

