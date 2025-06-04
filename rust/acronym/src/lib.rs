pub fn abbreviate(phrase: &str) -> String {
    let mut result = String::new();

    for word in phrase.split(|c: char| c.is_whitespace() || c == '-') {
        let mut word_iter = word.chars();
        if let Some(first_c) = word_iter.find(|c| c.is_ascii_alphabetic()) {
            result.push(first_c.to_ascii_uppercase());
        }

        // Handle camel case
        let mut prev = 'A'; // simulate prev - first char is always uppercase
        for c in word_iter {
            if prev.is_lowercase() && c.is_uppercase() {
                result.push(c);
            }
            prev = c;
        }
    }

    result
}
