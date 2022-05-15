use std::fmt::Debug;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CorX {
    Circle,
    Xross,
    Empty,
}

impl Debug for CorX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CorX::Circle => write!(f, "○"),

            CorX::Xross => write!(f, "✖️"),
            CorX::Empty => write!(f, " "),
        }
    }
}
impl Default for CorX {
    fn default() -> Self {
        CorX::Empty
    }
}
