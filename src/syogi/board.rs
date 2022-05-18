use crate::masu::nboard::MasuBoard;

use super::koma::{Koma, SyogiKoma};

pub trait Syogi<T> {
    fn move_to(
        &mut self,
        from_h: usize,
        from_v: usize,
        to_h: usize,
        to_v: usize,
    ) -> Result<Option<T>, String>;
    fn rev(&mut self, holizon: usize, valtical: usize) -> Result<(), String>;
    fn put(&mut self, holizon: usize, valtical: usize) -> Result<(), String>;
}

pub struct SyogiBoard {
    masu: MasuBoard<Koma>,
}
impl SyogiBoard {
    fn get(&mut self, holizon: usize, valtical: usize) -> Result<Koma, String> {
        self.masu.get(holizon, valtical)
    }
    fn is_movable(&self, holizon: usize, valtical: usize) -> bool {
        true
    }
    fn is_empty(&self, holizon: usize, valtical: usize) -> bool {
        match self.masu.get(holizon, valtical) {
            Ok(koma) => match koma {
                Koma::Empty => true,
                _ => false,
            },
            Err(_) => false,
        }
    }
}

impl Syogi<Koma> for SyogiBoard {
    fn move_to(
        &mut self,
        from_h: usize,
        from_v: usize,
        to_h: usize,
        to_v: usize,
    ) -> Result<Option<Koma>, String> {
        let koma = self.get(from_h, from_v);
        match koma {
            Ok(koma) => {
                let paths = koma.movable_paths(from_h, from_v);
                Ok(None)
            }
            Err(e) => Err(e),
        }
    }
    fn rev(&mut self, holizon: usize, valtical: usize) -> Result<(), String> {
        Ok(())
    }
    fn put(&mut self, holizon: usize, valtical: usize) -> Result<(), String> {
        Ok(())
    }
}
