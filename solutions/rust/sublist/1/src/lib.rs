#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(a: &[i32], b: &[i32]) -> Comparison {
        if a == b {
        Comparison::Equal
    } else if is_sublist(a, b) {
        Comparison::Sublist
    } else if is_sublist(b, a) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}
fn is_sublist<T: PartialEq>(small: &[T], big: &[T]) -> bool {
    if small.is_empty() {
        return true;
    }
    big.windows(small.len()).any(|window| window == small)
}