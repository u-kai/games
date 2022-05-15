use osero::{game::Osero, stone::OseroStone};

mod masu;
mod osero;
fn main() {
    let mut osero = Osero::new_game();
    osero.print()
}
