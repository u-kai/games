use std::fmt::Debug;

use crate::masu::calcurator::IndexCalcurator;

use super::komas::{
    gin::Gin, hisya::Hisya, hohei::Hohei, kaku::Kaku, kasya::Kasya, keima::Keima, kin::Kin,
    ohsyo::Ohsyo,
};
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RL {
    Right,
    Left,
}
pub trait SyogiKoma {
    fn movable_paths(&self, holizon: usize, valtical: usize) -> Vec<Vec<IndexCalcurator>>;
    fn rev(&mut self) -> ();
}

pub fn create_index(holizon: usize, valtical: usize) -> IndexCalcurator {
    IndexCalcurator::new(9, 9, holizon, valtical)
}
pub fn maybe_to_vec<T: Clone>(maybe: Result<T, String>) -> Vec<T> {
    [maybe]
        .iter()
        .filter_map(|t| t.as_ref().ok())
        .cloned()
        .collect()
}
#[derive(Clone, PartialEq, Eq)]
pub enum Koma {
    Ohsyo(Ohsyo),
    Hisya(Hisya),
    Kaku(Kaku),
    //Kin(Kin),
    //Gin(Gin),
    //Keima(Keima),
    //Kasya(Kasya),
    //Hohei(Hohei),
    Empty,
}
impl Koma {
    pub fn movable_paths(&self, holizon: usize, valtical: usize) -> Vec<Vec<IndexCalcurator>> {
        match self {
            Koma::Empty => {
                panic!("this is empty")
            }
            Koma::Ohsyo(ousyo) => ousyo.movable_paths(holizon, valtical),
            Koma::Kaku(kaku) => kaku.movable_paths(holizon, valtical),
            Koma::Hisya(hisya) => hisya.movable_paths(holizon, valtical),
        }
    }
}
impl Debug for Koma {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            //Koma::Ohsyo(ohsyo) => {
            //if ohsyo.is_gyoku() {
            //write!(f, "玉")
            //} else {
            //write!(f, "王")
            //}
            //}
            ////Koma::Hisya(_) => {
            ////write!(f, "飛車")
            ////}
            //Koma::Kaku(_) => {
            //write!(f, "角")
            //}
            ////Koma::Kin(_) => {
            ////write!(f, "金")
            ////}
            ////Koma::Gin(_) => {
            ////write!(f, "銀")
            ////}
            ////Koma::Keima(_) => {
            ////write!(f, "桂馬")
            ////}
            ////Koma::Kasya(_) => {
            ////write!(f, "香車")
            ////}
            ////Koma::Hohei(_) => {
            ////write!(f, "歩兵")
            ////}
            Koma::Empty => {
                write!(f, r#"　　"#)
            }
            _i => Ok(println!("{:?}", self)),
        }
    }
}
impl Default for Koma {
    fn default() -> Self {
        Koma::Empty
    }
}
