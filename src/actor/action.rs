#[derive(Clone)]
pub enum Action {
    Call,
    Raise(usize),
    Fold
}