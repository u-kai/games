use std::fmt::Debug;

#[derive(PartialEq, Clone)]
pub struct PicrossFiled {
    filed: Vec<Vec<Squares>>,
    left_rules: Vec<Vec<usize>>,
    above_rules: Vec<Vec<usize>>,
}
impl PicrossFiled {
    pub fn new(filed: Vec<Vec<SquaresColor>>) -> Self {
        let len = filed.len();
        let squares_filed = filed
            .iter()
            .map(|row| {
                row.iter()
                    .map(|square| square.to_init_squares())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let mut above_rules = vec![vec![0; len]; len];
        let mut left_rules = vec![vec![0; len]; len];
        let mut above_indexs = vec![0; len];
        for (i, row) in filed.into_iter().enumerate() {
            let mut left_index = 0;
            for (j, cell) in row.into_iter().enumerate() {
                if cell == SquaresColor::White {
                    above_indexs[j] += 1;
                    left_index += 1;
                }
                if cell == SquaresColor::Black {
                    above_rules[j][above_indexs[j]] += 1;
                    left_rules[i][left_index] += 1;
                }
            }
        }
        Self {
            filed: squares_filed,
            above_rules: Self::filtering_rule(above_rules),
            left_rules: Self::filtering_rule(left_rules),
        }
    }
    pub fn is_clear(&self) -> bool {
        self.filed.iter().all(|row| {
            row.iter()
                .all(|s| (s.can_fill() && s.done_fill()) || !s.can_fill())
        })
    }
    fn column(&self, j: usize) -> Vec<Squares> {
        self.filed.iter().map(|row| row[j]).collect()
    }
    fn filtering_rule(rules: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        rules
            .into_iter()
            .map(|row| {
                let excloud_zero = row.into_iter().filter(|n| *n != 0).collect::<Vec<_>>();
                if excloud_zero.len() == 0 {
                    vec![0]
                } else {
                    excloud_zero
                }
            })
            .collect()
    }
    pub fn fill_charenge(&mut self, i: usize, j: usize) -> bool {
        if !self.can_fill(i, j) {
            println!("i {} j {} is not fill", i, j);
            false
        } else {
            self.filed[i][j].fill();
            true
        }
    }
    fn square_num(&self) -> usize {
        self.filed.len()
    }
    fn can_fill(&self, i: usize, j: usize) -> bool {
        if i >= self.square_num() || j >= self.square_num() {
            panic!(
                "i = {} j = {} square_num = {} so i or j is begger than square_num",
                i,
                j,
                self.square_num()
            );
        };
        self.filed[i][j].can_fill()
    }
}

impl Debug for PicrossFiled {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for row in &self.filed {
            let mut r = String::new();
            for cell in row {
                let square = if cell.color == SquaresColor::Black {
                    "■"
                } else {
                    "□"
                };
                r = format!("{}{} ", r, square);
            }
            s = format!("{}{}\n", s, r)
        }
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod picross_test {
    use super::*;
    fn picross_sample() -> PicrossFiled {
        let black = SquaresColor::Black;
        let white = SquaresColor::default();
        let picross = PicrossFiled::new(vec![
            vec![black, black, white, white, white],
            vec![black, white, white, white, white],
            vec![black, black, black, white, white],
            vec![black; 5],
            vec![black, white, black, white, white],
        ]);
        picross
    }
    fn fill_sample() -> PicrossFiled {
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

        picross
    }
    #[test]
    fn is_clear_test() {
        let mut sample = picross_sample();
        sample.fill_charenge(0, 0);
        assert!(!sample.is_clear());
        sample.fill_charenge(0, 1);
        assert!(!sample.is_clear());
        sample.fill_charenge(0, 2);
        assert!(!sample.is_clear());
        sample.fill_charenge(1, 0);
        assert!(!sample.is_clear());
        sample.fill_charenge(2, 0);
        assert!(!sample.is_clear());
        sample.fill_charenge(2, 1);
        assert!(!sample.is_clear());
        sample.fill_charenge(2, 2);
        assert!(!sample.is_clear());
        sample.fill_charenge(3, 0);
        assert!(!sample.is_clear());
        sample.fill_charenge(3, 1);
        assert!(!sample.is_clear());
        sample.fill_charenge(3, 2);
        assert!(!sample.is_clear());
        sample.fill_charenge(3, 3);
        assert!(!sample.is_clear());
        sample.fill_charenge(3, 4);
        assert!(!sample.is_clear());
        sample.fill_charenge(4, 0);
        assert!(!sample.is_clear());
        sample.fill_charenge(4, 2);
        assert!(sample.is_clear());

        //      3 2
        //      1 1 3 1 2
        //    2 ■ ■ □ □ □
        //    2 ■ ■ □ □ □
        //  1 1 ■ □ ■ □ □
        //    4 □ ■ ■ ■ ■
        //1 1 1 ■ □ ■ □ ■
        let black = SquaresColor::Black;
        let white = SquaresColor::default();
        let mut filed = PicrossFiled::new(vec![
            vec![black, black, white, white, white],
            vec![black, black, white, white, white],
            vec![black, white, black, white, white],
            vec![white, black, black, black, black],
            vec![black, white, black, white, black],
        ]);
        filed.fill_charenge(0, 0);
        assert!(!filed.is_clear());
        filed.fill_charenge(0, 1);
        assert!(!filed.is_clear());
        filed.fill_charenge(1, 0);
        assert!(!filed.is_clear());
        filed.fill_charenge(1, 1);
        assert!(!filed.is_clear());
        filed.fill_charenge(2, 0);
        assert!(!filed.is_clear());
        filed.fill_charenge(2, 2);
        assert!(!filed.is_clear());
        filed.fill_charenge(3, 1);
        assert!(!filed.is_clear());
        filed.fill_charenge(3, 2);
        assert!(!filed.is_clear());
        filed.fill_charenge(3, 3);
        assert!(!filed.is_clear());
        filed.fill_charenge(3, 4);
        assert!(!filed.is_clear());
        filed.fill_charenge(4, 0);
        assert!(!filed.is_clear());
        filed.fill_charenge(4, 2);
        assert!(!filed.is_clear());
        filed.fill_charenge(4, 4);
        assert!(filed.is_clear());
    }
    #[test]
    fn fill_charenge_test() {
        let mut picross = fill_sample();
        assert!(!picross.fill_charenge(4, 2));
        assert!(!picross.fill_charenge(4, 3));
    }
    #[test]
    fn new_test() {
        let black = SquaresColor::Black;
        let white = SquaresColor::default();
        //      1
        //    5 2 3 1 1
        //  2 ■ ■ □ □ □
        //  1 ■ □ □ □ □
        //  3 ■ ■ ■ □ □
        //  5 ■ ■ ■ ■ ■
        //1 1 ■ □ ■ □ □
        let filed = PicrossFiled::new(vec![
            vec![black, black, white, white, white],
            vec![black, white, white, white, white],
            vec![black, black, black, white, white],
            vec![black; 5],
            vec![black, white, black, white, white],
        ]);
        assert_eq!(
            filed.left_rules,
            vec![vec![2], vec![1], vec![3], vec![5], vec![1, 1]],
        );
        assert_eq!(
            filed.above_rules,
            vec![vec![5], vec![1, 2], vec![3], vec![1], vec![1]],
        );
        //        1
        //    1 3 1
        //  3 ■ ■ ■
        //  1 □ ■ □
        //  2 □ ■ ■
        let filed = PicrossFiled::new(vec![
            vec![black, black, black],
            vec![white, black, white],
            vec![white, black, black],
        ]);
        assert_eq!(filed.left_rules, vec![vec![3], vec![1], vec![2]],);
        assert_eq!(filed.above_rules, vec![vec![1], vec![3], vec![1, 1]],);

        //      3 2
        //      1 1 3 1 2
        //    2 ■ ■ □ □ □
        //    2 ■ ■ □ □ □
        //  1 1 ■ □ ■ □ □
        //    4 □ ■ ■ ■ ■
        //1 1 1 ■ □ ■ □ ■
        let filed = PicrossFiled::new(vec![
            vec![black, black, white, white, white],
            vec![black, black, white, white, white],
            vec![black, white, black, white, white],
            vec![white, black, black, black, black],
            vec![black, white, black, white, black],
        ]);
        assert_eq!(
            filed.left_rules,
            vec![vec![2], vec![2], vec![1, 1], vec![4], vec![1, 1, 1]],
        );
        assert_eq!(
            filed.above_rules,
            vec![vec![3, 1], vec![2, 1], vec![3], vec![1], vec![2]],
        );
    }
    #[test]
    fn can_fill_test() {
        //        1
        //    1 3 1
        //  3 ■ ■ ■
        //  1 □ ■ □
        //  2 □ ■ ■
        let ok = Squares::new(SquaresColor::White, true);
        let ng = Squares::new(SquaresColor::White, false);
        let mut filed = PicrossFiled {
            filed: vec![vec![ok, ok, ok], vec![ng, ok, ng], vec![ng, ok, ok]],
            left_rules: vec![vec![3], vec![1], vec![2]],
            above_rules: vec![vec![1], vec![3], vec![1, 1]],
        };
        assert!(filed.can_fill(0, 0));
        assert!(filed.can_fill(0, 1));
        assert!(filed.can_fill(0, 2));
        filed.fill_charenge(2, 1);
        filed.fill_charenge(2, 2);
        println!("{:?}", filed);
        assert!(!filed.can_fill(2, 0))
    }
}
#[derive(PartialEq, Clone, Copy, Debug)]
pub(super) struct Squares {
    color: SquaresColor,
    can_fill: bool,
}
impl Default for Squares {
    fn default() -> Self {
        Self {
            color: Default::default(),
            can_fill: true,
        }
    }
}
impl Squares {
    fn new(color: SquaresColor, can_fill: bool) -> Self {
        Self { color, can_fill }
    }
    fn fill(&mut self) {
        self.color.fill();
    }
    fn reverse(&mut self) {
        self.color.reverse()
    }
    fn done_fill(&self) -> bool {
        self.color.done_fill()
    }
    fn can_fill(&self) -> bool {
        !self.color.done_fill() && self.can_fill
    }
}
#[derive(PartialEq, Clone, Copy)]
pub enum SquaresColor {
    White,
    Black,
}

impl SquaresColor {
    fn fill(&mut self) {
        *self = Self::Black
    }
    fn to_init_squares(&self) -> Squares {
        match self {
            Self::Black => Squares {
                color: Self::White,
                can_fill: true,
            },
            Self::White => Squares {
                color: Self::White,
                can_fill: false,
            },
        }
    }
    fn reverse(&mut self) {
        match self {
            Self::Black => *self = Self::White,
            Self::White => *self = Self::Black,
        }
    }
    fn done_fill(&self) -> bool {
        *self == Self::Black
    }
}
impl Default for SquaresColor {
    fn default() -> Self {
        Self::White
    }
}

impl Debug for SquaresColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::White => write!(f, "□"),
            Self::Black => write!(f, "■"),
        }
    }
}
