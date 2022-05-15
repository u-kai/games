use crate::masu::masu::Masu;

use super::stone::OseroStone;
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
    pub fn print(&self) {
        self.masu.print()
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
}
