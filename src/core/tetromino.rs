#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TetrominoShape {
    I, O, T, L, J, S, Z,
}

impl TetrominoShape {
    pub fn color_id(&self) -> u8 {
        match self {
            TetrominoShape::I => 1,
            TetrominoShape::O => 2,
            TetrominoShape::T => 3,
            TetrominoShape::L => 4,
            TetrominoShape::J => 5,
            TetrominoShape::S => 6,
            TetrominoShape::Z => 7,
        }
    }
}

pub struct Tetromino {
    pub rotations: [[(i32, i32); 4]; 4],
}

impl Tetromino {
    pub fn get(shape: TetrominoShape) -> &'static Tetromino {
        match shape {
            TetrominoShape::I => &TETROMINO_I,
            TetrominoShape::O => &TETROMINO_O,
            TetrominoShape::T => &TETROMINO_T,
            TetrominoShape::L => &TETROMINO_L,
            TetrominoShape::J => &TETROMINO_J,
            TetrominoShape::S => &TETROMINO_S,
            TetrominoShape::Z => &TETROMINO_Z,
        }
    }
}

static TETROMINO_I: Tetromino = Tetromino {
    rotations: [
        [(0, 0), (1, 0), (2, 0), (3, 0)],
        [(2, -1), (2, 0), (2, 1), (2, 2)],
        [(0, 1), (1, 1), (2, 1), (3, 1)],
        [(1, -1), (1, 0), (1, 1), (1, 2)],
    ],
};

static TETROMINO_O: Tetromino = Tetromino {
    rotations: [
        [(0, 0), (1, 0), (0, 1), (1, 1)],
        [(0, 0), (1, 0), (0, 1), (1, 1)],
        [(0, 0), (1, 0), (0, 1), (1, 1)],
        [(0, 0), (1, 0), (0, 1), (1, 1)],
    ],
};

static TETROMINO_T: Tetromino = Tetromino {
    rotations: [
        [(1, 0), (0, 1), (1, 1), (2, 1)],
        [(1, 0), (1, 1), (2, 1), (1, 2)],
        [(0, 1), (1, 1), (2, 1), (1, 2)],
        [(1, 0), (0, 1), (1, 1), (1, 2)],
    ],
};

static TETROMINO_L: Tetromino = Tetromino {
    rotations: [
        [(0, 0), (0, 1), (1, 1), (2, 1)],
        [(1, 0), (1, 1), (1, 2), (2, 0)],
        [(0, 1), (1, 1), (2, 1), (2, 2)],
        [(1, 0), (0, 2), (1, 1), (1, 2)],
    ],
};

static TETROMINO_J: Tetromino = Tetromino {
    rotations: [
        [(2, 0), (0, 1), (1, 1), (2, 1)],
        [(1, 0), (1, 1), (1, 2), (2, 2)],
        [(0, 1), (1, 1), (2, 1), (0, 2)],
        [(0, 0), (1, 0), (1, 1), (1, 2)],
    ],
};

static TETROMINO_S: Tetromino = Tetromino {
    rotations: [
        [(1, 0), (2, 0), (0, 1), (1, 1)],
        [(1, 0), (1, 1), (2, 1), (2, 2)],
        [(1, 1), (2, 1), (0, 2), (1, 2)],
        [(0, 0), (0, 1), (1, 1), (1, 2)],
    ],
};

static TETROMINO_Z: Tetromino = Tetromino {
    rotations: [
        [(0, 0), (1, 0), (1, 1), (2, 1)],
        [(2, 0), (1, 1), (2, 1), (1, 2)],
        [(0, 1), (1, 1), (1, 2), (2, 2)],
        [(1, 0), (0, 1), (1, 1), (0, 2)],
    ],
};
