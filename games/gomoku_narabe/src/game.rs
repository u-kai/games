use std::io::stdin;

use super::{board::GomokuBoard, c_or_x::CorX};

#[derive(Debug, Clone, Copy)]
struct GomokuPlayer {
    koma: CorX,
}
impl GomokuPlayer {
    pub fn new(koma: CorX) -> Self {
        GomokuPlayer { koma }
    }
    pub fn get_koma(&self) -> CorX {
        self.koma
    }
}
pub struct GomokuCLI {
    player1: GomokuPlayer,
    player2: GomokuPlayer,
    board: GomokuBoard,
    turn: GomokuPlayer,
}
impl GomokuCLI {
    pub fn new() -> Self {
        let player1 = GomokuPlayer::new(CorX::Circle);
        let player2 = GomokuPlayer::new(CorX::Xross);
        GomokuCLI {
            player1,
            player2,
            board: GomokuBoard::new(),
            turn: player1,
        }
    }
    pub fn start(&mut self) {
        println!("START Gomoku Narabe!!!");
        self.board.print();
        loop {
            if let Some(winner) = self.board.winner() {
                println!("winner is {:?}", winner);
                break;
            }
            println!("{:?}の番です", self.turn.get_koma());
            let h_v = self.get_h_v();
            match self.board.put(h_v.0, h_v.1, self.turn.get_koma()) {
                Ok(_) => self.board.print(),
                Err(_) => {
                    println!(
                        "[{},{}]には置けません.もう一度選択してください",
                        h_v.0, h_v.1
                    );
                    continue;
                }
            }
            self.switch_turn();
        }
    }
    fn switch_turn(&mut self) {
        match self.turn.get_koma() {
            CorX::Circle => self.turn = self.player2,
            CorX::Xross => self.turn = self.player1,
            _ => panic!("not player is empty"),
        }
    }
    fn get_h_v(&self) -> (usize, usize) {
        let mut buf = String::new();
        let _ = stdin().read_line(&mut buf);
        let vec = buf
            .split(" ")
            .filter(|s| s.len() < 3) //大きい数字は受け付けない
            .filter_map(|s| s.trim().parse::<usize>().ok())
            .collect::<Vec<_>>();
        if vec.len() == 2 {
            return (vec[0], vec[1]);
        }
        (10000000, 10000000)
    }
}
