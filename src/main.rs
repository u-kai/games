use picross::game::{PicrossFiled, SquaresColor};

use crate::helper::input::get_i_j;

mod gomoku_narabe;
mod helper;
mod masu;
mod osero;
mod picross;
mod syogi;

fn main() {
    let black = SquaresColor::Black;
    let white = SquaresColor::default();
    let mut picross = PicrossFiled::new(vec![
        vec![white, white, black, white, white],
        vec![white, black, black, black, white],
        vec![white, black, black, black, white],
        vec![white, black, black, black, white],
        vec![white, black, black, black, white],
        vec![white, white, black, white, white],
        vec![white, white, black, white, white],
        vec![white, white, black, white, white],
        vec![black, black, black, black, black],
        vec![black, black, black, black, black],
    ]);
    println!("スタート");
    println!();
    loop {
        println!("{:?}", picross);
        let d = get_i_j();
        if picross.fill_charenge(d.0, d.1) {
            if picross.is_clear() {
                println!("クリア");
                break;
            }
        } else {
            println!("そこには置けません")
        };
    }
    println!("{:?}", picross);
}
