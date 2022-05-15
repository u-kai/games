use crate::masu::masu::Masu;

use super::stone::{self, OseroStone};
pub struct Osero {
    masu: Masu<OseroStone>,
}

impl Osero {
    pub fn new_game() -> Osero {
        let mut masu = Masu::new(8, 8);
        masu.change(3, 3, OseroStone::White);
        masu.change(3, 4, OseroStone::Black);
        masu.change(4, 3, OseroStone::Black);
        masu.change(4, 4, OseroStone::White);
        Osero { masu }
    }
    pub fn masu(&self, holizon: usize, valtical: usize) -> OseroStone {
        self.masu.get(holizon, valtical)
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
            self.challenge_up(holizon, valtical, stone);
            self.challenge_down(holizon, valtical, stone);
            self.challenge_right(holizon, valtical, stone);
            self.challenge_left(holizon, valtical, stone);
            self.challenge_up_right(holizon, valtical, stone);
            self.challenge_down_right(holizon, valtical, stone);
            self.challenge_up_left(holizon, valtical, stone);
            self.challenge_down_left(holizon, valtical, stone);
            Ok(())
        } else {
            Err(format!("can not put [{},{}]", holizon, valtical))
        }
    }
    pub fn print(&self) {
        self.masu.print()
    }
    fn is_up_puttable(&self, holizon: usize, valtical: usize, stone: OseroStone) -> bool {
        self.up_next_v_index(holizon, valtical, stone).is_some()
    }
    fn is_down_puttable(&self, holizon: usize, valtical: usize, stone: OseroStone) -> bool {
        self.down_next_v_index(holizon, valtical, stone).is_some()
    }
    fn is_right_puttable(&self, holizon: usize, valtical: usize, stone: OseroStone) -> bool {
        self.right_next_h_index(holizon, valtical, stone).is_some()
    }
    fn is_left_puttable(&self, holizon: usize, valtical: usize, stone: OseroStone) -> bool {
        self.left_next_h_index(holizon, valtical, stone).is_some()
    }
    fn is_up_right_puttable(&self, holizon: usize, valtical: usize, stone: OseroStone) -> bool {
        self.up_right_next_index(holizon, valtical, stone).is_some()
    }
    fn is_down_right_puttable(&self, holizon: usize, valtical: usize, stone: OseroStone) -> bool {
        self.down_right_next_index(holizon, valtical, stone)
            .is_some()
    }
    fn is_up_left_puttable(&self, holizon: usize, valtical: usize, stone: OseroStone) -> bool {
        self.up_left_next_index(holizon, valtical, stone).is_some()
    }
    fn is_down_left_puttable(&self, holizon: usize, valtical: usize, stone: OseroStone) -> bool {
        self.down_left_next_index(holizon, valtical, stone)
            .is_some()
    }
    fn challenge_up(&mut self, holizon: usize, valtical: usize, stone: OseroStone) {
        if let Some(next_up_index) = self.up_next_v_index(holizon, valtical, stone) {
            for v in next_up_index..=valtical {
                self.masu.change(holizon, v, stone);
            }
        }
    }
    fn challenge_down(&mut self, holizon: usize, valtical: usize, stone: OseroStone) {
        if let Some(next_down_index) = self.down_next_v_index(holizon, valtical, stone) {
            for v in valtical..=next_down_index {
                self.masu.change(holizon, v, stone);
            }
        }
    }
    fn challenge_right(&mut self, holizon: usize, valtical: usize, stone: OseroStone) {
        if let Some(next_right_index) = self.right_next_h_index(holizon, valtical, stone) {
            for h in holizon..=next_right_index {
                self.masu.change(h, valtical, stone);
            }
        }
    }
    fn challenge_left(&mut self, holizon: usize, valtical: usize, stone: OseroStone) {
        if let Some(next_left_index) = self.left_next_h_index(holizon, valtical, stone) {
            for h in next_left_index..=holizon {
                self.masu.change(h, valtical, stone);
            }
        }
    }
    fn challenge_up_right(&mut self, holizon: usize, valtical: usize, stone: OseroStone) {
        if let Some((n_h, n_v)) = self.up_right_next_index(holizon, valtical, stone) {
            for (i, h) in (holizon..=n_h).enumerate() {
                self.masu.change(h, valtical - i, stone)
            }
        }
    }
    fn challenge_down_right(&mut self, holizon: usize, valtical: usize, stone: OseroStone) {
        if let Some((n_h, n_v)) = self.down_right_next_index(holizon, valtical, stone) {
            for (i, h) in (holizon..=n_h).enumerate() {
                self.masu.change(h, valtical + i, stone)
            }
        }
    }
    fn challenge_up_left(&mut self, holizon: usize, valtical: usize, stone: OseroStone) {
        if let Some((n_h, n_v)) = self.up_left_next_index(holizon, valtical, stone) {
            for (i, h) in (n_h..=holizon).enumerate() {
                self.masu.change(h, n_v + i, stone)
            }
        }
    }
    fn challenge_down_left(&mut self, holizon: usize, valtical: usize, stone: OseroStone) {
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
    use crate::osero::stone::OseroStone;

    use super::Osero;

    #[test]
    fn new_game_test() {
        let game = Osero::new_game();
        assert_eq!(game.masu(3, 3), OseroStone::White);
        assert_eq!(game.masu(3, 4), OseroStone::Black);
        assert_eq!(game.masu(4, 3), OseroStone::Black);
        assert_eq!(game.masu(4, 4), OseroStone::White);
    }
    #[test]
    fn is_puttable_test() {
        let osero = Osero::new_game();
        assert_eq!(osero.is_puttable(3, 3, OseroStone::Black), false);
        osero.print();
        assert_eq!(osero.is_puttable(2, 3, OseroStone::Black), true);
        assert_eq!(osero.is_puttable(4, 2, OseroStone::Black), true);
    }
    #[test]
    fn put_test() {
        let mut osero = Osero::new_game();
        osero.put(2, 3, OseroStone::Black);
        assert_eq!(osero.masu(3, 3), OseroStone::Black);
        osero.print();
        osero.put(4, 2, OseroStone::White);
        osero.print();
        assert_eq!(osero.masu(4, 3), OseroStone::White);
        osero.put(5, 3, OseroStone::Black);
        osero.print();
        assert_eq!(osero.masu(4, 3), OseroStone::Black);
        osero.put(2, 2, OseroStone::White);
        assert_eq!(osero.masu(3, 3), OseroStone::White);
        osero.put(5, 2, OseroStone::Black);
        assert_eq!(osero.masu(4, 3), OseroStone::Black);
        osero.print();
    }
}
