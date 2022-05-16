use crate::masu::masu::MasuBoard;

use super::stone::OseroStone;
pub struct OseroBoard {
    masu: MasuBoard<OseroStone>,
}

const H_LEN: usize = 8;
const V_LEN: usize = 8;

impl OseroBoard {
    pub fn new_game() -> OseroBoard {
        let mut masu = MasuBoard::new(H_LEN, V_LEN);
        masu.change(3, 3, OseroStone::White);
        masu.change(3, 4, OseroStone::Black);
        masu.change(4, 3, OseroStone::Black);
        masu.change(4, 4, OseroStone::White);
        OseroBoard { masu }
    }
    pub fn masu(&self, holizon: usize, valtical: usize) -> OseroStone {
        self.masu.get(holizon, valtical)
    }
    pub fn get_black_num(&self) -> usize {
        self.get_stone_num(OseroStone::Black)
    }
    pub fn get_white_num(&self) -> usize {
        self.get_stone_num(OseroStone::White)
    }
    fn get_stone_num(&self, color: OseroStone) -> usize {
        self.masu
            .all()
            .iter()
            .flatten()
            .filter(|stone| stone == &&color)
            .count()
    }
    pub fn is_fill(&self) -> bool {
        self.masu
            .all()
            .iter()
            .flatten()
            .all(|stone| stone != &OseroStone::Empty)
    }
    pub fn is_pass(&self, stone: OseroStone) -> bool {
        for i in 0..8 {
            for j in 0..8 {
                if self.is_puttable(i, j, stone) {
                    return false;
                }
            }
        }
        true
    }
    pub fn is_puttable(&self, holizon: usize, valtical: usize, stone: OseroStone) -> bool {
        match self.masu(holizon, valtical) {
            OseroStone::Empty => {
                self.is_up_puttable(holizon, valtical, stone)
                    || self.is_down_puttable(holizon, valtical, stone)
                    || self.is_right_puttable(holizon, valtical, stone)
                    || self.is_left_puttable(holizon, valtical, stone)
                    || self.is_up_right_puttable(holizon, valtical, stone)
                    || self.is_down_right_puttable(holizon, valtical, stone)
                    || self.is_up_left_puttable(holizon, valtical, stone)
                    || self.is_down_left_puttable(holizon, valtical, stone)
            }
            _ => false,
        }
    }
    pub fn put(
        &mut self,
        holizon: usize,
        valtical: usize,
        stone: OseroStone,
    ) -> Result<(), String> {
        if self.is_puttable(holizon, valtical, stone) {
            self.masu.change(holizon, valtical, stone);
            self.change_up(holizon, valtical, stone);
            self.change_down(holizon, valtical, stone);
            self.change_right(holizon, valtical, stone);
            self.change_left(holizon, valtical, stone);
            self.change_up_right(holizon, valtical, stone);
            self.change_down_right(holizon, valtical, stone);
            self.change_up_left(holizon, valtical, stone);
            self.change_down_left(holizon, valtical, stone);
            Ok(())
        } else {
            Err(format!("can not put [{},{}]", holizon, valtical))
        }
    }
    pub fn print(&self) {
        self.masu.print()
    }
    fn is_up_puttable(&self, holizon: usize, valtical: usize, stone: OseroStone) -> bool {
        if let Some(v_index) = self.up_next_v_index(holizon, valtical, stone) {
            // 直近は除く
            !((valtical - 1) == v_index)
        } else {
            false
        }
    }
    fn is_down_puttable(&self, holizon: usize, valtical: usize, stone: OseroStone) -> bool {
        if let Some(v_index) = self.down_next_v_index(holizon, valtical, stone) {
            !((valtical + 1) == v_index)
        } else {
            false
        }
    }
    fn is_right_puttable(&self, holizon: usize, valtical: usize, stone: OseroStone) -> bool {
        if let Some(h_index) = self.right_next_h_index(holizon, valtical, stone) {
            !((holizon + 1) == h_index)
        } else {
            false
        }
    }
    fn is_left_puttable(&self, holizon: usize, valtical: usize, stone: OseroStone) -> bool {
        if let Some(h_index) = self.left_next_h_index(holizon, valtical, stone) {
            !((holizon - 1) == h_index)
        } else {
            false
        }
    }
    fn is_up_right_puttable(&self, holizon: usize, valtical: usize, stone: OseroStone) -> bool {
        if let Some((h_index, v_index)) = self.up_right_next_index(holizon, valtical, stone) {
            !((holizon + 1) == h_index) && !((valtical - 1) == v_index)
        } else {
            false
        }
    }
    fn is_down_right_puttable(&self, holizon: usize, valtical: usize, stone: OseroStone) -> bool {
        if let Some((h_index, v_index)) = self.down_right_next_index(holizon, valtical, stone) {
            !((holizon + 1) == h_index) && !((valtical + 1) == v_index)
        } else {
            false
        }
    }
    fn is_up_left_puttable(&self, holizon: usize, valtical: usize, stone: OseroStone) -> bool {
        if let Some((h_index, v_index)) = self.up_left_next_index(holizon, valtical, stone) {
            !((holizon - 1) == h_index) && !((valtical - 1) == v_index)
        } else {
            false
        }
    }
    fn is_down_left_puttable(&self, holizon: usize, valtical: usize, stone: OseroStone) -> bool {
        if let Some((h_index, v_index)) = self.down_left_next_index(holizon, valtical, stone) {
            !((holizon - 1) == h_index) && !((valtical + 1) == v_index)
        } else {
            false
        }
    }
    fn change_up(&mut self, holizon: usize, valtical: usize, stone: OseroStone) {
        if let Some(next_up_index) = self.up_next_v_index(holizon, valtical, stone) {
            for v in next_up_index..=valtical {
                self.masu.change(holizon, v, stone);
            }
        }
    }
    fn change_down(&mut self, holizon: usize, valtical: usize, stone: OseroStone) {
        if let Some(next_down_index) = self.down_next_v_index(holizon, valtical, stone) {
            for v in valtical..=next_down_index {
                self.masu.change(holizon, v, stone);
            }
        }
    }
    fn change_right(&mut self, holizon: usize, valtical: usize, stone: OseroStone) {
        if let Some(next_right_index) = self.right_next_h_index(holizon, valtical, stone) {
            for h in holizon..=next_right_index {
                self.masu.change(h, valtical, stone);
            }
        }
    }
    fn change_left(&mut self, holizon: usize, valtical: usize, stone: OseroStone) {
        if let Some(next_left_index) = self.left_next_h_index(holizon, valtical, stone) {
            for h in next_left_index..=holizon {
                self.masu.change(h, valtical, stone);
            }
        }
    }
    fn change_up_right(&mut self, holizon: usize, valtical: usize, stone: OseroStone) {
        if let Some((n_h, _)) = self.up_right_next_index(holizon, valtical, stone) {
            for (i, h) in (holizon..=n_h).enumerate() {
                self.masu.change(h, valtical - i, stone)
            }
        }
    }
    fn change_down_right(&mut self, holizon: usize, valtical: usize, stone: OseroStone) {
        if let Some((n_h, _)) = self.down_right_next_index(holizon, valtical, stone) {
            for (i, h) in (holizon..=n_h).enumerate() {
                self.masu.change(h, valtical + i, stone)
            }
        }
    }
    fn change_up_left(&mut self, holizon: usize, valtical: usize, stone: OseroStone) {
        if let Some((n_h, n_v)) = self.up_left_next_index(holizon, valtical, stone) {
            for (i, h) in (n_h..=holizon).enumerate() {
                self.masu.change(h, n_v + i, stone)
            }
        }
    }
    fn change_down_left(&mut self, holizon: usize, valtical: usize, stone: OseroStone) {
        if let Some((n_h, n_v)) = self.down_left_next_index(holizon, valtical, stone) {
            for (i, h) in (n_h..=holizon).enumerate() {
                self.masu.change(h, n_v - i, stone)
            }
        }
    }

