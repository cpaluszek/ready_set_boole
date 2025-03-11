use ready_set_boole::arithmetic::gray_code;

pub fn main() {
    for arg in std::env::args().skip(1) {
        let val: u32 = arg.parse().expect("Invalid number argument");
        let gray = gray_code(val);
        println!(
            "{:<10} = {:08b}  ->  {:<10} = {:08b}",
            val, val, gray, gray
        );
    }
}

