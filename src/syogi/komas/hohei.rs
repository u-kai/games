use crate::{
    masu::calcurator::IndexCalcurator,
    syogi::koma::{create_index, maybe_to_vec, SyogiKoma, RL},
};

use super::kin::kin_move;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hohei {
    is_rev: bool,
    r_l: RL,
}

impl Hohei {
    pub fn new(r_l: RL) -> Self {
        Hohei { r_l, is_rev: false }
    }
}

impl SyogiKoma for Hohei {
    fn movable_paths(&self, holizon: usize, valtical: usize) -> Vec<Vec<IndexCalcurator>> {
        let now_index = create_index(holizon, valtical);
        if self.is_rev {
            kin_move(self.r_l.clone(), holizon, valtical)
        } else {
            match self.r_l {
                RL::Right => {
                    vec![maybe_to_vec(now_index.get_up())]
                }
                RL::Left => vec![maybe_to_vec(now_index.get_down())],
            }
        }
    }
    fn rev(&mut self) -> () {
        self.is_rev = true
    }
}
#[cfg(test)]
mod ho_test {
    use crate::syogi::koma::{create_index, SyogiKoma, RL};

    use super::Hohei;
    #[test]
    fn case_R_case_rev() {
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | |歩兵| | | | | | | |<-(1,7)
        // | | | | | | | | | |
        let mut ho = Hohei::new(RL::Right);
        ho.rev();
        assert_eq!(
            ho.movable_paths(1, 7),
            vec![
                //up
                vec![create_index(1, 6)],
                //down
                vec![create_index(1, 8)],
                //right
                vec![create_index(2, 7)],
                //left
                vec![create_index(0, 7)],
                //up-right
                vec![create_index(2, 6),],
                //up-left
                vec![create_index(0, 6)],
            ]
        )
    }
    #[test]
    fn case_L_rev() {
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | |歩兵| | | | | | | |<-(1,7)
        // | | | | | | | | | |
        let mut ho = Hohei::new(RL::Left);
        ho.rev();
        assert_eq!(
            ho.movable_paths(1, 7),
            vec![
                //up
                vec![create_index(1, 6)],
                //down
                vec![create_index(1, 8)],
                //right
                vec![create_index(2, 7)],
                //left
                vec![create_index(0, 7)],
                //down-right
                vec![create_index(2, 8)],
                //down-left
                vec![create_index(0, 8)]
            ]
        )
    }

    #[test]
    fn case_R() {
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | |歩兵| | | | | | | |<-(1,6)
        // | | | | | | | | | |
        // | | | | | | | | | |
        let ho = Hohei::new(RL::Right);
        assert_eq!(
            ho.movable_paths(1, 6),
            vec![vec![
                //up
                create_index(1, 5)
            ]]
        )
    }
    #[test]
    fn case_L() {
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | |歩兵| | | | | | | |<-(1,6)
        // | | | | | | | | | |
        // | | | | | | | | | |
        let ho = Hohei::new(RL::Left);
        assert_eq!(
            ho.movable_paths(1, 6),
            vec![vec![
                //down
                create_index(1, 7)
            ]]
        )
    }
}
