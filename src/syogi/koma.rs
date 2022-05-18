use std::fmt::Debug;

use super::komas::{
    gin::Gin, hisya::Hisya, hohei::Hohei, kaku::Kaku, kasya::Kasya, keima::Keima, kin::Kin,
    ohsyo::Ohsyo,
};
#[derive(Clone, PartialEq, Eq)]
pub enum RL {
    Right,
    Left,
}
pub trait SyogiKoma {
    fn is_movable(&self, holizon: usize, valtical: usize) -> bool;
    fn reverce(&mut self) -> ();
}

#[derive(Clone, PartialEq, Eq)]
pub enum Koma {
    Ohsyo(Ohsyo),
    Hisya(Hisya),
    Kaku(Kaku),
    Kin(Kin),
    Gin(Gin),
    Keima(Keima),
    Kasya(Kasya),
    Hohei(Hohei),
    Empty,
}
impl Debug for Koma {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Koma::Ohsyo(_) => {
                write!(f, "王")
            }
            Koma::Hisya(_) => {
                write!(f, "飛車")
            }
            Koma::Kaku(_) => {
                write!(f, "角")
            }
            Koma::Kin(_) => {
                write!(f, "金")
            }
            Koma::Gin(_) => {
                write!(f, "銀")
            }
            Koma::Keima(_) => {
                write!(f, "桂馬")
            }
            Koma::Kasya(_) => {
                write!(f, "香車")
            }
            Koma::Hohei(_) => {
                write!(f, "歩兵")
            }
            Koma::Empty => {
                write!(f, r#"　　"#)
            }
        }
    }
}
