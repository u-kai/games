use gomoku_narabe::game::GomokuCLI;
use osero::game::OseroCLI;
use syogi::board::SyogiBoard;

mod gomoku_narabe;
mod masu;
mod osero;

mod syogi;

fn main() {
    let syougi = SyogiBoard::new();
    syougi.print()
}
