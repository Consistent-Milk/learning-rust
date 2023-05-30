pub fn square(s: u32) -> u64 {
    if s > 64 || s < 1 {
        panic!("Square must be between 1 and 64");
    }

    let grains: u64 = (2 as u64).pow(s - 1);

    return grains;
}

pub fn total() -> u64 {
    return 18_446_744_073_709_551_615;
}
