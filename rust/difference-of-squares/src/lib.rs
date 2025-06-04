pub fn square_of_sum(n: u32) -> u32 {
    let sum = match n {
        0 => 0,
        // This division by 2 works with u32 because
        // (1 + n) * n is always even! It's fun to prove it
        // If you need a tip, see if the output below is even when:
        //     n is even => then there must be an x such that n = 2x
        //     n is odd  => then there must be an x such that n = 2x + 1
        _ => ((1 + n) * n) / 2,
    };

    sum.pow(2)
}

// https://www.youtube.com/watch?v=aXbT37IlyZQ
pub fn sum_of_squares(n: u32) -> u32 {
    match n {
        0 => 0,
        _ => (n * (n + 1) * (2 * n + 1)) / 6,
    }
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
