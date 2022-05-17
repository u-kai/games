use std::fmt::Debug;

use super::{calcurator::IndexCalcurator, masu::Masu};
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MasuBoard<T>
where
    T: Debug + Clone + Copy + PartialEq + Eq,
{
    masu: Vec<Vec<T>>,
    h_len: usize,
    v_len: usize,
}

impl<T> MasuBoard<T>
where
    T: Debug + Clone + Copy + PartialEq + Eq + Default,
{
    pub fn new(h_len: usize, v_len: usize) -> Self {
        MasuBoard {
            h_len,
            v_len,
            masu: vec![vec![T::default(); h_len]; v_len],
        }
    }
    pub fn all(&self) -> &Vec<Vec<T>> {
        &self.masu
    }
    fn get(&self, holizon: usize, valtical: usize) -> Result<T, String> {
        if holizon >= self.h_len || valtical >= self.v_len {
            return Err(format!("index out of bounds :[{}{}]", holizon, valtical));
        }
        Ok(self.masu[valtical][holizon])
    }
    pub fn change(&mut self, holizon: usize, valtical: usize, koma: T) -> Result<(), String> {
        if holizon >= self.h_len || valtical >= self.v_len {
            return Err(format!("index out of bounds :[{}{}]", holizon, valtical));
        }
        self.masu[valtical][holizon] = koma;
        Ok(())
    }
    pub fn get_down_right_line(&self, holizon: usize, valtical: usize) -> Vec<Masu<T>> {
        self.create_index(holizon, valtical)
            .get_down_right_line()
            .iter()
            .map(|index| self.new_masu(index.clone()))
            .collect()
    }
    pub fn get_up_right_line(&self, holizon: usize, valtical: usize) -> Vec<Masu<T>> {
        self.create_index(holizon, valtical)
            .get_up_right_line()
            .iter()
            .map(|index| self.new_masu(index.clone()))
            .collect()
    }
    pub fn get_up_left_line(&self, holizon: usize, valtical: usize) -> Vec<Masu<T>> {
        self.create_index(holizon, valtical)
            .get_up_left_line()
            .iter()
            .map(|index| self.new_masu(index.clone()))
            .collect()
    }
    pub fn get_down_left_line(&self, holizon: usize, valtical: usize) -> Vec<Masu<T>> {
        self.create_index(holizon, valtical)
            .get_down_left_line()
            .iter()
            .map(|index| self.new_masu(index.clone()))
            .collect()
    }
    pub fn get_left_line(&self, holizon: usize, valtical: usize) -> Vec<Masu<T>> {
        self.create_index(holizon, valtical)
            .get_left_line()
            .iter()
            .map(|index| self.new_masu(index.clone()))
            .collect()
    }
    pub fn get_right_line(&self, holizon: usize, valtical: usize) -> Vec<Masu<T>> {
        self.create_index(holizon, valtical)
            .get_right_line()
            .iter()
            .map(|index| self.new_masu(index.clone()))
            .collect()
    }
    pub fn get_down_line(&self, holizon: usize, valtical: usize) -> Vec<Masu<T>> {
        self.create_index(holizon, valtical)
            .get_down_line()
            .iter()
            .map(|index| self.new_masu(index.clone()))
            .collect()
    }
    pub fn get_up_line(&self, holizon: usize, valtical: usize) -> Vec<Masu<T>> {
        self.create_index(holizon, valtical)
            .get_up_line()
            .iter()
            .map(|index| self.new_masu(index.clone()))
            .collect()
    }
    pub fn get_up(&self, holizon: usize, valtical: usize) -> Result<Masu<T>, String> {
        self.get_neighbor(self.create_index(holizon, valtical).get_up())
    }
    pub fn get_down(&self, holizon: usize, valtical: usize) -> Result<Masu<T>, String> {
        self.get_neighbor(self.create_index(holizon, valtical).get_down())
    }
    pub fn get_right(&self, holizon: usize, valtical: usize) -> Result<Masu<T>, String> {
        self.get_neighbor(self.create_index(holizon, valtical).get_right())
    }
    pub fn get_left(&self, holizon: usize, valtical: usize) -> Result<Masu<T>, String> {
        self.get_neighbor(self.create_index(holizon, valtical).get_left())
    }
    pub fn get_up_left(&self, holizon: usize, valtical: usize) -> Result<Masu<T>, String> {
        self.get_neighbor(self.create_index(holizon, valtical).get_up_left())
    }
    pub fn get_down_left(&self, holizon: usize, valtical: usize) -> Result<Masu<T>, String> {
        self.get_neighbor(self.create_index(holizon, valtical).get_down_left())
    }
    pub fn get_up_right(&self, holizon: usize, valtical: usize) -> Result<Masu<T>, String> {
        self.get_neighbor(self.create_index(holizon, valtical).get_up_right())
    }
    pub fn get_down_right(&self, holizon: usize, valtical: usize) -> Result<Masu<T>, String> {
        self.get_neighbor(self.create_index(holizon, valtical).get_down_right())
    }
    fn get_neighbor(&self, index: Result<IndexCalcurator, String>) -> Result<Masu<T>, String> {
        match index {
            Ok(new_index) => Ok(self.new_masu(new_index)),
            Err(e) => Err(e),
        }
    }
    fn new_masu(&self, index: IndexCalcurator) -> Masu<T> {
        let koma = self.get(index.get_h(), index.get_v()).unwrap();
        Masu::new(koma, index.get_h(), index.get_v())
    }
    fn create_index(&self, holizon: usize, valtical: usize) -> IndexCalcurator {
        IndexCalcurator::new(self.h_len, self.v_len, holizon, valtical)
    }

    pub fn print(&self) {
        let mut log = String::new();
        for v in &self.masu {
            //log = format!("{}\n{}\n", log, "--".repeat(self.0.len()));
            for h in v {
                log = format!("{}|{:?}", log, h);
            }
            log = format!("{}|\n", log)
        }
        println!("{}", log);
    }
}

