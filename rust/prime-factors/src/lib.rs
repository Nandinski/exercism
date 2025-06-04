pub fn factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut iter = two_and_odd_iterator();
    while n > 1 {
        let limit_check = (n as f64).sqrt() as u64;
        let factor = iter
            .by_ref()
            .take_while(|&f| f <= limit_check)
            .find(|&x| n % x == 0)
            .unwrap_or(n);
        while n % factor == 0 {
            n /= factor;
            factors.push(factor);
        }
    }

    factors
}

fn two_and_odd_iterator() -> impl Iterator<Item = u64> {
    std::iter::once(2).chain((3..).step_by(2))
}
