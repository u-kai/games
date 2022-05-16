use std::{collections::btree_map::Values, fmt::Debug};

use super::index::MasuIndex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Masu<T>
where
    T: Debug + Clone + Copy + PartialEq + Eq,
{
    masu: Vec<Vec<T>>,
    h_len: usize,
    v_len: usize,
}

impl<T> Masu<T>
where
    T: Debug + Clone + Copy + PartialEq + Eq + Default,
{
    pub fn new(h_len: usize, v_len: usize) -> Self {
        Masu {
            h_len,
            v_len,
            masu: vec![vec![T::default(); h_len]; v_len],
        }
    }
    pub fn get_up_line(&self, holizon: usize, valtical: usize) -> Vec<T> {
        let mut valtical = valtical;
        let mut result = Vec::new();
        while valtical > 0 {
            result.push(self.get_up(holizon, valtical).unwrap());
            valtical -= 1;
            println!("{}", valtical);
        }
        result
    }
    pub fn change(&mut self, holizon: usize, valtical: usize, koma: T) {
        self.masu[valtical][holizon] = koma
    }
    pub fn get(&self, holizon: usize, valtical: usize) -> T {
        self.masu[valtical][holizon]
    }
    pub fn get_up(&self, holizon: usize, valtical: usize) -> Result<T, String> {
        match MasuIndex::new(self.h_len, self.v_len, holizon, valtical).get_up() {
            Ok(new_index) => Ok(self.get(new_index.get_h(), new_index.get_v())),
            Err(e) => Err(e),
        }
    }
    pub fn get_down(&self, holizon: usize, valtical: usize) -> Result<T, String> {
        match MasuIndex::new(self.h_len, self.v_len, holizon, valtical).get_down() {
            Ok(new_index) => Ok(self.get(new_index.get_h(), new_index.get_v())),
            Err(e) => Err(e),
        }
    }
    pub fn get_right(&self, holizon: usize, valtical: usize) -> Result<T, String> {
        match MasuIndex::new(self.h_len, self.v_len, holizon, valtical).get_right() {
            Ok(new_index) => Ok(self.get(new_index.get_h(), new_index.get_v())),
            Err(e) => Err(e),
        }
    }
    pub fn get_left(&self, holizon: usize, valtical: usize) -> Result<T, String> {
        match MasuIndex::new(self.h_len, self.v_len, holizon, valtical).get_left() {
            Ok(new_index) => Ok(self.get(new_index.get_h(), new_index.get_v())),
            Err(e) => Err(e),
        }
    }
    pub fn get_up_left(&self, holizon: usize, valtical: usize) -> Result<T, String> {
        match MasuIndex::new(self.h_len, self.v_len, holizon, valtical).get_up_left() {
            Ok(new_index) => Ok(self.get(new_index.get_h(), new_index.get_v())),
            Err(e) => Err(e),
        }
    }
    pub fn get_down_left(&self, holizon: usize, valtical: usize) -> Result<T, String> {
        match MasuIndex::new(self.h_len, self.v_len, holizon, valtical).get_down_left() {
            Ok(new_index) => Ok(self.get(new_index.get_h(), new_index.get_v())),
            Err(e) => Err(e),
        }
    }
    pub fn get_up_right(&self, holizon: usize, valtical: usize) -> Result<T, String> {
        match MasuIndex::new(self.h_len, self.v_len, holizon, valtical).get_up_right() {
            Ok(new_index) => Ok(self.get(new_index.get_h(), new_index.get_v())),
            Err(e) => Err(e),
        }
    }
    pub fn get_down_right(&self, holizon: usize, valtical: usize) -> Result<T, String> {
        match MasuIndex::new(self.h_len, self.v_len, holizon, valtical).get_down_right() {
            Ok(new_index) => Ok(self.get(new_index.get_h(), new_index.get_v())),
            Err(e) => Err(e),
        }
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
mod masu_test {
    use crate::osero::stone::OseroStone;

    use super::Masu;
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
    #[test]
    fn get_up_line_test() {
        //          |Y| | | | |
        //          |N| | | | |
        //          |Y| | | | |
        // Point->  |N| | | | |
        //          | | | | | |
        let mut masu: Masu<Mock> = Masu::new(5, 5);
        masu.change(0, 0, Mock::Yes);
        masu.change(0, 1, Mock::No);
        masu.change(0, 2, Mock::Yes);
        masu.change(0, 3, Mock::No);
        assert_eq!(masu.get_up_line(0, 3), vec![Mock::Yes, Mock::No, Mock::Yes]);

        //          |Y| | | | |
        //          |N| | | | |
        //          | | | | | |
        //          |N| | | | |
        // Point->  | | | | | |
        let mut masu: Masu<Mock> = Masu::new(5, 5);
        masu.change(0, 0, Mock::Yes);
        masu.change(0, 1, Mock::No);
        masu.change(0, 2, Mock::Empty);
        masu.change(0, 3, Mock::No);
        assert_eq!(
            masu.get_up_line(0, 4),
            vec![Mock::No, Mock::Empty, Mock::No, Mock::Yes]
        )
    }

    #[test]
    fn get_down_right_test() {
        let mut masu: Masu<OseroStone> = Masu::new(8, 8);
        masu.change(2, 2, OseroStone::Black);
        masu.change(1, 1, OseroStone::White);
        assert_eq!(masu.get_down_right(1, 1).unwrap(), OseroStone::Black);
        assert_eq!(masu.get_down_right(0, 0).unwrap(), OseroStone::White);
        assert_eq!(masu.get_down_right(4, 4).unwrap(), OseroStone::Empty);
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
    fn get_up_right_test() {
        let mut masu: Masu<OseroStone> = Masu::new(8, 8);
        masu.change(2, 2, OseroStone::Black);
        masu.change(1, 1, OseroStone::White);
        assert_eq!(masu.get_up_right(1, 3).unwrap(), OseroStone::Black);
        assert_eq!(masu.get_up_right(0, 2).unwrap(), OseroStone::White);
        assert_eq!(masu.get_up_right(2, 1).unwrap(), OseroStone::Empty);
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
        let mut masu: Masu<OseroStone> = Masu::new(8, 8);
        masu.change(2, 2, OseroStone::Black);
        masu.change(1, 1, OseroStone::White);
        assert_eq!(masu.get_down_left(3, 1).unwrap(), OseroStone::Black);
        assert_eq!(masu.get_down_left(2, 0).unwrap(), OseroStone::White);
        assert_eq!(masu.get_down_left(4, 4).unwrap(), OseroStone::Empty);
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
        let mut masu: Masu<OseroStone> = Masu::new(8, 8);
        masu.change(2, 2, OseroStone::Black);
        masu.change(1, 1, OseroStone::White);
        assert_eq!(masu.get_up_left(3, 3).unwrap(), OseroStone::Black);
        assert_eq!(masu.get_up_left(2, 2).unwrap(), OseroStone::White);
        assert_eq!(masu.get_up_left(2, 1).unwrap(), OseroStone::Empty);
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
    fn get_down_test() {
        let mut masu: Masu<OseroStone> = Masu::new(8, 8);
        masu.change(2, 2, OseroStone::Black);
        masu.change(1, 1, OseroStone::White);
        assert_eq!(masu.get_down(2, 1).unwrap(), OseroStone::Black);
        assert_eq!(masu.get_down(1, 0).unwrap(), OseroStone::White);
        assert_eq!(masu.get_down(4, 4).unwrap(), OseroStone::Empty);
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
        let mut masu: Masu<OseroStone> = Masu::new(8, 8);
        masu.change(2, 2, OseroStone::Black);
        masu.change(1, 1, OseroStone::White);
        assert_eq!(masu.get_up(2, 3).unwrap(), OseroStone::Black);
        assert_eq!(masu.get_up(1, 2).unwrap(), OseroStone::White);
        assert_eq!(masu.get_up(2, 1).unwrap(), OseroStone::Empty);
        assert_eq!(masu.get_up(0, 0), Err("[0,0] up is out bound".to_string()));
        assert_eq!(masu.get_up(7, 0), Err("[7,0] up is out bound".to_string()));
    }
    #[test]
    fn change_test() {
        let mut masu: Masu<OseroStone> = Masu::new(8, 8);
        masu.change(3, 5, OseroStone::White);
        assert_eq!(masu.get(3, 5), OseroStone::White);
    }

    #[test]
    fn new_test() {
        impl Masu<OseroStone> {
            pub fn all(&self) -> &Vec<Vec<OseroStone>> {
                &self.masu
            }
        }
        let masu: Masu<OseroStone> = Masu::new(8, 8);
        assert_eq!(
            &vec![
                vec![
                    OseroStone::Empty,
                    OseroStone::Empty,
                    OseroStone::Empty,
                    OseroStone::Empty,
                    OseroStone::Empty,
                    OseroStone::Empty,
                    OseroStone::Empty,
                    OseroStone::Empty
                ],
                vec![
                    OseroStone::Empty,
                    OseroStone::Empty,
                    OseroStone::Empty,
                    OseroStone::Empty,
                    OseroStone::Empty,
                    OseroStone::Empty,
                    OseroStone::Empty,
                    OseroStone::Empty
                ],
                vec![
                    OseroStone::Empty,
                    OseroStone::Empty,
                    OseroStone::Empty,
                    OseroStone::Empty,
                    OseroStone::Empty,
                    OseroStone::Empty,
                    OseroStone::Empty,
                    OseroStone::Empty
                ],
                vec![
                    OseroStone::Empty,
                    OseroStone::Empty,
                    OseroStone::Empty,
                    OseroStone::Empty,
                    OseroStone::Empty,
                    OseroStone::Empty,
                    OseroStone::Empty,
                    OseroStone::Empty
                ],
                vec![
                    OseroStone::Empty,
                    OseroStone::Empty,
                    OseroStone::Empty,
                    OseroStone::Empty,
                    OseroStone::Empty,
                    OseroStone::Empty,
                    OseroStone::Empty,
                    OseroStone::Empty
                ],
                vec![
                    OseroStone::Empty,
                    OseroStone::Empty,
                    OseroStone::Empty,
                    OseroStone::Empty,
                    OseroStone::Empty,
                    OseroStone::Empty,
                    OseroStone::Empty,
                    OseroStone::Empty
                ],
                vec![
                    OseroStone::Empty,
                    OseroStone::Empty,
                    OseroStone::Empty,
                    OseroStone::Empty,
                    OseroStone::Empty,
                    OseroStone::Empty,
                    OseroStone::Empty,
                    OseroStone::Empty
                ],
                vec![
                    OseroStone::Empty,
                    OseroStone::Empty,
                    OseroStone::Empty,
                    OseroStone::Empty,
                    OseroStone::Empty,
                    OseroStone::Empty,
                    OseroStone::Empty,
                    OseroStone::Empty
                ],
            ],
            masu.all()
        )
    }
}
