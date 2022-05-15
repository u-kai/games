use gomoku_narabe::game::GomokuCLI;
use osero::game::OseroCLI;

mod gomoku_narabe;
mod masu;
mod osero;
fn main() {
    let mut gomoku = GomokuCLI::new();
    gomoku.start();
    //let mut osero = OseroCLI::new();
    //osero.start()
}
