use std::fmt::Debug;

use masu::calcurator::IndexCalcurator;

use crate::koma::{create_index, maybe_to_vec, SyogiKoma, RL};

#[derive(Clone, PartialEq, Eq)]
pub struct Ohsyo {
    r_l: RL,
}
impl Ohsyo {
    pub fn new(r_l: RL) -> Self {
        Ohsyo { r_l }
    }
    pub fn is_gyoku(&self) -> bool {
        match self.r_l {
            RL::Right => true,
            _ => false,
        }
    }
}

impl SyogiKoma for Ohsyo {
    fn movable_paths(&self, holizon: usize, valtical: usize) -> Vec<Vec<IndexCalcurator>> {
        let now_index = create_index(holizon, valtical);
        [
            now_index.get_up(),
            now_index.get_down(),
            now_index.get_right(),
            now_index.get_left(),
            now_index.get_up_right(),
            now_index.get_up_left(),
            now_index.get_down_right(),
            now_index.get_down_left(),
        ]
        .iter()
        .cloned()
        .map(|maybe| maybe_to_vec(maybe))
        .filter(|vec| vec.len() > 0)
        .collect()
    }
    fn rev(&mut self) -> () {
        ()
    }
}

impl Debug for Ohsyo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_gyoku() {
            write!(f, "玉")
        } else {
            write!(f, "王")
        }
    }
}
#[cfg(test)]
mod ohsyo_tests {
    use crate::{
        koma::{create_index, SyogiKoma, RL},
        komas::ohsyo::Ohsyo,
    };

    #[test]
    fn movable_paths_test() {
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | |王| | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        let ohsyo = Ohsyo::new(RL::Left);
        //now index
        assert_eq!(
            ohsyo.movable_paths(4, 5),
            vec![
                vec![create_index(4, 4)],
                vec![create_index(4, 6)],
                vec![create_index(5, 5)],
                vec![create_index(3, 5)],
                vec![create_index(5, 4)],
                vec![create_index(3, 4)],
                vec![create_index(5, 6)],
                vec![create_index(3, 6)],
            ]
        );
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // 王| | | | | | | | | |
        let ohsyo = Ohsyo::new(RL::Left);
        //now index
        assert_eq!(
            ohsyo.movable_paths(0, 8),
            vec![
                vec![create_index(0, 7)],
                vec![create_index(1, 8)],
                vec![create_index(1, 7)],
            ]
        )
    }
}
