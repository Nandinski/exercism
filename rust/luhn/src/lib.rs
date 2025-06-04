/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    code.chars()
        .rev()
        .filter(|&c| c != ' ')
        .try_fold((0, 0), |(sum, counter), c| {
            c.to_digit(10).map(|mut d| {
                if counter % 2 == 1 {
                    d *= 2;
                    if d > 9 {
                        d -= 9;
                    }
                }

                (sum + d, counter + 1)
            })
        })
        .map_or(false, |(sum, counter)| sum % 10 == 0 && counter > 1)
}
