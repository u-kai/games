use std::fmt::Debug;

use crate::koma::{create_index, maybe_to_vec, SyogiKoma, RL};
use masu::calcurator::IndexCalcurator;

#[derive(Clone, PartialEq, Eq)]
pub struct Hisya {
    r_l: RL,
    is_rev: bool,
}

impl Hisya {
    fn new(r_l: RL) -> Self {
        Hisya { r_l, is_rev: false }
    }
}
impl SyogiKoma for Hisya {
    fn movable_paths(&self, holizon: usize, valtical: usize) -> Vec<Vec<IndexCalcurator>> {
        let now_index = create_index(holizon, valtical);
        if self.is_rev {
            return vec![
                maybe_to_vec(now_index.get_up_right()),
                maybe_to_vec(now_index.get_up_left()),
                maybe_to_vec(now_index.get_down_right()),
                maybe_to_vec(now_index.get_down_left()),
                now_index.get_up_line(),
                now_index.get_down_line(),
                now_index.get_right_line(),
                now_index.get_left_line(),
            ];
        }
        vec![
            now_index.get_up_line(),
            now_index.get_down_line(),
            now_index.get_right_line(),
            now_index.get_left_line(),
        ]
    }
    fn rev(&mut self) -> () {
        self.is_rev = true
    }
}
impl Debug for Hisya {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_rev {
            write!(f, "龍玉")
        } else {
            write!(f, "飛車")
        }
    }
}
#[cfg(test)]

mod hisya_test {
    use crate::koma::{create_index, SyogiKoma, RL};

    use super::Hisya;
    #[test]
    fn movale_paths_case_rev_test() {
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | |飛車| |<- (7,7)
        // | | | | | | | | | |
        let mut hisya = Hisya::new(RL::Left);
        hisya.rev();
        assert_eq!(
            hisya.movable_paths(7, 7),
            vec![
                //up-right
                vec![create_index(8, 6)],
                //up-left
                vec![create_index(6, 6)],
                //down-right
                vec![create_index(8, 8)],
                //down-left
                vec![create_index(6, 8)],
                // up line
                vec![
                    create_index(7, 6),
                    create_index(7, 5),
                    create_index(7, 4),
                    create_index(7, 3),
                    create_index(7, 2),
                    create_index(7, 1),
                    create_index(7, 0),
                ],
                //down line
                vec![create_index(7, 8)],
                //right line
                vec![create_index(8, 7)],
                //left line
                vec![
                    create_index(6, 7),
                    create_index(5, 7),
                    create_index(4, 7),
                    create_index(3, 7),
                    create_index(2, 7),
                    create_index(1, 7),
                    create_index(0, 7),
                ]
            ]
        )
    }

    #[test]
    fn movale_paths_case_not_rev_test() {
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | |飛車| |<- (7,7)
        // | | | | | | | | | |
        let hisya = Hisya::new(RL::Left);
        assert_eq!(
            hisya.movable_paths(7, 7),
            vec![
                // up line
                vec![
                    create_index(7, 6),
                    create_index(7, 5),
                    create_index(7, 4),
                    create_index(7, 3),
                    create_index(7, 2),
                    create_index(7, 1),
                    create_index(7, 0),
                ],
                //down line
                vec![create_index(7, 8)],
                //right line
                vec![create_index(8, 7)],
                //left line
                vec![
                    create_index(6, 7),
                    create_index(5, 7),
                    create_index(4, 7),
                    create_index(3, 7),
                    create_index(2, 7),
                    create_index(1, 7),
                    create_index(0, 7),
                ]
            ]
        )
    }
}
