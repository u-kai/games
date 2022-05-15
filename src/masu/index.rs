#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MasuIndex {
    h_len: usize,
    v_len: usize,
    h_index: usize,
    v_index: usize,
}

impl MasuIndex {
    pub fn new(h_len: usize, v_len: usize, h_index: usize, v_index: usize) -> Self {
        if h_index >= h_len || v_index >= v_len {
            panic!("index is out bound!!!")
        }
        MasuIndex {
            h_len,
            v_len,
            v_index,
            h_index,
        }
    }
    pub fn get_v(&self) -> usize {
        self.v_index
    }
    pub fn get_h(&self) -> usize {
        self.h_index
    }
    pub fn get_up(&self) -> Result<MasuIndex, String> {
        if self.is_up_edge() {
            return Err(self.err_msg("up"));
        }
        let index = self.create(self.h_index, self.v_index - 1);
        Ok(index)
    }
    pub fn get_down(&self) -> Result<MasuIndex, String> {
        if self.is_down_edge() {
            return Err(self.err_msg("down"));
        }
        let index = self.create(self.h_index, self.v_index + 1);
        Ok(index)
    }
    pub fn get_left(&self) -> Result<MasuIndex, String> {
        if self.is_left_edge() {
            return Err(self.err_msg("left"));
        }
        let index = self.create(self.h_index - 1, self.v_index);
        Ok(index)
    }
    pub fn get_right(&self) -> Result<MasuIndex, String> {
        if self.is_right_edge() {
            return Err(self.err_msg("right"));
        }
        let index = self.create(self.h_index + 1, self.v_index);
        Ok(index)
    }
    pub fn get_up_right(&self) -> Result<MasuIndex, String> {
        match self.get_up() {
            Ok(index) => {
                let right_index = index.get_right();
                right_index
            }
            Err(e) => Err(e),
        }
    }
    pub fn get_up_left(&self) -> Result<MasuIndex, String> {
        match self.get_up() {
            Ok(index) => {
                let left_index = index.get_left();
                left_index
            }
            Err(e) => Err(e),
        }
    }
    pub fn get_down_right(&self) -> Result<MasuIndex, String> {
        match self.get_down() {
            Ok(index) => {
                let right_index = index.get_right();
                right_index
            }
            Err(e) => Err(e),
        }
    }
    pub fn get_down_left(&self) -> Result<MasuIndex, String> {
        match self.get_down() {
            Ok(index) => {
                let left_index = index.get_left();
                left_index
            }
            Err(e) => Err(e),
        }
    }
    fn create(&self, holizon: usize, valtical: usize) -> MasuIndex {
        MasuIndex::new(self.h_len, self.v_len, holizon, valtical)
    }
    fn is_up_edge(&self) -> bool {
        self.v_index == 0
    }
    fn is_down_edge(&self) -> bool {
        self.v_index == (self.v_len - 1)
    }
    fn is_left_edge(&self) -> bool {
        self.h_index == 0
    }
    fn is_right_edge(&self) -> bool {
        self.h_index == (self.v_len - 1)
    }
    fn err_msg(&self, posi: &str) -> String {
        format!("[{},{}] {} is out bound", self.h_index, self.v_index, posi)
    }
}

