use crate::masu::masu::MasuBoard;

use super::c_or_x::CorX;

pub struct GomokuBoard {
    masu: MasuBoard<CorX>,
}
impl GomokuBoard {
    pub fn new() -> Self {
        let masu: MasuBoard<CorX> = MasuBoard::new(3, 3);

        GomokuBoard { masu }
    }
    fn masu(&self, holizon: usize, valtical: usize) -> CorX {
        self.masu.get(holizon, valtical).unwrap()
    }
    pub fn print(&self) {
        self.masu.print()
    }
    pub fn put(&mut self, holizon: usize, valtical: usize, koma: CorX) -> Result<(), String> {
        if self.is_puttable(holizon, valtical) {
            match self.masu.change(holizon, valtical, koma) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        } else {
            Err(format!("can not put [{},{}]", holizon, valtical))
        }
    }
    pub fn is_puttable(&self, holizon: usize, valtical: usize) -> bool {
        if holizon >= 3 || valtical >= 3 {
            return false;
        }
        self.masu(holizon, valtical) == CorX::Empty
    }
    pub fn winner(&self) -> Option<CorX> {
        // check indexs
        if self.is_down_right_align(0, 0) {
            return Some(self.masu(0, 0));
        }
        if self.is_down_left_align(2, 0) {
            return Some(self.masu(2, 0));
        }
        for h in 0..3 {
            if self.is_down_align(h, 0) {
                return Some(self.masu(h, 0));
            }
        }
        for v in 0..3 {
            if self.is_right_align(v, 0) {
                return Some(self.masu(v, 0));
            }
        }
        //todo
        None
    }
    fn is_down_left_align(&self, holizon: usize, valtical: usize) -> bool {
        let koma = self.masu(holizon, valtical);
        match koma {
            CorX::Empty => false,
            _ => {
                let down = self.masu.get_down_left(holizon, valtical);
                match down {
                    Ok(down) => {
                        if down == koma {
                            self.is_down_left_align(holizon - 1, valtical + 1)
                        } else {
                            false
                        }
                    }
                    Err(_) => true,
                }
            }
        }
    }
    fn is_down_right_align(&self, holizon: usize, valtical: usize) -> bool {
        let koma = self.masu(holizon, valtical);
        match koma {
            CorX::Empty => false,
            _ => {
                let down = self.masu.get_down_right(holizon, valtical);
                match down {
                    Ok(down) => {
                        if down == koma {
                            self.is_down_right_align(holizon + 1, valtical + 1)
                        } else {
                            false
                        }
                    }
                    Err(_) => true,
                }
            }
        }
    }
    fn is_right_align(&self, holizon: usize, valtical: usize) -> bool {
        let koma = self.masu(holizon, valtical);
        match koma {
            CorX::Empty => false,
            _ => {
                let down = self.masu.get_right(holizon, valtical);
                match down {
                    Ok(down) => {
                        if down == koma {
                            self.is_right_align(holizon + 1, valtical)
                        } else {
                            false
                        }
                    }
                    Err(_) => true,
                }
            }
        }
    }
    fn is_down_align(&self, holizon: usize, valtical: usize) -> bool {
        let koma = self.masu(holizon, valtical);
        match koma {
            CorX::Empty => false,
            _ => {
                let down = self.masu.get_down(holizon, valtical);
                match down {
                    Ok(down) => {
                        if down == koma {
                            self.is_down_align(holizon, valtical + 1)
                        } else {
                            false
                        }
                    }
                    Err(_) => true,
                }
            }
        }
    }
}

#[cfg(test)]
mod gomoku_test {
    use crate::gomoku_narabe::c_or_x::CorX;

    use super::GomokuBoard;
    #[test]
    fn winner_test() {
        let mut board = GomokuBoard::new();
        board.put(0, 0, CorX::Circle).unwrap();
        board.put(0, 1, CorX::Circle).unwrap();
        board.put(0, 2, CorX::Circle).unwrap();
        assert_eq!(board.winner(), Some(CorX::Circle));
        let mut board = GomokuBoard::new();
        board.put(0, 0, CorX::Circle).unwrap();
        board.put(1, 0, CorX::Circle).unwrap();
        board.put(2, 0, CorX::Circle).unwrap();
        assert_eq!(board.winner(), Some(CorX::Circle));
        let mut board = GomokuBoard::new();
        board.put(0, 0, CorX::Circle).unwrap();
        board.put(1, 1, CorX::Circle).unwrap();
        board.put(2, 2, CorX::Circle).unwrap();
        assert_eq!(board.winner(), Some(CorX::Circle));
        let mut board = GomokuBoard::new();
        board.put(2, 0, CorX::Circle).unwrap();
        board.put(1, 1, CorX::Circle).unwrap();
        board.put(0, 2, CorX::Circle).unwrap();
        assert_eq!(board.winner(), Some(CorX::Circle));
    }
    #[test]
    fn is_puttable() {
        let mut board = GomokuBoard::new();
        board.put(0, 0, CorX::Circle).unwrap();
        assert_eq!(board.is_puttable(0, 0), false);
    }

    #[test]
    fn put_test() {
        let mut board = GomokuBoard::new();
        board.put(0, 0, CorX::Circle).unwrap();
        assert_eq!(board.masu(0, 0), CorX::Circle)
    }
}
