use std::fmt::Debug;

#[derive(PartialEq, Clone)]
pub struct PicrossFiled {
    filed: Vec<Vec<Squares>>,
    left_rules: Vec<Vec<usize>>,
    above_rules: Vec<Vec<usize>>,
}
impl PicrossFiled {
    pub fn new(filed: Vec<Vec<SquaresColor>>) -> Self {
        //      1
        //    5 2 3 1 1
        //  2 ■ ■ □ □ □
        //  1 ■ □ □ □ □
        //  3 ■ ■ ■ □ □
        //  5 ■ ■ ■ ■ ■
        //1 1 ■ □ ■ □ □
        let len = filed.len();
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
            filed: vec![vec![Default::default(); len]; len],
            above_rules: Self::filtering_rule(above_rules),
            left_rules: Self::filtering_rule(left_rules),
        }
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
    fn simulation(&self, i: usize, j: usize) -> bool {
        //      1
        //    5 2 3 1 1
        //  2 ■ ■ □ □ □
        //  1 ■ ■ □ □ □
        //  3 ■ ■ ■ □ □
        //  5 ■ ■ ■ ■ ■
        //1 1 ■ □ ■ □ □
        let mut copy = self.clone();
        copy.filed[i][j].fill();
        let row = copy.filed[i].clone();
        let column = copy.column(j);
        let mut row_rule = (&self.left_rules[i]).into_iter();
        let mut column_rule = (&self.above_rules[j]).into_iter();
        println!("i = {} j = {}", i, j);
        println!("row = {:?}", row);
        println!("row_rule = {:?}", row_rule);

        println!("column = {:?}", column);
        println!("row_rule = {:?}", row_rule);
        if row_ok(row, &mut row_rule, 0) && row_ok(column, &mut column_rule, 0) {
            return true;
        }
        false
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
        if self.filed[i][j].color == SquaresColor::Black {
            return false;
        }
        self.simulation(i, j)
    }
    fn column_filled_num(&self, j: usize) -> usize {
        self.filed.iter().fold(0, |acc, row| {
            acc + {
                row.iter().enumerate().fold(0, |acc, (index, cell)| {
                    acc + if cell.color.done_fill() && j == index {
                        1
                    } else {
                        0
                    }
                })
            }
        })
    }
    fn row_filled_num(&self, i: usize) -> usize {
        self.filed[i].iter().fold(0, |acc, cell| {
            acc + if cell.color.done_fill() { 1 } else { 0 }
        })
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
fn row_ok<'a>(
    v: Vec<Squares>,
    rules: &mut impl Iterator<Item = &'a usize>,
    start_index: usize,
) -> bool {
    let rule = rules.next();
    if rule == None {
        return true;
    }
    let rule = rule.unwrap();
    let start = v.iter().enumerate().skip(start_index);
    let start_black = start.skip_while(|(_, s)| s.color == SquaresColor::White);
    let sequence_black = start_black
        .take_while(|(_, s)| s.color == SquaresColor::Black)
        .collect::<Vec<_>>();
    let len = sequence_black.len();
    if len > *rule {
        return false;
    }
    let next_start_index = match sequence_black.last() {
        Some(last) => last.0,
        None => {
            if *rule == 0 {
                return true;
            }
            v.len()
            //panic!("rule should be 0 but rule = {}", rule);
        }
    };
    row_ok(v, rules, next_start_index)
}

#[cfg(test)]
mod picross_test {
    use super::*;
    #[test]
    fn row_simutaltion_test() {
        let black = Squares::new(SquaresColor::Black, true);
        let white = Squares::new(SquaresColor::White, true);

        let row = vec![black, white, black, white, white];
        let rules: [usize; 2] = [1, 1];
        let start_index = 0;
        assert!(row_ok(row, &mut rules.iter(), start_index));

        let row = vec![black, black, white, white, black];
        let rules: [usize; 2] = [2, 1];
        let start_index = 0;
        assert!(row_ok(row, &mut rules.iter(), start_index));

        let row = vec![black, black, black, black, white];
        let rules: [usize; 2] = [2, 1];
        let start_index = 0;
        assert!(!row_ok(row, &mut rules.iter(), start_index));

        let row = vec![white, black, black, white, black];
        let rules: [usize; 2] = [2, 1];
        let start_index = 0;
        assert!(row_ok(row, &mut rules.iter(), start_index));

        let row = vec![white, white, white, white, black];
        let rules: [usize; 1] = [0];
        let start_index = 0;
        assert!(!row_ok(row, &mut rules.iter(), start_index));

        let row = vec![white, white, white, white, white];
        let rules: [usize; 1] = [0];
        let start_index = 0;
        assert!(row_ok(row, &mut rules.iter(), start_index));

        let row = vec![black, black, black, black, black];
        let rules: [usize; 1] = [5];
        let start_index = 0;
        assert!(row_ok(row, &mut rules.iter(), start_index));
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
        picross
    }
    #[test]
    fn filled_num_test() {
        let picross = fill_sample();
        println!("{:?}", picross);
        //row
        assert_eq!(picross.row_filled_num(0), 2);
        assert_eq!(picross.row_filled_num(1), 1);
        assert_eq!(picross.row_filled_num(2), 3);
        assert_eq!(picross.row_filled_num(3), 5);
        assert_eq!(picross.row_filled_num(4), 2);
        //column
        assert_eq!(picross.column_filled_num(0), 5);
        assert_eq!(picross.column_filled_num(1), 3);
        assert_eq!(picross.column_filled_num(2), 3);
        assert_eq!(picross.column_filled_num(3), 1);
        assert_eq!(picross.column_filled_num(4), 1);
    }
    #[test]
    fn fill_charenge_test() {
        let black = SquaresColor::Black;
        let white = SquaresColor::default();
        let mut picross = PicrossFiled::new(vec![
            vec![black, black, white, white, white],
            vec![black, white, white, white, white],
            vec![black, black, black, white, white],
            vec![black; 5],
            vec![black, white, black, white, white],
        ]);
        assert!(picross.fill_charenge(0, 1));
        assert!(picross.fill_charenge(0, 2));
        assert!(picross.fill_charenge(1, 0));
        assert!(picross.fill_charenge(2, 0));
        assert!(picross.fill_charenge(2, 1));
        assert!(picross.fill_charenge(2, 2));
        assert!(picross.fill_charenge(3, 0));
        assert!(picross.fill_charenge(3, 1));
        assert!(picross.fill_charenge(3, 2));
        assert!(picross.fill_charenge(3, 3));
        assert!(picross.fill_charenge(3, 4));
        assert!(picross.fill_charenge(4, 0));
        assert!(picross.fill_charenge(4, 2));
        assert!(!picross.fill_charenge(4, 2));
        //assert!(!picross.fill_charenge(4, 3));
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
    fn can_fill(&self) -> bool {
        self.color == SquaresColor::White && self.can_fill
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
