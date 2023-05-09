#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub filename: String,
    pub start: usize,
    pub end: usize,
}