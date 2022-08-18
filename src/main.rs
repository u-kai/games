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
        vec![black, black, white, white, white],
        vec![black, white, white, white, white],
        vec![black, black, black, white, white],
        vec![black; 5],
        vec![black, white, black, white, white],
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
