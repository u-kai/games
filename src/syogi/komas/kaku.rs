use crate::{
    masu::calcurator::IndexCalcurator,
    syogi::koma::{SyogiKoma, RL},
};

#[derive(Clone, PartialEq, Eq)]
pub struct Kaku {
    r_l: RL,
    is_rev: bool,
    h_index: usize,
    v_index: usize,
}
impl Kaku {
    pub fn new(r_l: RL) -> Self {
        match r_l {
            RL::Right => Kaku {
                r_l,
                is_rev: false,
                h_index: 1,
                v_index: 7,
            },
            RL::Left => Kaku {
                r_l,
                is_rev: false,
                h_index: 1,
                v_index: 7,
            },
        }
    }
    fn movable_paths(&self) -> Vec<IndexCalcurator> {
        let now_index = self.create_now_index();
        vec![
            now_index.get_up_right_line(),
            now_index.get_up_left_line(),
            now_index.get_down_right_line(),
            now_index.get_down_left_line(),
        ]
        .concat()
    }
    fn create_now_index(&self) -> IndexCalcurator {
        IndexCalcurator::new(9, 9, self.h_index, self.v_index)
    }
}

impl SyogiKoma for Kaku {
    fn is_movable(&self, holizon: usize, valtical: usize) -> bool {
        self.movable_paths()
            .iter()
            .map(|index| (index.get_h(), index.get_v()))
            .find(|(h, v)| h == &holizon && v == &valtical)
            .is_some()
    }
    fn reverce(&mut self) -> () {
        self.is_rev = true
    }
}

#[test]
fn is_movable_test() {
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
    assert_eq!(kaku.is_movable(0, 8), true);
    assert_eq!(kaku.is_movable(2, 6), true);
    assert_eq!(kaku.is_movable(2, 8), true);
    assert_eq!(kaku.is_movable(2, 3), false);
    assert_eq!(kaku.is_movable(1, 8), false);
    assert_eq!(kaku.is_movable(1, 0), false);
}
