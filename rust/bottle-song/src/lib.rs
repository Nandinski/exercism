pub fn recite(start_bottles: u32, take_down: u32) -> String {
    assert!(start_bottles >= take_down);
    (0..take_down)
        .map(|take| verse(start_bottles - take))
        .collect::<Vec<_>>()
        .join("\n")
}

fn verse(bottle_count: u32) -> String {
    let b_word = title_case(number_to_word(bottle_count));
    let b_noun = bottle_or_bottles(bottle_count);
    let post_b_word = number_to_word(bottle_count - 1);
    let post_b_noun = bottle_or_bottles(bottle_count - 1);
    format!(
        "{b_word} green {b_noun} hanging on the wall,\n\
         {b_word} green {b_noun} hanging on the wall,\n\
         And if one green bottle should accidentally fall,\n\
         There'll be {post_b_word} green {post_b_noun} hanging on the wall.\n"
    )
}

// From: https://stackoverflow.com/a/38406885
fn title_case(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(f) => f.to_uppercase().to_string() + c.as_str(),
        None => String::new(),
    }
}

fn bottle_or_bottles(count: u32) -> &'static str {
    match count {
        1 => "bottle",
        _ => "bottles",
    }
}

fn number_to_word(n: u32) -> &'static str {
    const NUMBERS: [&str; 11] = [
        "no", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
    ];
    NUMBERS
        .get(n as usize)
        .expect("?? We don't know how to count past ten!")
}
