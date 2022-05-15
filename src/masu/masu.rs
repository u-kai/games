use std::fmt::Debug;

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
    pub fn all(&self) -> &Vec<Vec<T>> {
        &self.masu
    }
    pub fn change(&mut self, holizon: usize, valtical: usize, koma: T) {
        self.masu[valtical][holizon] = koma
    }
    pub fn get(&self, holizon: usize, valtical: usize) -> T {
        self.masu[valtical][holizon]
    }
    pub fn is_edge(&self, holizon: usize, valtical: usize) -> bool {
        valtical == (self.v_len - 1) || holizon == (self.h_len - 1) || holizon == 0 || valtical == 0
    }
    pub fn is_out_bound(&self, holizon: usize, valtical: usize) -> bool {
        valtical >= self.v_len || holizon >= self.h_len
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
    fn is_out_bound_test() {
        let masu: Masu<OseroStone> = Masu::new(8, 8);
        assert_eq!(masu.is_out_bound(8, 0), true);
        assert_eq!(masu.is_out_bound(1, 9), true);
        assert_eq!(masu.is_out_bound(7, 7), false);
    }
    #[test]
    fn is_edge_test() {
        let masu: Masu<OseroStone> = Masu::new(8, 8);
        assert_eq!(masu.is_edge(3, 5), false);
        assert_eq!(masu.is_edge(7, 0), true);
        assert_eq!(masu.is_edge(3, 7), true);
    }
    #[test]
    fn change_test() {
        let mut masu: Masu<OseroStone> = Masu::new(8, 8);
        masu.change(3, 5, OseroStone::White);
        assert_eq!(masu.get(3, 5), OseroStone::White);
    }

    #[test]
    fn new_test() {
        let masu: Masu<OseroStone> = Masu::new(8, 8);
        masu.print();
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
