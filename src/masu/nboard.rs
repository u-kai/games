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
    ///pub fn get_down_right_line(&self, holizon: usize, valtical: usize) -> Vec<T> {
    ///let mut holizon = holizon;
    ///let mut valtical = valtical;
    ///let mut result = Vec::new();
    ///while self.get_down_right(holizon, valtical).is_ok() {
    ///result.push(self.get_down_right(holizon, valtical).unwrap());
    ///holizon += 1;
    ///valtical += 1;
    ///}
    ///result
    ///}
    pub fn get_up_right_line(&self, holizon: usize, valtical: usize) -> Vec<T> {
        let mut holizon = holizon;
        let mut valtical = valtical;
        let mut result = Vec::new();
        while self.get_up_right(holizon, valtical).is_ok() {
            result.push(self.get_up_right(holizon, valtical).unwrap());
            holizon += 1;
            valtical -= 1;
        }
        result
    }
    pub fn get_up_left_line(&self, holizon: usize, valtical: usize) -> Vec<T> {
        let mut holizon = holizon;
        let mut valtical = valtical;
        let mut result = Vec::new();
        while self.get_up_left(holizon, valtical).is_ok() {
            result.push(self.get_up_left(holizon, valtical).unwrap());
            holizon -= 1;
            valtical -= 1;
        }
        result
    }
    pub fn get_down_left_line(&self, holizon: usize, valtical: usize) -> Vec<T> {
        let mut holizon = holizon;
        let mut valtical = valtical;
        let mut result = Vec::new();
        while self.get_down_left(holizon, valtical).is_ok() {
            result.push(self.get_down_left(holizon, valtical).unwrap());
            holizon -= 1;
            valtical += 1;
        }
        result
    }
    pub fn get_left_line(&self, holizon: usize, valtical: usize) -> Vec<T> {
        let mut holizon = holizon;
        let mut result = Vec::new();
        while self.get_left(holizon, valtical).is_ok() {
            result.push(self.get_left(holizon, valtical).unwrap());
            holizon -= 1;
        }
        result
    }
    pub fn get_right_line(&self, holizon: usize, valtical: usize) -> Vec<T> {
        let mut holizon = holizon;
        let mut result = Vec::new();
        while self.get_right(holizon, valtical).is_ok() {
            result.push(self.get_right(holizon, valtical).unwrap());
            holizon += 1;
        }
        result
    }
    pub fn get_down_line(&self, holizon: usize, valtical: usize) -> Vec<T> {
        let mut valtical = valtical;
        let mut result = Vec::new();
        while self.get_down(holizon, valtical).is_ok() {
            result.push(self.get_down(holizon, valtical).unwrap());
            valtical += 1;
        }
        result
    }
    //pub fn get_up_line(&self, holizon: usize, valtical: usize) -> Vec<T> {
    //let mut valtical = valtical;
    //let mut result = Vec::new();
    //while self.get_up(holizon, valtical).is_ok() {
    //result.push(self.get_up(holizon, valtical).unwrap());
    //valtical -= 1;
    //}
    //result
    //}
    pub fn change(&mut self, holizon: usize, valtical: usize, koma: T) -> Result<(), String> {
        if holizon >= self.h_len || valtical >= self.v_len {
            return Err(format!("index out of bounds :[{}{}]", holizon, valtical));
        }
        self.masu[valtical][holizon] = koma;
        Ok(())
    }
    pub fn get(&self, holizon: usize, valtical: usize) -> Result<T, String> {
        if holizon >= self.h_len || valtical >= self.v_len {
            return Err(format!("index out of bounds :[{}{}]", holizon, valtical));
        }
        Ok(self.masu[valtical][holizon])
    }
    pub fn get_up(&self, holizon: usize, valtical: usize) -> Result<Masu<T>, String> {
        match IndexCalcurator::new(self.h_len, self.v_len, holizon, valtical).get_up() {
            Ok(new_index) => Ok(self.new_masu(new_index)),
            Err(e) => Err(e),
        }
    }
    pub fn get_down(&self, holizon: usize, valtical: usize) -> Result<T, String> {
        match IndexCalcurator::new(self.h_len, self.v_len, holizon, valtical).get_down() {
            Ok(new_index) => Ok(self.get(new_index.get_h(), new_index.get_v()).unwrap()),
            Err(e) => Err(e),
        }
    }
    pub fn get_right(&self, holizon: usize, valtical: usize) -> Result<T, String> {
        match IndexCalcurator::new(self.h_len, self.v_len, holizon, valtical).get_right() {
            Ok(new_index) => Ok(self.get(new_index.get_h(), new_index.get_v()).unwrap()),
            Err(e) => Err(e),
        }
    }
    pub fn get_left(&self, holizon: usize, valtical: usize) -> Result<T, String> {
        match IndexCalcurator::new(self.h_len, self.v_len, holizon, valtical).get_left() {
            Ok(new_index) => Ok(self.get(new_index.get_h(), new_index.get_v()).unwrap()),
            Err(e) => Err(e),
        }
    }
    pub fn get_up_left(&self, holizon: usize, valtical: usize) -> Result<T, String> {
        match IndexCalcurator::new(self.h_len, self.v_len, holizon, valtical).get_up_left() {
            Ok(new_index) => Ok(self.get(new_index.get_h(), new_index.get_v()).unwrap()),
            Err(e) => Err(e),
        }
    }
    pub fn get_down_left(&self, holizon: usize, valtical: usize) -> Result<T, String> {
        match IndexCalcurator::new(self.h_len, self.v_len, holizon, valtical).get_down_left() {
            Ok(new_index) => Ok(self.get(new_index.get_h(), new_index.get_v()).unwrap()),
            Err(e) => Err(e),
        }
    }
    pub fn get_up_right(&self, holizon: usize, valtical: usize) -> Result<T, String> {
        match IndexCalcurator::new(self.h_len, self.v_len, holizon, valtical).get_up_right() {
            Ok(new_index) => Ok(self.get(new_index.get_h(), new_index.get_v()).unwrap()),
            Err(e) => Err(e),
        }
    }
    pub fn get_down_right(&self, holizon: usize, valtical: usize) -> Result<Masu<T>, String> {
        match IndexCalcurator::new(self.h_len, self.v_len, holizon, valtical).get_down_right() {
            Ok(new_index) => Ok(self.new_masu(new_index)),
            Err(e) => Err(e),
        }
    }
    fn new_masu(&self, index: IndexCalcurator) -> Masu<T> {
        let koma = self.get(index.get_h(), index.get_v()).unwrap();
        Masu::new(koma, index.get_h(), index.get_v())
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
