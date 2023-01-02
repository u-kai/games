use std::io::stdin;

use super::{board::OseroBoard, stone::OseroStone};

#[derive(Debug, Clone, Copy)]
struct OseroPlayer {
    stone: OseroStone,
}
impl OseroPlayer {
    pub fn new(stone: OseroStone) -> Self {
        OseroPlayer { stone }
    }
    pub fn get_stone(&self) -> OseroStone {
        self.stone
    }

    pub fn stone_color(&self) -> &str {
        self.stone.color()
    }
}

/// 5 4
/// 5 5
/// 4 5
/// 5 3
/// 4 2
/// 3 1
/// 3 2
/// 3 5
/// 2 3
/// 1 3
pub struct OseroCLI {
    player1: OseroPlayer,
    player2: OseroPlayer,
    board: OseroBoard,
    turn: OseroPlayer,
}
impl OseroCLI {
    pub fn new() -> Self {
        let player1 = OseroPlayer::new(OseroStone::Black);
        let player2 = OseroPlayer::new(OseroStone::White);
        OseroCLI {
            player1,
            player2,
            board: OseroBoard::new_game(),
            turn: player1,
        }
    }
    pub fn start(&mut self) {
        println!("START Osero!!!");
        self.board.print();
        loop {
            if self.board.is_pass(self.turn.get_stone()) {
                self.switch_turn();
                continue;
            }
            println!("{}の番です", self.turn.stone_color());
            let h_v = self.get_h_v();
            match self.board.put(h_v.0, h_v.1, self.turn.get_stone()) {
                Ok(_) => self.board.print(),
                Err(_) => {
                    println!(
                        "[{},{}]には置けません.もう一度選択してください",
                        h_v.0, h_v.1
                    );
                    continue;
                }
            }
            if self.board.is_fill()
                || (self.board.is_pass(OseroStone::White) && self.board.is_pass(OseroStone::Black))
            {
                break;
            }
            self.switch_turn()
        }
        let message = if self.board.get_black_num() > self.board.get_white_num() {
            format!("{} is win", self.player1.stone_color())
        } else if self.board.get_black_num() < self.board.get_white_num() {
            format!("{} is win", self.player2.stone_color())
        } else {
            "Drow".to_string()
        };
        println!("{}", message);
    }
    fn switch_turn(&mut self) {
        match self.turn.get_stone() {
            OseroStone::Black => self.turn = self.player2,
            OseroStone::White => self.turn = self.player1,
            _ => panic!("not player is empty"),
        }
    }
    fn get_h_v(&self) -> (usize, usize) {
        let mut buf = String::new();
        let _ = stdin().read_line(&mut buf);
        let vec = buf
            .split(" ")
            .filter_map(|s| s.trim().parse::<usize>().ok())
            .collect::<Vec<_>>();
        (vec[0], vec[1])
    }
}
