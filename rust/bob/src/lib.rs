pub fn reply(message: &str) -> &str {
    let message = message.trim();
    let is_question = message.ends_with('?');
    let is_yelling =
        message.to_uppercase() == message && message.chars().any(|c| c.is_alphabetic());
    let is_silence = message.is_empty();

    if is_question && is_yelling {
        "Calm down, I know what I'm doing!"
    } else if is_question {
        "Sure."
    } else if is_yelling {
        "Whoa, chill out!"
    } else if is_silence {
        "Fine. Be that way!"
    } else {
        "Whatever."
    }
}
