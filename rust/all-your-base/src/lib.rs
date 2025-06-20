#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

macro_rules! ensure {
    ($condition:expr, $error:expr) => {
        if !$condition {
            return Err($error);
        }
    };
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return the corresponding Error enum if the conversion is impossible.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    ensure!(from_base >= 2, Error::InvalidInputBase);
    ensure!(to_base >= 2, Error::InvalidOutputBase);

    let mut num_base_10 = number.iter().try_fold(0, |sum, &digit| {
        ensure!(digit < from_base, Error::InvalidDigit(digit));
        Ok(sum * from_base + digit)
    })?;

    if num_base_10 == 0 {
        return Ok(vec![0]);
    }

    let mut result = Vec::new();
    while num_base_10 > 0 {
        result.push(num_base_10 % to_base);
        num_base_10 /= to_base;
    }
    result.reverse();
    Ok(result)
}