    fn up_next_v_index(&self, holizon: usize, valtical: usize, stone: OseroStone) -> Option<usize> {
        match self.masu.get_up(holizon, valtical) {
            Ok(up_stone) => match up_stone {
                OseroStone::Empty => None,
                _ => {
                    if up_stone == stone {
                        Some(valtical - 1)
                    } else {
                        self.up_next_v_index(holizon, valtical - 1, stone)
                    }
                }
            },
            Err(_) => None,
        }
    }
    fn up_right_next_index(
        &self,
        holizon: usize,
        valtical: usize,
        stone: OseroStone,
    ) -> Option<(usize, usize)> {
        match self.masu.get_up_right(holizon, valtical) {
            Ok(up_stone) => match up_stone {
                OseroStone::Empty => None,
                _ => {
                    if up_stone == stone {
                        Some((holizon + 1, valtical - 1))
                    } else {
                        self.up_right_next_index(holizon + 1, valtical - 1, stone)
                    }
                }
            },
            Err(_) => None,
        }
    }
    fn up_left_next_index(
        &self,
        holizon: usize,
        valtical: usize,
        stone: OseroStone,
    ) -> Option<(usize, usize)> {
        match self.masu.get_up_left(holizon, valtical) {
            Ok(up_stone) => match up_stone {
                OseroStone::Empty => None,
                _ => {
                    if up_stone == stone {
                        Some((holizon - 1, valtical - 1))
                    } else {
                        self.up_left_next_index(holizon - 1, valtical - 1, stone)
                    }
                }
            },
            Err(_) => None,
        }
    }
    fn down_next_v_index(
        &self,
        holizon: usize,
        valtical: usize,
        stone: OseroStone,
    ) -> Option<usize> {
        match self.masu.get_down(holizon, valtical) {
            Ok(down_stone) => match down_stone {
                OseroStone::Empty => None,
                _ => {
                    if down_stone == stone {
                        Some(valtical + 1)
                    } else {
                        self.down_next_v_index(holizon, valtical + 1, stone)
                    }
                }
            },
            Err(_) => None,
        }
    }
    fn down_right_next_index(
        &self,
        holizon: usize,
        valtical: usize,
        stone: OseroStone,
    ) -> Option<(usize, usize)> {
        match self.masu.get_down_right(holizon, valtical) {
            Ok(down_stone) => match down_stone {
                OseroStone::Empty => None,
                _ => {
                    if down_stone == stone {
                        Some((holizon + 1, valtical + 1))
                    } else {
                        self.down_right_next_index(holizon + 1, valtical + 1, stone)
                    }
                }
            },
            Err(_) => None,
        }
    }
    fn down_left_next_index(
        &self,
        holizon: usize,
        valtical: usize,
        stone: OseroStone,
    ) -> Option<(usize, usize)> {
        match self.masu.get_down_left(holizon, valtical) {
            Ok(up_stone) => match up_stone {
                OseroStone::Empty => None,
                _ => {
                    if up_stone == stone {
                        Some((holizon - 1, valtical + 1))
                    } else {
                        self.down_left_next_index(holizon - 1, valtical + 1, stone)
                    }
                }
            },
            Err(_) => None,
        }
    }
    fn right_next_h_index(
        &self,
        holizon: usize,
        valtical: usize,
        stone: OseroStone,
    ) -> Option<usize> {
        match self.masu.get_right(holizon, valtical) {
            Ok(up_stone) => match up_stone {
                OseroStone::Empty => None,
                _ => {
                    if up_stone == stone {
                        Some(holizon + 1)
                    } else {
                        self.right_next_h_index(holizon + 1, valtical, stone)
                    }
                }
            },
            Err(_) => None,
        }
    }
    fn left_next_h_index(
        &self,
        holizon: usize,
        valtical: usize,
        stone: OseroStone,
    ) -> Option<usize> {
        match self.masu.get_left(holizon, valtical) {
            Ok(left_stone) => match left_stone {
                OseroStone::Empty => None,
                _ => {
                    if left_stone == stone {
                        Some(holizon - 1)
                    } else {
                        self.left_next_h_index(holizon - 1, valtical, stone)
                    }
                }
            },
            Err(_) => None,
        }
    }
}

