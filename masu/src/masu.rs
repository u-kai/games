use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Masu<T>
where
    T: Debug + Default + Clone + PartialEq + Eq,
{
    koma: T,
    h_index: usize,
    v_index: usize,
}
impl<T> Masu<T>
where
    T: Debug + Default + Clone + PartialEq + Eq,
{
    pub fn new(koma: T, h_index: usize, v_index: usize) -> Self {
        Masu {
            koma,
            h_index,
            v_index,
        }
    }
    pub fn change(&mut self, koma: T) {
        self.koma = koma
    }
    pub fn get_h_index(&self) -> usize {
        self.h_index
    }
    pub fn get_v_index(&self) -> usize {
        self.v_index
    }
}
