use std::fmt::Debug;

#[derive(PartialEq, Clone)]
pub(super) struct PicrossFiled {
    filed: Vec<Vec<Squares>>,
    left_conditions: Vec<Vec<usize>>,
    above_conditions: Vec<Vec<usize>>,
}
impl PicrossFiled {
    fn new(filed: Vec<Vec<SquaresColor>>) -> Self {
        //      1
        //    5 2 3 1 1
        //  2 ■ ■ □ □ □
        //  1 ■ □ □ □ □
        //  3 ■ ■ ■ □ □
        //  5 ■ ■ ■ ■ ■
        //1 1 ■ □ ■ □ □
        let len = filed.len();
        let mut above_conditions = vec![vec![0; len]; len];
        let mut left_conditions = vec![vec![0; len]; len];
        let mut above_indexs = vec![0; len];
        for (i, row) in filed.into_iter().enumerate() {
            let mut left_index = 0;
            for (j, cell) in row.into_iter().enumerate() {
                if cell == SquaresColor::White {
                    above_indexs[j] += 1;
                    left_index += 1;
                }
                if cell == SquaresColor::Black {
                    above_conditions[j][above_indexs[j]] += 1;
                    left_conditions[i][left_index] += 1;
                }
            }
        }
        Self {
            filed: vec![vec![Default::default(); len]; len],
            above_conditions: Self::filtering_condition(above_conditions),
            left_conditions: Self::filtering_condition(left_conditions),
        }
    }
    fn filtering_condition(conditions: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        conditions
            .into_iter()
            .map(|row| {
                let mut excloud_zero = row.into_iter().filter(|n| *n != 0).collect::<Vec<_>>();
                excloud_zero.reverse();
                if excloud_zero.len() == 0 {
                    vec![0]
                } else {
                    excloud_zero
                }
            })
            .collect()
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
        }
        self.filed[i][j].can_fill()
    }
}

#[cfg(test)]
mod picross_test {
    use super::*;
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
            filed.left_conditions,
            vec![vec![2], vec![1], vec![3], vec![5], vec![1, 1]],
        );
        assert_eq!(
            filed.above_conditions,
            vec![vec![5], vec![2, 1], vec![3], vec![1], vec![1]],
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
        assert_eq!(filed.left_conditions, vec![vec![3], vec![1], vec![2]],);
        assert_eq!(filed.above_conditions, vec![vec![1], vec![3], vec![1, 1]],);

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
            filed.left_conditions,
            vec![vec![2], vec![2], vec![1, 1], vec![4], vec![1, 1, 1]],
        );
        assert_eq!(
            filed.above_conditions,
            vec![vec![1, 3], vec![1, 2], vec![3], vec![1], vec![2]],
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
        let filed = PicrossFiled {
            filed: vec![vec![ok, ok, ok], vec![ng, ok, ng], vec![ng, ok, ok]],
            left_conditions: vec![vec![3], vec![1], vec![2]],
            above_conditions: vec![vec![1], vec![3], vec![1, 1]],
        };
        assert!(filed.can_fill(0, 0));
        assert!(filed.can_fill(0, 1));
        assert!(filed.can_fill(0, 2));
        assert!(!filed.can_fill(2, 0))
    }
}
#[derive(PartialEq, Clone, Copy)]
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
pub(super) enum SquaresColor {
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
