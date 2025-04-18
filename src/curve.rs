pub fn map(x: u16, y: u16) -> f64 {
    let mut packed: u64 = 0;
    let x = x as u64;
    let y = y as u64;
    for i in 0..16 {
        packed |= ((x >> i) & 1) << (2 * i);
        packed |= ((y >> i) & 1) << (2 * i + 1)
    }
    return (packed as f64) / (u32::MAX as f64);
}

pub fn reverse_map(n: f64) -> (u16, u16) {
    let packed = (n * (u32::MAX as f64)) as u32;

    let mut x: u16 = 0;
    let mut y: u16 = 0;
    for i in 0..16 {
        x |= ((packed >> (2 * i) & 1) as u16) << i;
        y |= ((packed >> (2 * i + 1) & 1) as u16) << i;
    }
    (x, y)
}


