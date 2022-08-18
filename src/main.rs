use picross::game::{PicrossFiled, SquaresColor};

mod gomoku_narabe;
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
    picross.fill_charenge(0, 0);
    picross.fill_charenge(0, 1);
    picross.fill_charenge(0, 2);
    picross.fill_charenge(1, 0);
    picross.fill_charenge(1, 1);
    picross.fill_charenge(2, 0);
    picross.fill_charenge(2, 1);
    picross.fill_charenge(2, 2);
    picross.fill_charenge(3, 0);
    picross.fill_charenge(3, 1);
    picross.fill_charenge(3, 2);
    picross.fill_charenge(3, 3);
    picross.fill_charenge(3, 4);
    picross.fill_charenge(4, 0);
    picross.fill_charenge(4, 2);
    println!("{:?}", picross);
}
