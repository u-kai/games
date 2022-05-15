use osero::game::OseroCLI;

mod gomoku_narabe;
mod masu;
mod osero;
fn main() {
    let mut osero = OseroCLI::new();
    osero.start()
}
