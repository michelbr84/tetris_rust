use super::piece::Piece;

pub const BOARD_WIDTH: usize = 10;
pub const BOARD_HEIGHT: usize = 20;

pub struct Board {
    pub grid: [[u8; BOARD_WIDTH]; BOARD_HEIGHT],
}

impl Board {
    pub fn new() -> Self {
        Self {
            grid: [[0; BOARD_WIDTH]; BOARD_HEIGHT],
        }
    }

    pub fn can_place(&self, piece: &Piece) -> bool {
        self.can_move(piece, 0, 0)
    }

    pub fn can_move(&self, piece: &Piece, dx: i32, dy: i32) -> bool {
        for (x, y) in piece.blocks() {
            let nx = x + dx;
            let ny = y + dy;

            if nx < 0 || nx >= BOARD_WIDTH as i32 || ny >= BOARD_HEIGHT as i32 {
                return false;
            }
            if ny >= 0 && self.grid[ny as usize][nx as usize] != 0 {
                return false;
            }
        }
        true
    }

    pub fn lock_piece(&mut self, piece: &Piece) {
        for (x, y) in piece.blocks() {
            if y >= 0 && y < BOARD_HEIGHT as i32 && x >= 0 && x < BOARD_WIDTH as i32 {
                self.grid[y as usize][x as usize] = piece.color_id;
            }
        }
    }

    pub fn clear_lines(&mut self) -> u32 {
        let mut cleared = 0;
        let mut new_grid = [[0u8; BOARD_WIDTH]; BOARD_HEIGHT];
        let mut write_row = BOARD_HEIGHT - 1;

        for read_row in (0..BOARD_HEIGHT).rev() {
            if self.grid[read_row].iter().all(|&c| c != 0) {
                cleared += 1;
            } else {
                new_grid[write_row] = self.grid[read_row];
                if write_row > 0 {
                    write_row -= 1;
                }
            }
        }

        self.grid = new_grid;
        cleared
    }

    pub fn drop_distance(&self, piece: &Piece) -> usize {
        let mut distance = 0;
        while self.can_move(piece, 0, (distance + 1) as i32) {
            distance += 1;
        }
        distance
    }
}
