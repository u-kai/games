use std::fmt::Debug;

use crate::{
    masu::calcurator::IndexCalcurator,
    syogi::koma::{create_index, maybe_to_vec, SyogiKoma, RL},
};

#[derive(Clone, PartialEq, Eq)]
pub struct Kaku {
    r_l: RL,
    is_rev: bool,
}
impl Kaku {
    pub fn new(r_l: RL) -> Self {
        Kaku { r_l, is_rev: false }
    }
}

impl SyogiKoma for Kaku {
    fn movable_paths(&self, holizon: usize, valtical: usize) -> Vec<Vec<IndexCalcurator>> {
        let now_index = create_index(holizon, valtical);
        if self.is_rev {
            return vec![
                maybe_to_vec(now_index.get_up()),
                maybe_to_vec(now_index.get_down()),
                maybe_to_vec(now_index.get_right()),
                maybe_to_vec(now_index.get_left()),
                now_index.get_up_right_line(),
                now_index.get_up_left_line(),
                now_index.get_down_right_line(),
                now_index.get_down_left_line(),
            ]
            .iter()
            .filter(|vec| vec.len() > 0)
            .cloned()
            .collect();
        }
        vec![
            now_index.get_up_right_line(),
            now_index.get_up_left_line(),
            now_index.get_down_right_line(),
            now_index.get_down_left_line(),
        ]
        .iter()
        .filter(|vec| vec.len() > 0)
        .cloned()
        .collect()
    }
    fn rev(&mut self) -> () {
        self.is_rev = true
    }
}

impl Debug for Kaku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_rev {
            write!(f, "角")
        } else {
            write!(f, "龍馬")
        }
    }
}

#[cfg(test)]
mod kaku_test {
    use crate::{
        masu::calcurator::IndexCalcurator,
        syogi::{
            koma::{create_index, SyogiKoma, RL},
            komas::kaku::Kaku,
        },
    };
    #[test]
    fn movavle_paths_case_rev_test() {
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | |角| | | | | | | |
        // | | | | | | | | | |
        let mut kaku = Kaku::new(RL::Left);
        kaku.rev();
        assert_eq!(
            kaku.movable_paths(1, 7),
            vec![
                vec![create_index(1, 6)],
                vec![create_index(1, 8)],
                vec![create_index(2, 7)],
                vec![create_index(0, 7)],
                vec![
                    create_index(2, 6),
                    create_index(3, 5),
                    create_index(4, 4),
                    create_index(5, 3),
                    create_index(6, 2),
                    create_index(7, 1),
                    create_index(8, 0)
                ],
                vec![create_index(0, 6)],
                vec![create_index(2, 8)],
                vec![create_index(0, 8)]
            ]
        )
    }

    #[test]
    fn movable_paths_case_not_rev_test() {
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | |角| | | | | | | |
        // | | | | | | | | | |
        let kaku = Kaku::new(RL::Left);
        fn create(h: usize, v: usize) -> IndexCalcurator {
            IndexCalcurator::new(9, 9, h, v)
        }
        assert_eq!(
            kaku.movable_paths(1, 7),
            vec![
                vec![
                    create_index(2, 6),
                    create_index(3, 5),
                    create_index(4, 4),
                    create_index(5, 3),
                    create_index(6, 2),
                    create_index(7, 1),
                    create_index(8, 0)
                ],
                vec![create_index(0, 6)],
                vec![create_index(2, 8)],
                vec![create_index(0, 8)]
            ]
        )
    }
}
