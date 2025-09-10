#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Action {
    Call,
    Raise(usize),
    Fold
}