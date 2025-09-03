pub fn square(s: u32) -> u64 {
        1_u64 << (s - 1)
}

pub fn total() -> u128 {
        (1_u128 << 64) - 1
}
