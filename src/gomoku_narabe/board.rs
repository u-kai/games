use crate::masu::masu::Masu;

use super::c_or_x::CorX;

pub struct GomokuBoard {
    masu: Masu<CorX>,
}
impl GomokuBoard {
    pub fn new() -> Self {
        let masu: Masu<CorX> = Masu::new(3, 3);

        GomokuBoard { masu }
    }
    pub fn masu(&self, holizon: usize, valtical: usize) -> CorX {
        self.masu.get(holizon, valtical)
    }
    pub fn put(&mut self, holizon: usize, valtical: usize, koma: CorX) {
        self.masu.change(holizon, valtical, koma)
    }
    pub fn is_puttable(&self, holizon: usize, valtical: usize) -> bool {
        self.masu(holizon, valtical) == CorX::Empty
    }
    pub fn winner(&self) -> Option<CorX> {
        // check indexs
        //0,0 down right down-right

        if self.is_down_aligne(0, 0) {
            return Some(self.masu(0, 0));
        }
        //1,0 down
        //2,0 down
        //0,1 right
        //0,2 right
        //todo
        None
    }
    fn is_down_aligne(&self, holizon: usize, valtical: usize) -> bool {
        let koma = self.masu(holizon, valtical);
        match koma {
            CorX::Empty => false,
            _ => {
                let down = self.masu.get_down(holizon, valtical);
                match down {
                    Ok(down) => {
                        if down == koma {
                            self.is_down_aligne(holizon, valtical + 1)
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
        board.put(0, 0, CorX::Circle);
        board.put(0, 1, CorX::Circle);
        board.put(0, 2, CorX::Circle);
        assert_eq!(board.winner(), Some(CorX::Circle));
        board.put(1, 1, CorX::Circle);
        board.put(2, 2, CorX::Circle);
        //todo
        //assert_eq!(board.winner(), Some(CorX::Circle));
    }
    #[test]
    fn is_puttable() {
        let mut board = GomokuBoard::new();
        board.put(0, 0, CorX::Circle);
        assert_eq!(board.is_puttable(0, 0), false);
    }

    #[test]
    fn put_test() {
        let mut board = GomokuBoard::new();
        board.put(0, 0, CorX::Circle);
        assert_eq!(board.masu(0, 0), CorX::Circle)
    }
}
