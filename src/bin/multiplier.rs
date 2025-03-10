use ready_set_boole::multiplier;

pub fn main() {
    let mut product: u32 = 1;
    for arg in std::env::args().skip(1) {
        let val: u32 = arg.parse().expect("Invalid number argument");
        product = multiplier(product, val);
    }
    println!("Product: {product}");
}

