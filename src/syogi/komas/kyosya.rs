use crate::{
    masu::calcurator::IndexCalcurator,
    syogi::koma::{create_index, maybe_to_vec, SyogiKoma, RL},
};

use super::kin::{kin_move, Kin};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Kyosya {
    r_l: RL,
    is_rev: bool,
}

impl Kyosya {
    pub fn new(r_l: RL) -> Self {
        Kyosya { r_l, is_rev: false }
    }
}

impl SyogiKoma for Kyosya {
    fn movable_paths(&self, holizon: usize, valtical: usize) -> Vec<Vec<IndexCalcurator>> {
        let now_index = create_index(holizon, valtical);
        match self.r_l {
            RL::Right => {
                if self.is_rev {
                    return kin_move(self.r_l.clone(), holizon, valtical);
                }
                vec![now_index.get_up_line()]
            }
            _ => {
                if self.is_rev {
                    return kin_move(self.r_l.clone(), holizon, valtical);
                }
                vec![now_index.get_down_line()]
            }
        }
    }
    fn rev(&mut self) -> () {
        self.is_rev = true
    }
}

#[cfg(test)]

mod kyosya_tests {
    use crate::syogi::koma::{create_index, SyogiKoma, RL};

    use super::Kyosya;
    #[test]

    fn movable_paths_case_R_rev_test() {
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | |香車| | | | | | | |<-(1,7)
        // | | | | | | | | | |
        let mut kyosya = Kyosya::new(RL::Right);
        kyosya.rev();
        assert_eq!(
            kyosya.movable_paths(1, 7),
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
        // | |香車| | | | | | | |<-(1,7)
        // | | | | | | | | | |
        let mut kyosya = Kyosya::new(RL::Left);
        kyosya.rev();
        assert_eq!(
            kyosya.movable_paths(1, 7),
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
        // | |香車| | | | | | | |<-(1,7)
        // | | | | | | | | | |
        let kyosya = Kyosya::new(RL::Left);
        assert_eq!(
            kyosya.movable_paths(1, 7),
            vec![
                //down-line
                vec![create_index(1, 8)],
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
        // | |香車| | | | | | | |<-(1,7)
        // | | | | | | | | | |
        let kyosya = Kyosya::new(RL::Right);
        assert_eq!(
            kyosya.movable_paths(1, 7),
            vec![
                //up-line
                vec![
                    create_index(1, 6),
                    create_index(1, 5),
                    create_index(1, 4),
                    create_index(1, 3),
                    create_index(1, 2),
                    create_index(1, 1),
                    create_index(1, 0)
                ],
            ]
        )
    }
}
