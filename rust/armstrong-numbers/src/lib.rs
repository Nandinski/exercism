pub fn is_armstrong_number(num: u32) -> bool {
    let digit_count = num.checked_ilog10().unwrap_or(0) + 1;

    std::iter::successors(Some((num, num % 10)), |(r, _)| {
        (*r != 0).then(|| {
            let r = r / 10;
            (r, r % 10)
        })
    })
    .try_fold(0u32, |sum, (_, digit)| {
        sum.checked_add(digit.pow(digit_count))
    })
    .is_some_and(|armstrong_candidate| armstrong_candidate == num)
}
