use super::tetromino::{Tetromino, TetrominoShape};

#[derive(Clone)]
pub struct Piece {
    pub x: i32,
    pub y: i32,
    pub shape: TetrominoShape,
    pub rotation: usize,
    pub color_id: u8,
}

impl Piece {
    pub fn new(shape: TetrominoShape) -> Self {
        Self {
            x: 3,
            y: 0,
            shape,
            rotation: 0,
            color_id: shape.color_id(),
        }
    }

    pub fn blocks(&self) -> Vec<(i32, i32)> {
        Tetromino::get(self.shape).rotations[self.rotation]
            .iter()
            .map(|(dx, dy)| (self.x + dx, self.y + dy))
            .collect()
    }

    pub fn rotated_cw(&self) -> Self {
        let mut clone = self.clone();
        clone.rotation = (clone.rotation + 1) % 4;
        clone
    }

    pub fn rotated_ccw(&self) -> Self {
        let mut clone = self.clone();
        clone.rotation = (clone.rotation + 3) % 4;
        clone
    }

    pub fn with_offset(&self, dx: i32, dy: i32) -> Self {
        let mut clone = self.clone();
        clone.x += dx;
        clone.y += dy;
        clone
    }
}