#[cfg(test)]
mod osero_test {

    use crate::{masu::masu::MasuBoard, osero::stone::OseroStone};

    use super::OseroBoard;

    #[test]
    fn new_game_test() {
        let game = OseroBoard::new_game();
        assert_eq!(game.masu(3, 3), OseroStone::White);
        assert_eq!(game.masu(3, 4), OseroStone::Black);
        assert_eq!(game.masu(4, 3), OseroStone::Black);
        assert_eq!(game.masu(4, 4), OseroStone::White);
    }
    #[test]
    fn is_puttable_test() {
        let mut osero = OseroBoard::new_game();
        assert_eq!(osero.is_puttable(3, 3, OseroStone::Black), false);
        osero.print();
        assert_eq!(osero.is_puttable(2, 3, OseroStone::Black), true);
        osero.put(2, 3, OseroStone::Black).unwrap();
        osero.print();
        assert_eq!(osero.is_puttable(4, 2, OseroStone::Black), false);
        assert_eq!(osero.is_puttable(4, 2, OseroStone::White), true);
    }
    #[test]
    fn is_puttable_test_full_white() {
        // most quick white
        // 5 4
        // 5 5
        // 4 5
        // 5 3
        // 4 2
        // 3 1
        // 3 2
        // 3 5
        // 2 3
        // 1 3
        //
        let mut osero = OseroBoard::new_game();
        osero.put(5, 4, OseroStone::Black).unwrap();
        osero.print();
        osero.put(5, 5, OseroStone::White).unwrap();
        osero.print();
        osero.put(4, 5, OseroStone::Black).unwrap();
        osero.print();
        osero.put(5, 3, OseroStone::White).unwrap();
        osero.print();
        osero.put(4, 2, OseroStone::Black).unwrap();
        osero.print();
        osero.put(3, 1, OseroStone::White).unwrap();
        osero.print();
        osero.put(3, 2, OseroStone::Black).unwrap();
        osero.print();
        osero.put(3, 5, OseroStone::White).unwrap();
        osero.print();
        osero.put(2, 3, OseroStone::Black).unwrap();
        osero.print();
        osero.put(1, 3, OseroStone::White).unwrap();
        osero.print();
        assert_eq!(osero.get_black_num(), 0);
        assert_eq!(osero.get_white_num(), 14);
    }

