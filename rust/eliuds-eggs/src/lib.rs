pub fn egg_count(display_value: u32) -> usize {
    (0..32)
        .filter(|shift| (display_value & (1 << shift)) != 0)
        .count()
}
