use crate::{
    masu::calcurator::IndexCalcurator,
    syogi::koma::{create_index, maybe_to_vec, SyogiKoma, RL},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gin {
    r_l: RL,
    is_rev: bool,
}
impl Gin {
    pub fn new(r_l: RL) -> Self {
        Gin { r_l, is_rev: false }
    }
}

impl SyogiKoma for Gin {
    fn movable_paths(&self, holizon: usize, valtical: usize) -> Vec<Vec<IndexCalcurator>> {
        let now_index = create_index(holizon, valtical);
        match self.r_l {
            RL::Right => {
                if self.is_rev {
                    return vec![
                        maybe_to_vec(now_index.get_up()),
                        maybe_to_vec(now_index.get_down()),
                        maybe_to_vec(now_index.get_right()),
                        maybe_to_vec(now_index.get_left()),
                        maybe_to_vec(now_index.get_up_right()),
                        maybe_to_vec(now_index.get_up_left()),
                    ];
                }
                vec![
                    maybe_to_vec(now_index.get_up()),
                    maybe_to_vec(now_index.get_up_right()),
                    maybe_to_vec(now_index.get_up_left()),
                    maybe_to_vec(now_index.get_down_right()),
                    maybe_to_vec(now_index.get_down_left()),
                ]
            }
            _ => {
                if self.is_rev {
                    return vec![
                        maybe_to_vec(now_index.get_up()),
                        maybe_to_vec(now_index.get_down()),
                        maybe_to_vec(now_index.get_right()),
                        maybe_to_vec(now_index.get_left()),
                        maybe_to_vec(now_index.get_down_right()),
                        maybe_to_vec(now_index.get_down_left()),
                    ];
                }
                vec![
                    maybe_to_vec(now_index.get_down()),
                    maybe_to_vec(now_index.get_up_right()),
                    maybe_to_vec(now_index.get_up_left()),
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
mod kin_tests {
    use crate::syogi::koma::{create_index, SyogiKoma, RL};

    use super::Gin;
    #[test]

    fn movable_paths_case_rev_L_test() {
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | |銀| | | | | | | |<-(1,7)
        // | | | | | | | | | |
        let mut gin = Gin::new(RL::Left);
        gin.rev();
        assert_eq!(
            gin.movable_paths(1, 7),
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

    fn movable_paths_test_case_L() {
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | |銀| | | | | | | |<-(1,7)
        // | | | | | | | | | |
        let gin = Gin::new(RL::Left);
        assert_eq!(
            gin.movable_paths(1, 7),
            vec![
                //down
                vec![create_index(1, 8),],
                //up-right
                vec![create_index(2, 6),],
                //up-left
                vec![create_index(0, 6)],
                //down-right
                vec![create_index(2, 8)],
                //down-left
                vec![create_index(0, 8)]
            ]
        )
    }
    #[test]

    fn movable_paths_case_rev_R_test() {
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | |銀| | | | | | | |<-(1,7)
        // | | | | | | | | | |
        let mut gin = Gin::new(RL::Right);
        gin.rev();
        assert_eq!(
            gin.movable_paths(1, 7),
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

    fn movable_paths_test_case_R() {
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | |銀| | | | | | | |<-(1,7)
        // | | | | | | | | | |
        let gin = Gin::new(RL::Right);
        assert_eq!(
            gin.movable_paths(1, 7),
            vec![
                //up
                vec![create_index(1, 6)],
                //up-right
                vec![create_index(2, 6),],
                //up-left
                vec![create_index(0, 6)],
                //down-right
                vec![create_index(2, 8)],
                //down-left
                vec![create_index(0, 8)]
            ]
        )
    }
}