#[cfg(test)]
mod index_test {
    use crate::masu::index::MasuIndex;
    #[test]
    fn get_down_left_test() {
        assert_eq!(
            MasuIndex::new(8, 8, 2, 1).get_down_left().unwrap(),
            MasuIndex::new(8, 8, 1, 2)
        );
        assert_eq!(
            MasuIndex::new(8, 8, 3, 2).get_down_left().unwrap(),
            MasuIndex::new(8, 8, 2, 3)
        );
        assert_eq!(
            MasuIndex::new(8, 8, 6, 4).get_down_left().unwrap(),
            MasuIndex::new(8, 8, 5, 5)
        );
        assert_eq!(
            MasuIndex::new(8, 8, 0, 6).get_down_left(),
            Err("[0,7] left is out bound".to_string())
        );
        assert_eq!(
            MasuIndex::new(8, 8, 1, 7).get_down_left(),
            Err("[1,7] down is out bound".to_string())
        );
    }
    #[test]
    fn get_down_right_test() {
        assert_eq!(
            MasuIndex::new(8, 8, 2, 1).get_down_right().unwrap(),
            MasuIndex::new(8, 8, 3, 2)
        );
        assert_eq!(
            MasuIndex::new(8, 8, 3, 2).get_down_right().unwrap(),
            MasuIndex::new(8, 8, 4, 3)
        );
        assert_eq!(
            MasuIndex::new(8, 8, 6, 4).get_down_right().unwrap(),
            MasuIndex::new(8, 8, 7, 5)
        );
        assert_eq!(
            MasuIndex::new(8, 8, 7, 5).get_down_right(),
            Err("[7,6] right is out bound".to_string())
        );
        assert_eq!(
            MasuIndex::new(8, 8, 6, 7).get_down_right(),
            Err("[6,7] down is out bound".to_string())
        );
    }
    #[test]
    fn get_up_left_test() {
        assert_eq!(
            MasuIndex::new(8, 8, 2, 1).get_up_left().unwrap(),
            MasuIndex::new(8, 8, 1, 0)
        );
        assert_eq!(
            MasuIndex::new(8, 8, 3, 2).get_up_left().unwrap(),
            MasuIndex::new(8, 8, 2, 1)
        );
        assert_eq!(
            MasuIndex::new(8, 8, 6, 4).get_up_left().unwrap(),
            MasuIndex::new(8, 8, 5, 3)
        );
        assert_eq!(
            MasuIndex::new(8, 8, 0, 6).get_up_left(),
            Err("[0,5] left is out bound".to_string())
        );
        assert_eq!(
            MasuIndex::new(8, 8, 1, 0).get_up_left(),
            Err("[1,0] up is out bound".to_string())
        );
    }
    #[test]
    fn get_up_right_test() {
        assert_eq!(
            MasuIndex::new(8, 8, 2, 1).get_up_right().unwrap(),
            MasuIndex::new(8, 8, 3, 0)
        );
        assert_eq!(
            MasuIndex::new(8, 8, 3, 2).get_up_right().unwrap(),
            MasuIndex::new(8, 8, 4, 1)
        );
        assert_eq!(
            MasuIndex::new(8, 8, 6, 4).get_up_right().unwrap(),
            MasuIndex::new(8, 8, 7, 3)
        );
        assert_eq!(
            MasuIndex::new(8, 8, 7, 7).get_up_right(),
            Err("[7,6] right is out bound".to_string())
        );
        assert_eq!(
            MasuIndex::new(8, 8, 6, 0).get_up_right(),
            Err("[6,0] up is out bound".to_string())
        );
    }
    #[test]
    fn get_right_test() {
        assert_eq!(
            MasuIndex::new(8, 8, 2, 1).get_right().unwrap(),
            MasuIndex::new(8, 8, 3, 1)
        );
        assert_eq!(
            MasuIndex::new(8, 8, 3, 2).get_right().unwrap(),
            MasuIndex::new(8, 8, 4, 2)
        );
        assert_eq!(
            MasuIndex::new(8, 8, 6, 4).get_right().unwrap(),
            MasuIndex::new(8, 8, 7, 4)
        );
        assert_eq!(
            MasuIndex::new(8, 8, 7, 7).get_right(),
            Err("[7,7] right is out bound".to_string())
        );
    }
    #[test]
    fn get_left_test() {
        assert_eq!(
            MasuIndex::new(8, 8, 2, 1).get_left().unwrap(),
            MasuIndex::new(8, 8, 1, 1)
        );
        assert_eq!(
            MasuIndex::new(8, 8, 3, 2).get_left().unwrap(),
            MasuIndex::new(8, 8, 2, 2)
        );
        assert_eq!(
            MasuIndex::new(8, 8, 7, 4).get_left().unwrap(),
            MasuIndex::new(8, 8, 6, 4)
        );
        assert_eq!(
            MasuIndex::new(8, 8, 0, 7).get_left(),
            Err("[0,7] left is out bound".to_string())
        );
    }

    #[test]
    fn get_down_test() {
        assert_eq!(
            MasuIndex::new(8, 8, 2, 1).get_down().unwrap(),
            MasuIndex::new(8, 8, 2, 2)
        );
        assert_eq!(
            MasuIndex::new(8, 8, 3, 2).get_down().unwrap(),
            MasuIndex::new(8, 8, 3, 3)
        );
        assert_eq!(
            MasuIndex::new(8, 8, 7, 4).get_down().unwrap(),
            MasuIndex::new(8, 8, 7, 5)
        );
        assert_eq!(
            MasuIndex::new(8, 8, 2, 7).get_down(),
            Err("[2,7] down is out bound".to_string())
        );
    }

    #[test]
    fn get_up_test() {
        assert_eq!(
            MasuIndex::new(8, 8, 2, 1).get_up().unwrap(),
            MasuIndex::new(8, 8, 2, 0)
        );
        assert_eq!(
            MasuIndex::new(8, 8, 3, 2).get_up().unwrap(),
            MasuIndex::new(8, 8, 3, 1)
        );
        assert_eq!(
            MasuIndex::new(8, 8, 7, 4).get_up().unwrap(),
            MasuIndex::new(8, 8, 7, 3)
        );
        assert_eq!(
            MasuIndex::new(8, 8, 2, 0).get_up(),
            Err("[2,0] up is out bound".to_string())
        );
    }
}
