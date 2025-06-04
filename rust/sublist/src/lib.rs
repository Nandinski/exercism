#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    fn is_superlist<T: PartialEq>(a: &[T], b: &[T]) -> bool {
        b.is_empty() || a.windows(b.len()).any(|slice| slice == b)
    }

    if a.len() == b.len() && a == b {
        Comparison::Equal
    } else if a.len() > b.len() && is_superlist(a, b) {
        Comparison::Superlist
    } else if a.len() < b.len() && is_superlist(b, a) {
        Comparison::Sublist
    } else {
        Comparison::Unequal
    }
}