    #[test]
    fn is_pass_test() {
        let osero = OseroBoard::new_game();
        assert_eq!(osero.is_pass(OseroStone::Black), false);
        impl OseroBoard {
            pub fn almost_fill() -> Self {
                let mut masu: MasuBoard<OseroStone> = MasuBoard::new(8, 8);
                for i in 0..7 {
                    for j in 0..8 {
                        masu.change(i, j, OseroStone::Black)
                    }
                }
                OseroBoard { masu }
            }
        }
        let osero = OseroBoard::almost_fill();
        assert_eq!(osero.is_pass(OseroStone::White), true);
        let mut osero = OseroBoard::new_game();
        osero.put(5, 4, OseroStone::Black).unwrap();
        osero.put(5, 5, OseroStone::White).unwrap();
        osero.put(3, 2, OseroStone::Black).unwrap();
        osero.put(6, 4, OseroStone::White).unwrap();
        osero.put(7, 4, OseroStone::Black).unwrap();
        osero.put(7, 3, OseroStone::White).unwrap();
        osero.put(5, 6, OseroStone::Black).unwrap();
        osero.put(7, 5, OseroStone::White).unwrap();
        assert_eq!(osero.is_pass(OseroStone::Black), true);
    }
    #[test]
    fn is_fill_test() {
        impl OseroBoard {
            pub fn fill() -> Self {
                let mut masu: MasuBoard<OseroStone> = MasuBoard::new(8, 8);
                for i in 0..8 {
                    for j in 0..8 {
                        masu.change(i, j, OseroStone::Black)
                    }
                }
                OseroBoard { masu }
            }
        }
        let osero = OseroBoard::new_game();
        assert_eq!(osero.is_fill(), false);
        let osero = OseroBoard::fill();
        assert_eq!(osero.is_fill(), true);
    }
    #[test]
    fn put_test() {
        let mut osero = OseroBoard::new_game();
        osero.put(2, 3, OseroStone::Black).unwrap();
        assert_eq!(osero.masu(3, 3), OseroStone::Black);
        osero.put(4, 2, OseroStone::White).unwrap();
        assert_eq!(osero.masu(4, 3), OseroStone::White);
        osero.put(5, 3, OseroStone::Black).unwrap();
        assert_eq!(osero.masu(4, 3), OseroStone::Black);
        osero.put(2, 2, OseroStone::White).unwrap();
        assert_eq!(osero.masu(3, 3), OseroStone::White);
    }
    #[test]
    fn get_num() {
        let mut osero = OseroBoard::new_game();
        assert_eq!(osero.get_black_num(), 2);
        assert_eq!(osero.get_white_num(), 2);
        osero.put(2, 3, OseroStone::Black).unwrap();
        assert_eq!(osero.get_black_num(), 4);
        assert_eq!(osero.get_white_num(), 1);
    }
}
