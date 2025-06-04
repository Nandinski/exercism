pub fn find<S: AsRef<[T]>, T: Ord>(space: S, key: T) -> Option<usize> {
    let space = space.as_ref();
    let mut start = 0_usize;
    let mut end = space.len();
    while start < end {
        let middle = (start + end) / 2;
        match space[middle].cmp(&key) {
            std::cmp::Ordering::Less => start = middle + 1,
            std::cmp::Ordering::Equal => return Some(middle),
            std::cmp::Ordering::Greater => end = middle,
        }
    }
    None
}
