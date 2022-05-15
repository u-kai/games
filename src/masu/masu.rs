use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Masu<T>(Vec<Vec<T>>)
where
    T: Debug + Clone + Copy + PartialEq + Eq;

impl<T> Masu<T>
where
    T: Debug + Clone + Copy + PartialEq + Eq + Default,
{
    pub fn new(h_len: usize, v_len: usize) -> Self {
        Masu(vec![vec![T::default(); h_len]; v_len])
    }
    pub fn all(&self) -> &Vec<Vec<T>> {
        &self.0
    }
    pub fn change(&mut self, holizon: usize, valtical: usize, koma: T) {
        self.0[valtical][holizon] = koma
    }
    pub fn get(&self, holizon: usize, valtical: usize) -> T {
        self.0[valtical][holizon]
    }
    pub fn print(&self) {
        let mut log = String::new();
        for v in &self.0 {
            //log = format!("{}\n{}\n", log, "--".repeat(self.0.len()));
            for h in v {
                log = format!("{}|{:?}", log, h);
            }
            log = format!("{}|\n", log)
        }
        println!("{}", log);
    }
    pub fn is_edge(&self, holizon: usize, valtical: usize) -> bool {
        valtical == (self.0.len() - 1) || holizon == (self.0[0].len() - 1)
    }
    pub fn is_out_bound(&self, holizon: usize, valtical: usize) -> bool {
        valtical >= self.0.len() || holizon >= self.0[0].len()
    }
    pub fn get_up(&self, holizon: usize, valtical: usize) -> Result<T, String> {
        if valtical == 0 {
            return Err(format!("[{},{}] up is out bound", holizon, valtical));
        }
        let new_h = holizon;
        let new_v = valtical - 1;
        if self.is_out_bound(new_h, new_v) {
            return Err(format!("[{},{}] up is out bound", holizon, valtical));
        }
        Ok(self.get(new_h, new_v))
    }
}

#[cfg(test)]
mod masu_test {
    use crate::osero::stone::OseroStone;

    use super::Masu;
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
