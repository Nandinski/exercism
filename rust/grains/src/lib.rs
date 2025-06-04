pub fn square(s: u32) -> u64 {
    assert!((1..=64).contains(&s), "square must be between 1 and 64");
    2u64.pow(s - 1)
}

pub fn total() -> u64 {
    // All 1's in u64
    // u64::MAX
    (1..=64).map(square).sum()
}
