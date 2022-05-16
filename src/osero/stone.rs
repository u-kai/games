use std::fmt::Debug;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum OseroStone {
    White,
    Black,
    Empty,
}
impl OseroStone {
    pub fn color(&self) -> &str {
        match self {
            OseroStone::White => "WHITE",
            OseroStone::Black => "BLACK",
            OseroStone::Empty => "EMPTY",
        }
    }
    pub fn change(&mut self, stone: OseroStone) {
        *self = stone
    }
}
impl Debug for OseroStone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let d = self;
        match d {
            OseroStone::Black => {
                write!(f, "●")
            }
            OseroStone::Empty => {
                write!(f, " ")
            }
            OseroStone::White => {
                write!(f, "○")
            }
        }
    }
}
impl Default for OseroStone {
    fn default() -> Self {
        OseroStone::Empty
    }
}
