use crate::{
    masu::calcurator::IndexCalcurator,
    syogi::koma::{create_index, maybe_to_vec, SyogiKoma, RL},
};

use super::kin::kin_move;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Keima {
    r_l: RL,
    is_rev: bool,
}

impl Keima {
    pub fn new(r_l: RL) -> Self {
        Keima { r_l, is_rev: false }
    }
}

impl SyogiKoma for Keima {
    fn movable_paths(&self, holizon: usize, valtical: usize) -> Vec<Vec<IndexCalcurator>> {
        let now_index = create_index(holizon, valtical);
        match self.r_l {
            RL::Right => {
                if self.is_rev {
                    return kin_move(self.r_l.clone(), holizon, valtical);
                }
                vec![
                    maybe_to_vec(now_index.get_up_right()),
                    maybe_to_vec(now_index.get_up_left()),
                ]
            }
            _ => {
                if self.is_rev {
                    return kin_move(self.r_l.clone(), holizon, valtical);
                }
                vec![
                    maybe_to_vec(now_index.get_down_right()),
                    maybe_to_vec(now_index.get_down_left()),
                ]
            }
        }
    }
    fn rev(&mut self) -> () {
        self.is_rev = true
    }
}

#[cfg(test)]

mod keima_tests {
    use crate::syogi::koma::{create_index, SyogiKoma, RL};

    use super::Keima;
    #[test]

    fn movable_paths_case_R_rev_test() {
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | |桂馬| | | | | | | |<-(1,7)
        // | | | | | | | | | |
        let mut keima = Keima::new(RL::Right);
        keima.rev();
        assert_eq!(
            keima.movable_paths(1, 7),
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

    fn movable_paths_case_L_rev_test() {
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | |桂馬| | | | | | | |<-(1,7)
        // | | | | | | | | | |
        let mut keima = Keima::new(RL::Left);
        keima.rev();
        assert_eq!(
            keima.movable_paths(1, 7),
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

    fn movable_paths_case_L_test() {
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | |桂馬| | | | | | | |<-(1,7)
        // | | | | | | | | | |
        let keima = Keima::new(RL::Left);
        assert_eq!(
            keima.movable_paths(1, 7),
            vec![
                //down-right
                vec![create_index(2, 8)],
                //down-left
                vec![create_index(0, 8)]
            ]
        )
    }

    #[test]

    fn movable_paths_case_R_test() {
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | |桂馬| | | | | | | |<-(1,7)
        // | | | | | | | | | |
        let keima = Keima::new(RL::Right);
        assert_eq!(
            keima.movable_paths(1, 7),
            vec![
                //up-right
                vec![create_index(2, 6)],
                //up-left
                vec![create_index(0, 6)],
            ]
        )
    }
}
