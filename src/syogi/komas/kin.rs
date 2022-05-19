use crate::{
    masu::calcurator::IndexCalcurator,
    syogi::koma::{create_index, maybe_to_vec, SyogiKoma, RL},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Kin {
    r_l: RL,
}
impl Kin {
    pub fn new(r_l: RL) -> Self {
        Kin { r_l }
    }
}

impl SyogiKoma for Kin {
    fn movable_paths(&self, holizon: usize, valtical: usize) -> Vec<Vec<IndexCalcurator>> {
        let now_index = create_index(holizon, valtical);
        match self.r_l {
            RL::Right => {
                vec![
                    maybe_to_vec(now_index.get_up()),
                    maybe_to_vec(now_index.get_down()),
                    maybe_to_vec(now_index.get_right()),
                    maybe_to_vec(now_index.get_left()),
                    maybe_to_vec(now_index.get_up_right()),
                    maybe_to_vec(now_index.get_up_left()),
                ]
            }
            _ => {
                vec![
                    maybe_to_vec(now_index.get_up()),
                    maybe_to_vec(now_index.get_down()),
                    maybe_to_vec(now_index.get_right()),
                    maybe_to_vec(now_index.get_left()),
                    maybe_to_vec(now_index.get_down_right()),
                    maybe_to_vec(now_index.get_down_left()),
                ]
            }
        }
    }
    fn rev(&mut self) -> () {
        ()
    }
}

#[cfg(test)]
mod kin_tests {
    use crate::syogi::koma::{create_index, SyogiKoma, RL};

    use super::Kin;
    #[test]

    fn movable_paths_test_case_L() {
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | |金| | | | | | | |<-(1,7)
        // | | | | | | | | | |
        let kin = Kin::new(RL::Left);
        assert_eq!(
            kin.movable_paths(1, 7),
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

    fn movable_paths_test_case_R() {
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | |金| | | | | | | |<-(1,7)
        // | | | | | | | | | |
        let kin = Kin::new(RL::Right);
        assert_eq!(
            kin.movable_paths(1, 7),
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
}
