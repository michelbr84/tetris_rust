use super::{Board, Piece};

pub struct Rules;

impl Rules {
    /// Scoring: 100/300/500/800 × (level + 1)
    pub fn score_for_lines(lines: u32, level: u32) -> u32 {
        let base = match lines {
            1 => 100,
            2 => 300,
            3 => 500,
            4 => 800,
            _ => 0,
        };
        base * (level + 1)
    }

    /// Simple wall kick: try offsets (0,0), (±1,0), (±2,0), (0,-1)
    pub fn try_wall_kick(board: &Board, piece: &Piece) -> Option<Piece> {
        let offsets = [
            (0, 0),
            (-1, 0),
            (1, 0),
            (-2, 0),
            (2, 0),
            (0, -1),
        ];

        for (dx, dy) in offsets {
            let kicked = piece.with_offset(dx, dy);
            if board.can_place(&kicked) {
                return Some(kicked);
            }
        }
        None
    }

    /// Gravity interval in seconds based on level
    pub fn gravity_interval(level: u32) -> f64 {
        let base_ms = 800.0;
        let min_ms = 80.0;
        let decay = 0.9_f64.powi(level as i32);
        (base_ms * decay).max(min_ms) / 1000.0
    }
}
