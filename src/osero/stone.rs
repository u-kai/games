use std::fmt::Debug;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum OseroStone {
    White,
    Black,
    Empty,
}
impl Debug for OseroStone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let d = self;
        match d {
            OseroStone::Black => {
                write!(f, "○")
            }
            OseroStone::Empty => {
                write!(f, " ")
            }
            OseroStone::White => {
                write!(f, "●")
            }
        }
    }
}
impl Default for OseroStone {
    fn default() -> Self {
        OseroStone::Empty
    }
}
