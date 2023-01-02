use masu::{calcurator::IndexCalcurator, nboard::MasuBoard};

use super::{
    koma::{create_index, Koma, RL},
    komas::{gin::Gin, keima::Keima, kin::Kin, kyosya::Kyosya, ohsyo::Ohsyo},
};

pub trait Syogi<T> {
    fn move_to(
        &mut self,
        from_h: usize,
        from_v: usize,
        to_h: usize,
        to_v: usize,
    ) -> Result<T, String>;
    fn rev(&mut self, holizon: usize, valtical: usize) -> Result<(), String>;
    fn put(&mut self, holizon: usize, valtical: usize) -> Result<(), String>;
}

pub struct SyogiBoard {
    masu: MasuBoard<Koma>,
}
impl SyogiBoard {
    pub fn new() -> Self {
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | | | | | | | | | |
        // | || | | | | | | |<-(1,7)
        // | | | | | | | | | |
        let mut masu = MasuBoard::new(9, 9);
        masu.change(0, 0, Koma::Kyosya(Kyosya::new(RL::Right)))
            .unwrap();
        masu.change(1, 0, Koma::Keima(Keima::new(RL::Right)))
            .unwrap();
        masu.change(2, 0, Koma::Gin(Gin::new(RL::Right))).unwrap();
        masu.change(3, 0, Koma::Kin(Kin::new(RL::Right))).unwrap();
        masu.change(4, 0, Koma::Ohsyo(Ohsyo::new(RL::Right)))
            .unwrap();
        masu.change(5, 0, Koma::Kin(Kin::new(RL::Right))).unwrap();
        masu.change(6, 0, Koma::Gin(Gin::new(RL::Right))).unwrap();
        masu.change(7, 0, Koma::Keima(Keima::new(RL::Right)))
            .unwrap();
        masu.change(8, 0, Koma::Kyosya(Kyosya::new(RL::Right)))
            .unwrap();
        SyogiBoard { masu }
    }
    fn get(&self, holizon: usize, valtical: usize) -> Result<Koma, String> {
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
    fn index_to_koma(&self, index: IndexCalcurator) -> Koma {
        self.get(index.get_h(), index.get_v()).unwrap()
    }
    pub fn print(&self) -> () {
        self.masu.print()
    }
}

impl Syogi<Koma> for SyogiBoard {
    fn move_to(
        &mut self,
        from_h: usize,
        from_v: usize,
        to_h: usize,
        to_v: usize,
    ) -> Result<Koma, String> {
        let koma = self.get(from_h, from_v);
        let dist = create_index(to_h, to_v);
        match koma {
            Ok(koma) => {
                Ok(koma)
                //let paths = koma.movable_paths(from_h, from_v);
                //let containes_paths = paths.iter().filter(|paths| paths.contains(&dist)).next();
                //if let Some(containes_paths) = containes_paths {
                //let dist_position = containes_paths
                //.iter()
                //.position(|index| index == &dist)
                //.unwrap();
                //let path = containes_paths.iter().take(dist_position + 1);
                //let last = path.len() - 1;
                //if path
                //.take(last)
                //.map(|index| self.index_to_koma(index.clone()))
                //.all(|koma| koma == Koma::Empty)
                //{
                //let index = path.last();
                //if let Some(index) = index {
                //let koma = self.index_to_koma(index.clone());
                //return Ok(koma);
                //}
                //}
                //return Err(format!("d"));
                //} else {
                //Err(format!(""))
                //}
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

//impl Debug for SyogiBoard {
//fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//write!(f, self.print())
//}
//}
