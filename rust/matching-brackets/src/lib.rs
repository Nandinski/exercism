pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();
    for b in string.chars() {
        match b {
            '(' => stack.push(')'),
            '{' => stack.push('}'),
            '[' => stack.push(']'),
            ')' | '}' | ']' => {
                if stack.pop() != Some(b) {
                    return false;
                }
            }
            _ => (),
        }
    }
    stack.is_empty()
}
