use osero::{game::Osero, stone::OseroStone};

mod masu;
mod osero;
fn main() {
    let mut osero = Osero::new_game();
    osero.print();
    osero.put(2, 3, OseroStone::Black);
    osero.print();
    osero.put(4, 2, OseroStone::White);
    osero.print();
    osero.put(5, 3, OseroStone::Black);
    osero.print();
    osero.put(2, 2, OseroStone::White);
    osero.print();
    osero.put(4, 1, OseroStone::Black);
    osero.print();
    osero.put(4, 0, OseroStone::White);
    osero.print();
    osero.put(5, 2, OseroStone::Black);
    osero.print();
}
