use rand::seq::SliceRandom;
use super::tetromino::TetrominoShape;

pub struct BagRandomizer {
    bag: Vec<TetrominoShape>,
}

impl BagRandomizer {
    pub fn new() -> Self {
        let mut rng = Self { bag: Vec::new() };
        rng.refill();
        rng
    }

    fn refill(&mut self) {
        use TetrominoShape::*;
        let mut pieces = vec![I, O, T, L, J, S, Z];
        pieces.shuffle(&mut rand::thread_rng());
        self.bag = pieces;
    }

    pub fn next(&mut self) -> TetrominoShape {
        if self.bag.is_empty() {
            self.refill();
        }
        self.bag.pop().unwrap()
    }
}