#[cfg(test)]
mod masu_new_test {
    #[test]
    fn get_down_right_line_test() {
        // Point -> |Y| | | | |
        //          | | | | | |
        //          | | | | | |
        //          | | | |N| |
        //          | | | | |Y|
        let mut masu: MasuBoard<Mock> = MasuBoard::new(5, 5);
        masu.change(0, 0, Mock::Yes).unwrap();
        masu.change(3, 3, Mock::No).unwrap();
        masu.change(4, 4, Mock::Yes).unwrap();
        assert_eq!(
            masu.get_down_right_line(0, 0),
            vec![
                Masu::new(Mock::Empty, 1, 1),
                Masu::new(Mock::Empty, 2, 2),
                Masu::new(Mock::No, 3, 3),
                Masu::new(Mock::Yes, 4, 4)
            ]
        );
    }
    #[test]
    fn get_up_left_line_test() {
        //          |Y| | | | |
        //          | | | | | |
        //          | | | | | |
        //          | | | |N| |
        //          | | | | | | <-Point
        let mut masu: MasuBoard<Mock> = MasuBoard::new(5, 5);
        masu.change(0, 0, Mock::Yes).unwrap();
        masu.change(3, 3, Mock::No).unwrap();
        assert_eq!(
            masu.get_up_left_line(4, 4),
            vec![
                Masu::new(Mock::No, 3, 3),
                Masu::new(Mock::Empty, 2, 2),
                Masu::new(Mock::Empty, 1, 1),
                Masu::new(Mock::Yes, 0, 0)
            ]
        );
    }
    #[test]
    fn get_up_right_line_test() {
        //          | | | | |Y|
        //          | | | |N| |
        //          | | | | | |
        //          | |Y| | | |
        //  Point-> | | | | | |
        let mut masu: MasuBoard<Mock> = MasuBoard::new(5, 5);
        masu.change(4, 0, Mock::Yes).unwrap();
        masu.change(3, 1, Mock::No).unwrap();
        masu.change(1, 3, Mock::Yes).unwrap();
        assert_eq!(
            masu.get_up_right_line(0, 4),
            vec![
                Masu::new(Mock::Yes, 1, 3),
                Masu::new(Mock::Empty, 2, 2),
                Masu::new(Mock::No, 3, 1),
                Masu::new(Mock::Yes, 4, 0)
            ]
        );
    }
    #[test]
    fn get_down_left_line_test() {
        //          | | | | |Y|<-Point
        //          | | | |N| |
        //          | | | | | |
        //          | |Y| | | |
        //          | | | | | |
        let mut masu: MasuBoard<Mock> = MasuBoard::new(5, 5);
        masu.change(4, 0, Mock::Yes).unwrap();
        masu.change(3, 1, Mock::No).unwrap();
        masu.change(1, 3, Mock::Yes).unwrap();
        assert_eq!(
            masu.get_down_left_line(4, 0),
            vec![
                Masu::new(Mock::No, 3, 1),
                Masu::new(Mock::Empty, 2, 2),
                Masu::new(Mock::Yes, 1, 3),
                Masu::new(Mock::Empty, 0, 4)
            ]
        );
    }
    #[test]
    fn get_left_line_test() {
        //          |Y| | |N|Y|<-Point
        //          | | | | | |
        //          | | | | | |
        //          | | | | | |
        //          | | | | | |
        let mut masu: MasuBoard<Mock> = MasuBoard::new(5, 5);
        masu.change(0, 0, Mock::Yes).unwrap();
        masu.change(3, 0, Mock::No).unwrap();
        masu.change(4, 0, Mock::Yes).unwrap();
        assert_eq!(
            masu.get_left_line(4, 0),
            vec![
                Masu::new(Mock::No, 3, 0),
                Masu::new(Mock::Empty, 2, 0),
                Masu::new(Mock::Empty, 1, 0),
                Masu::new(Mock::Yes, 0, 0)
            ]
        );
    }
    #[test]
    fn get_right_line_test() {
        //  Point-> |Y| | |N|Y|
        //          | | | | | |
        //          | | | | | |
        //          | | | | | |
        //          | | | | | |
        let mut masu: MasuBoard<Mock> = MasuBoard::new(5, 5);
        masu.change(0, 0, Mock::Yes).unwrap();
        masu.change(3, 0, Mock::No).unwrap();
        masu.change(4, 0, Mock::Yes).unwrap();
        assert_eq!(
            masu.get_right_line(0, 0),
            vec![
                Masu::new(Mock::Empty, 1, 0),
                Masu::new(Mock::Empty, 2, 0),
                Masu::new(Mock::No, 3, 0),
                Masu::new(Mock::Yes, 4, 0)
            ]
        );
    }
    #[test]
    fn get_down_line_test() {
        //          |Y| | | | |
        // Point->  |N| | | | |
        //          |Y| | | | |
        //          |N| | | | |
        //          | | | | | |
        let mut masu: MasuBoard<Mock> = MasuBoard::new(5, 5);
        masu.change(0, 0, Mock::Yes).unwrap();
        masu.change(0, 1, Mock::No).unwrap();
        masu.change(0, 2, Mock::Yes).unwrap();
        masu.change(0, 3, Mock::No).unwrap();
        assert_eq!(
            masu.get_down_line(0, 1),
            vec![
                Masu::new(Mock::Yes, 0, 2),
                Masu::new(Mock::No, 0, 3),
                Masu::new(Mock::Empty, 0, 4)
            ]
        );
    }
    #[test]
    fn get_up_line_test() {
        //          |Y| | | | |
        //          |N| | | | |
        //          |Y| | | | |
        // Point->  |N| | | | |
        //          | | | | | |
        let mut masu: MasuBoard<Mock> = MasuBoard::new(5, 5);
        masu.change(0, 0, Mock::Yes).unwrap();
        masu.change(0, 1, Mock::No).unwrap();
        masu.change(0, 2, Mock::Yes).unwrap();
        masu.change(0, 3, Mock::No).unwrap();
        assert_eq!(
            masu.get_up_line(0, 3),
            vec![
                Masu::new(Mock::Yes, 0, 2),
                Masu::new(Mock::No, 0, 1),
                Masu::new(Mock::Yes, 0, 0)
            ]
        );
    }
    #[test]
    fn get_up_right_test() {
        let mut masu: MasuBoard<Mock> = MasuBoard::new(8, 8);
        masu.change(2, 2, Mock::No).unwrap();
        masu.change(1, 1, Mock::Yes).unwrap();
        assert_eq!(masu.get_up_right(1, 3).unwrap(), Masu::new(Mock::No, 2, 2));
        assert_eq!(masu.get_up_right(0, 2).unwrap(), Masu::new(Mock::Yes, 1, 1));
        assert_eq!(
            masu.get_up_right(2, 1).unwrap(),
            Masu::new(Mock::Empty, 3, 0)
        );
        assert_eq!(
            masu.get_up_right(0, 0),
            Err("[0,0] up is out bound".to_string())
        );
        assert_eq!(
            masu.get_up_right(7, 0),
            Err("[7,0] up is out bound".to_string())
        );
    }
    #[test]
    fn get_down_left_test() {
        let mut masu: MasuBoard<Mock> = MasuBoard::new(8, 8);
        masu.change(2, 2, Mock::No).unwrap();
        masu.change(1, 1, Mock::Yes).unwrap();
        assert_eq!(masu.get_down_left(3, 1).unwrap(), Masu::new(Mock::No, 2, 2));
        assert_eq!(
            masu.get_down_left(2, 0).unwrap(),
            Masu::new(Mock::Yes, 1, 1)
        );
        assert_eq!(
            masu.get_down_left(4, 4).unwrap(),
            Masu::new(Mock::Empty, 3, 5)
        );
        assert_eq!(
            masu.get_down_left(0, 7),
            Err("[0,7] down is out bound".to_string())
        );
        assert_eq!(
            masu.get_down_left(7, 7),
            Err("[7,7] down is out bound".to_string())
        );
    }
    #[test]
    fn get_up_left_test() {
        let mut masu: MasuBoard<Mock> = MasuBoard::new(8, 8);
        masu.change(2, 2, Mock::No).unwrap();
        masu.change(1, 1, Mock::Yes).unwrap();
        assert_eq!(masu.get_up_left(3, 3).unwrap(), Masu::new(Mock::No, 2, 2));
        assert_eq!(masu.get_up_left(2, 2).unwrap(), Masu::new(Mock::Yes, 1, 1));
        assert_eq!(
            masu.get_up_left(2, 1).unwrap(),
            Masu::new(Mock::Empty, 1, 0)
        );
        assert_eq!(
            masu.get_up_left(0, 0),
            Err("[0,0] up is out bound".to_string())
        );
        assert_eq!(
            masu.get_up_left(7, 0),
            Err("[7,0] up is out bound".to_string())
        );
    }
    #[test]
    fn get_down_right_test() {
        let mut masu: MasuBoard<Mock> = MasuBoard::new(8, 8);
        masu.change(2, 2, Mock::No).unwrap();
        masu.change(1, 1, Mock::Yes).unwrap();
        assert_eq!(
            masu.get_down_right(1, 1).unwrap(),
            Masu::new(Mock::No, 2, 2)
        );
        assert_eq!(
            masu.get_down_right(0, 0).unwrap(),
            Masu::new(Mock::Yes, 1, 1)
        );
        assert_eq!(
            masu.get_down_right(4, 4).unwrap(),
            Masu::new(Mock::Empty, 5, 5)
        );
        assert_eq!(
            masu.get_down_right(0, 7),
            Err("[0,7] down is out bound".to_string())
        );
        assert_eq!(
            masu.get_down_right(7, 7),
            Err("[7,7] down is out bound".to_string())
        );
    }
    #[test]
    fn get_down_test() {
        let mut masu: MasuBoard<Mock> = MasuBoard::new(8, 8);
        masu.change(2, 2, Mock::No).unwrap();
        masu.change(1, 1, Mock::Yes).unwrap();
        assert_eq!(masu.get_down(2, 1).unwrap(), Masu::new(Mock::No, 2, 2));
        assert_eq!(masu.get_down(1, 0).unwrap(), Masu::new(Mock::Yes, 1, 1,));
        assert_eq!(masu.get_down(4, 4).unwrap(), Masu::new(Mock::Empty, 4, 5));
        assert_eq!(
            masu.get_down(0, 7),
            Err("[0,7] down is out bound".to_string())
        );
        assert_eq!(
            masu.get_down(7, 7),
            Err("[7,7] down is out bound".to_string())
        );
    }
    #[test]
    fn get_up_test() {
        let mut masu: MasuBoard<Mock> = MasuBoard::new(8, 8);
        masu.change(2, 2, Mock::No).unwrap();
        masu.change(1, 1, Mock::Yes).unwrap();
        assert_eq!(masu.get_up(2, 3).unwrap(), Masu::new(Mock::No, 2, 2));
        assert_eq!(masu.get_up(1, 2).unwrap(), Masu::new(Mock::Yes, 1, 1));
        assert_eq!(masu.get_up(2, 1).unwrap(), Masu::new(Mock::Empty, 2, 0));
        assert_eq!(masu.get_up(0, 0), Err("[0,0] up is out bound".to_string()));
        assert_eq!(masu.get_up(7, 0), Err("[7,0] up is out bound".to_string()));
    }
    #[test]
    fn change_test() {
        let mut masu: MasuBoard<Mock> = MasuBoard::new(8, 8);
        masu.change(3, 5, Mock::Yes).unwrap();
        assert_eq!(masu.get(3, 5).unwrap(), Mock::Yes);
    }
    #[test]
    fn new_test() {
        let masu: MasuBoard<Mock> = MasuBoard::new(8, 8);
        assert_eq!(
            &vec![
                vec![
                    Mock::Empty,
                    Mock::Empty,
                    Mock::Empty,
                    Mock::Empty,
                    Mock::Empty,
                    Mock::Empty,
                    Mock::Empty,
                    Mock::Empty
                ],
                vec![
                    Mock::Empty,
                    Mock::Empty,
                    Mock::Empty,
                    Mock::Empty,
                    Mock::Empty,
                    Mock::Empty,
                    Mock::Empty,
                    Mock::Empty
                ],
                vec![
                    Mock::Empty,
                    Mock::Empty,
                    Mock::Empty,
                    Mock::Empty,
                    Mock::Empty,
                    Mock::Empty,
                    Mock::Empty,
                    Mock::Empty
                ],
                vec![
                    Mock::Empty,
                    Mock::Empty,
                    Mock::Empty,
                    Mock::Empty,
                    Mock::Empty,
                    Mock::Empty,
                    Mock::Empty,
                    Mock::Empty
                ],
                vec![
                    Mock::Empty,
                    Mock::Empty,
                    Mock::Empty,
                    Mock::Empty,
                    Mock::Empty,
                    Mock::Empty,
                    Mock::Empty,
                    Mock::Empty
                ],
                vec![
                    Mock::Empty,
                    Mock::Empty,
                    Mock::Empty,
                    Mock::Empty,
                    Mock::Empty,
                    Mock::Empty,
                    Mock::Empty,
                    Mock::Empty
                ],
                vec![
                    Mock::Empty,
                    Mock::Empty,
                    Mock::Empty,
                    Mock::Empty,
                    Mock::Empty,
                    Mock::Empty,
                    Mock::Empty,
                    Mock::Empty
                ],
                vec![
                    Mock::Empty,
                    Mock::Empty,
                    Mock::Empty,
                    Mock::Empty,
                    Mock::Empty,
                    Mock::Empty,
                    Mock::Empty,
                    Mock::Empty
                ],
            ],
            masu.all()
        )
    }
    use crate::masu::masu::Masu;

    use super::MasuBoard;
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    enum Mock {
        Empty,
        Yes,
        No,
    }
    impl Default for Mock {
        fn default() -> Self {
            Mock::Empty
        }
    }
}
