pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }

    let mut primes = Vec::with_capacity(n as usize);
    // primes does not consider the prime number 2, even numbers will not be considered
    for candidate_n in (3..).step_by(2) {
        if is_prime(candidate_n, &primes) {
            primes.push(candidate_n);

            if primes.len() == n as usize {
                return candidate_n;
            }
        }
    }
    unreachable!() // for rust compiler -- 'for loop might have 0 iterations'
}

fn is_prime(candidate_n: u32, primes: &[u32]) -> bool {
    let limit_check = (candidate_n as f64).sqrt() as u32;
    primes
        .iter()
        .take_while(|&p| *p <= limit_check)
        .all(|p| candidate_n % p != 0)
}
