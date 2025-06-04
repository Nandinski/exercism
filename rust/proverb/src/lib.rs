use std::fmt::Write;

pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = String::new();

    for w in list.windows(2) {
        writeln!(proverb, "For want of a {} the {} was lost.", w[0], w[1]).unwrap();
    }

    if !list.is_empty() {
        write!(proverb, "And all for the want of a {}.", list[0]).unwrap();
    }

    proverb
}
