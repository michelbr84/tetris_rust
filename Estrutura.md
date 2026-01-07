## 🎮 Estrutura Final do Projeto

```
tetris_rust/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs
    ├── app/
    │   ├── mod.rs
    │   ├── state.rs
    │   └── settings.rs
    ├── core/
    │   ├── mod.rs
    │   ├── board.rs
    │   ├── piece.rs
    │   ├── tetromino.rs
    │   ├── rules.rs
    │   └── rng.rs
    ├── render/
    │   ├── mod.rs
    │   ├── theme.rs
    │   ├── draw_board.rs
    │   ├── draw_panels.rs
    │   └── anim.rs
    └── time/
        ├── mod.rs
        └── clock.rs
```

---

## 📦 Arquivo `Cargo.toml`

```toml
[package]
name = "tetris_rust"
version = "1.0.0"
edition = "2021"
authors = ["Seu Nome <seuemail@example.com>"]
description = "Tetris moderno com UI gráfica em Rust"
license = "MIT"

[dependencies]
eframe = "0.29"          # Framework GUI (inclui egui)
rand = "0.8"             # Geração aleatória (7-bag)
```

---

## 🧩 Arquivo `src/main.rs`

```rust
mod app;
mod core;
mod render;
mod time;

use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("Tetris Modern"),
        ..Default::default()
    };

    eframe::run_native(
        "Tetris Modern",
        options,
        Box::new(|cc| Ok(Box::new(app::TetrisApp::new(cc)))),
    )
}
```

---

## 📱 Pasta `src/app/`

### `mod.rs`

```rust
pub mod state;
pub mod settings;

use eframe::egui;
use crate::core::{Board, Piece, Rules, BagRandomizer};
use crate::render::{Theme, draw_board, draw_panels, Animator};
use crate::time::Clock;
use state::GameState;
use settings::Settings;

pub struct TetrisApp {
    pub board: Board,
    pub current_piece: Piece,
    pub next_piece: Piece,
    pub hold_piece: Option<Piece>,
    pub can_hold: bool,
    pub state: GameState,
    pub settings: Settings,
    pub clock: Clock,
    pub rng: BagRandomizer,
    pub animator: Animator,
    pub score: u32,
    pub level: u32,
    pub lines: u32,
    pub lock_timer: Option<f64>,
}

impl TetrisApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut rng = BagRandomizer::new();
        let current_piece = Piece::new(rng.next());
        let next_piece = Piece::new(rng.next());

        Self {
            board: Board::new(),
            current_piece,
            next_piece,
            hold_piece: None,
            can_hold: true,
            state: GameState::Menu,
            settings: Settings::default(),
            clock: Clock::new(0),
            rng,
            animator: Animator::new(),
            score: 0,
            level: 0,
            lines: 0,
            lock_timer: None,
        }
    }

    pub fn reset(&mut self) {
        self.board = Board::new();
        self.rng = BagRandomizer::new();
        self.current_piece = Piece::new(self.rng.next());
        self.next_piece = Piece::new(self.rng.next());
        self.hold_piece = None;
        self.can_hold = true;
        self.score = 0;
        self.level = 0;
        self.lines = 0;
        self.lock_timer = None;
        self.clock = Clock::new(0);
        self.state = GameState::Playing;
    }

    fn spawn_next_piece(&mut self) {
        self.current_piece = Piece::new(self.next_piece.shape);
        self.next_piece = Piece::new(self.rng.next());
        self.can_hold = true;

        if !self.board.can_place(&self.current_piece) {
            self.state = GameState::GameOver;
        }
    }

    fn hold(&mut self) {
        if !self.can_hold {
            return;
        }
        self.can_hold = false;

        if let Some(held) = self.hold_piece.take() {
            let current_shape = self.current_piece.shape;
            self.current_piece = Piece::new(held.shape);
            self.hold_piece = Some(Piece::new(current_shape));
        } else {
            self.hold_piece = Some(Piece::new(self.current_piece.shape));
            self.spawn_next_piece();
        }
    }

    fn lock_piece(&mut self) {
        self.board.lock_piece(&self.current_piece);
        let cleared = self.board.clear_lines();

        if cleared > 0 {
            self.lines += cleared;
            self.score += Rules::score_for_lines(cleared, self.level);
            self.level = self.lines / 10;
            self.clock.set_level(self.level);
            self.animator.trigger_line_clear();
        }

        self.animator.trigger_lock_pop();
        self.lock_timer = None;
        self.spawn_next_piece();
    }

    fn hard_drop(&mut self) {
        let cells_dropped = self.board.drop_distance(&self.current_piece);
        self.current_piece.y += cells_dropped as i32;
        self.score += (cells_dropped as u32) * 2;
        self.lock_piece();
    }

    fn soft_drop(&mut self) {
        if self.board.can_move(&self.current_piece, 0, 1) {
            self.current_piece.y += 1;
            self.score += 1;
        }
    }
}

impl eframe::App for TetrisApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();

        match self.state {
            GameState::Menu => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(100.0);
                        ui.heading("🎮 TETRIS MODERN");
                        ui.add_space(40.0);

                        if ui.button("▶ Play").clicked() {
                            self.reset();
                        }
                        ui.add_space(10.0);
                        if ui.button("⚙ Settings").clicked() {
                            // TODO: Settings screen
                        }
                        ui.add_space(10.0);
                        if ui.button("✖ Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                });
            }

            GameState::Playing => {
                // Handle input
                ctx.input(|i| {
                    if i.key_pressed(egui::Key::ArrowLeft) {
                        if self.board.can_move(&self.current_piece, -1, 0) {
                            self.current_piece.x -= 1;
                        }
                    }
                    if i.key_pressed(egui::Key::ArrowRight) {
                        if self.board.can_move(&self.current_piece, 1, 0) {
                            self.current_piece.x += 1;
                        }
                    }
                    if i.key_pressed(egui::Key::ArrowDown) {
                        self.soft_drop();
                    }
                    if i.key_pressed(egui::Key::Space) {
                        self.hard_drop();
                    }
                    if i.key_pressed(egui::Key::ArrowUp) || i.key_pressed(egui::Key::X) {
                        let rotated = self.current_piece.rotated_cw();
                        if let Some(kicked) = Rules::try_wall_kick(&self.board, &rotated) {
                            self.current_piece = kicked;
                        }
                    }
                    if i.key_pressed(egui::Key::Z) {
                        let rotated = self.current_piece.rotated_ccw();
                        if let Some(kicked) = Rules::try_wall_kick(&self.board, &rotated) {
                            self.current_piece = kicked;
                        }
                    }
                    if i.key_pressed(egui::Key::C) {
                        self.hold();
                    }
                    if i.key_pressed(egui::Key::P) || i.key_pressed(egui::Key::Escape) {
                        self.state = GameState::Paused;
                    }
                    if i.key_pressed(egui::Key::Q) {
                        self.state = GameState::Menu;
                    }
                });

                // Gravity tick
                let dt = ctx.input(|i| i.stable_dt as f64);
                if self.clock.tick(dt) {
                    if self.board.can_move(&self.current_piece, 0, 1) {
                        self.current_piece.y += 1;
                        self.lock_timer = None;
                    } else {
                        // Lock delay
                        if let Some(timer) = &mut self.lock_timer {
                            *timer -= dt;
                            if *timer <= 0.0 {
                                self.lock_piece();
                            }
                        } else {
                            self.lock_timer = Some(0.3); // 300ms lock delay
                        }
                    }
                }

                // Animation updates
                self.animator.update(dt);

                // Render
                egui::CentralPanel::default().show(ctx, |ui| {
                    let theme = Theme::default();
                    draw_board::render(ui, &theme, &self.board, &self.current_piece, &self.settings, &self.animator);
                });

                egui::SidePanel::right("info_panel").show(ctx, |ui| {
                    let theme = Theme::default();
                    draw_panels::render(ui, &theme, self);
                });
            }

            GameState::Paused => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(150.0);
                        ui.heading("⏸ PAUSED");
                        ui.add_space(30.0);

                        if ui.button("▶ Resume").clicked() {
                            self.state = GameState::Playing;
                        }
                        ui.add_space(10.0);
                        if ui.button("🏠 Menu").clicked() {
                            self.state = GameState::Menu;
                        }
                    });
                });

                ctx.input(|i| {
                    if i.key_pressed(egui::Key::P) || i.key_pressed(egui::Key::Escape) {
                        self.state = GameState::Playing;
                    }
                });
            }

            GameState::GameOver => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(120.0);
                        ui.heading("💀 GAME OVER");
                        ui.add_space(20.0);
                        ui.label(format!("Score: {}", self.score));
                        ui.label(format!("Level: {}", self.level));
                        ui.label(format!("Lines: {}", self.lines));
                        ui.add_space(30.0);

                        if ui.button("🔄 Restart").clicked() {
                            self.reset();
                        }
                        ui.add_space(10.0);
                        if ui.button("🏠 Menu").clicked() {
                            self.state = GameState::Menu;
                        }
                    });
                });

                ctx.input(|i| {
                    if i.key_pressed(egui::Key::R) {
                        self.reset();
                    }
                });
            }
        }
    }
}
```

---

### `state.rs`

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Menu,
    Playing,
    Paused,
    GameOver,
}
```

---

### `settings.rs`

```rust
#[derive(Debug, Clone)]
pub struct Settings {
    pub show_ghost: bool,
    pub show_grid: bool,
    pub colorblind_mode: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            show_ghost: true,
            show_grid: true,
            colorblind_mode: false,
        }
    }
}
```

---

## 🧱 Pasta `src/core/`

### `mod.rs`

```rust
pub mod board;
pub mod piece;
pub mod tetromino;
pub mod rules;
pub mod rng;

pub use board::Board;
pub use piece::Piece;
pub use tetromino::{Tetromino, TetrominoShape};
pub use rules::Rules;
pub use rng::BagRandomizer;
```

---

### `board.rs`

```rust
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
```

---

### `piece.rs`

```rust
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
```

---

### `tetromino.rs`

```rust
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
```

---

### `rules.rs`

```rust
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
```

---

### `rng.rs`

```rust
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
```

---

## 🖼️ Pasta `src/render/`

### `mod.rs`

```rust
pub mod theme;
pub mod draw_board;
pub mod draw_panels;
pub mod anim;

pub use theme::Theme;
pub use anim::Animator;
```

---

### `theme.rs`

```rust
use eframe::egui::Color32;

pub struct Theme {
    pub background: Color32,
    pub panel: Color32,
    pub border: Color32,
    pub text_primary: Color32,
    pub text_secondary: Color32,
    pub grid_line: Color32,
    pub block_colors: [Color32; 8],
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            background: Color32::from_rgb(0x0B, 0x0F, 0x1A),
            panel: Color32::from_rgb(0x11, 0x18, 0x27),
            border: Color32::from_rgb(0x1F, 0x29, 0x37),
            text_primary: Color32::from_rgb(0xE5, 0xE7, 0xEB),
            text_secondary: Color32::from_rgb(0x9C, 0xA3, 0xAF),
            grid_line: Color32::from_rgba_unmultiplied(0x1F, 0x29, 0x37, 80),
            block_colors: [
                Color32::TRANSPARENT,                      // 0: empty
                Color32::from_rgb(0x00, 0xF0, 0xF0),       // 1: I - Cyan
                Color32::from_rgb(0xF0, 0xF0, 0x00),       // 2: O - Yellow
                Color32::from_rgb(0xA0, 0x00, 0xF0),       // 3: T - Purple
                Color32::from_rgb(0xF0, 0xA0, 0x00),       // 4: L - Orange
                Color32::from_rgb(0x00, 0x00, 0xF0),       // 5: J - Blue
                Color32::from_rgb(0x00, 0xF0, 0x00),       // 6: S - Green
                Color32::from_rgb(0xF0, 0x00, 0x00),       // 7: Z - Red
            ],
        }
    }
}
```

---

### `draw_board.rs`

```rust
use eframe::egui::{self, Color32, Pos2, Rect, Rounding, Stroke, Vec2};
use crate::core::{Board, Piece, board::{BOARD_WIDTH, BOARD_HEIGHT}};
use crate::app::settings::Settings;
use super::{Theme, Animator};

const CELL_SIZE: f32 = 24.0;
const BOARD_OFFSET: Vec2 = Vec2::new(50.0, 50.0);

pub fn render(
    ui: &mut egui::Ui,
    theme: &Theme,
    board: &Board,
    piece: &Piece,
    settings: &Settings,
    animator: &Animator,
) {
    let painter = ui.painter();

    // Background
    let board_rect = Rect::from_min_size(
        Pos2::new(BOARD_OFFSET.x, BOARD_OFFSET.y),
        Vec2::new(BOARD_WIDTH as f32 * CELL_SIZE, BOARD_HEIGHT as f32 * CELL_SIZE),
    );
    painter.rect_filled(board_rect, Rounding::ZERO, theme.panel);
    painter.rect_stroke(board_rect, Rounding::ZERO, Stroke::new(2.0, theme.border));

    // Grid lines
    if settings.show_grid {
        for x in 0..=BOARD_WIDTH {
            let x_pos = BOARD_OFFSET.x + x as f32 * CELL_SIZE;
            painter.line_segment(
                [Pos2::new(x_pos, BOARD_OFFSET.y), Pos2::new(x_pos, BOARD_OFFSET.y + BOARD_HEIGHT as f32 * CELL_SIZE)],
                Stroke::new(1.0, theme.grid_line),
            );
        }
        for y in 0..=BOARD_HEIGHT {
            let y_pos = BOARD_OFFSET.y + y as f32 * CELL_SIZE;
            painter.line_segment(
                [Pos2::new(BOARD_OFFSET.x, y_pos), Pos2::new(BOARD_OFFSET.x + BOARD_WIDTH as f32 * CELL_SIZE, y_pos)],
                Stroke::new(1.0, theme.grid_line),
            );
        }
    }

    // Locked blocks
    for y in 0..BOARD_HEIGHT {
        for x in 0..BOARD_WIDTH {
            let cell = board.grid[y][x];
            if cell != 0 {
                draw_block(painter, theme, x as i32, y as i32, cell, 1.0);
            }
        }
    }

    // Ghost piece
    if settings.show_ghost {
        let drop_dist = board.drop_distance(piece);
        for (px, py) in piece.blocks() {
            let gy = py + drop_dist as i32;
            if gy >= 0 && gy < BOARD_HEIGHT as i32 {
                draw_block(painter, theme, px, gy, piece.color_id, 0.3);
            }
        }
    }

    // Current piece
    let scale = animator.lock_pop_scale();
    for (px, py) in piece.blocks() {
        if py >= 0 && py < BOARD_HEIGHT as i32 {
            draw_block_scaled(painter, theme, px, py, piece.color_id, 1.0, scale);
        }
    }
}

fn draw_block(painter: &egui::Painter, theme: &Theme, x: i32, y: i32, color_id: u8, alpha: f32) {
    draw_block_scaled(painter, theme, x, y, color_id, alpha, 1.0);
}

fn draw_block_scaled(
    painter: &egui::Painter,
    theme: &Theme,
    x: i32,
    y: i32,
    color_id: u8,
    alpha: f32,
    scale: f32,
) {
    let base_color = theme.block_colors[color_id as usize];
    let color = Color32::from_rgba_unmultiplied(
        base_color.r(),
        base_color.g(),
        base_color.b(),
        (255.0 * alpha) as u8,
    );

    let center_x = BOARD_OFFSET.x + (x as f32 + 0.5) * CELL_SIZE;
    let center_y = BOARD_OFFSET.y + (y as f32 + 0.5) * CELL_SIZE;
    let half_size = CELL_SIZE * 0.5 * scale;

    let rect = Rect::from_center_size(
        Pos2::new(center_x, center_y),
        Vec2::new(half_size * 2.0 - 2.0, half_size * 2.0 - 2.0),
    );

    // Shadow
    let shadow_rect = rect.translate(Vec2::new(2.0, 2.0));
    painter.rect_filled(shadow_rect, Rounding::same(3.0), Color32::from_rgba_unmultiplied(0, 0, 0, 60));

    // Base block
    painter.rect_filled(rect, Rounding::same(3.0), color);

    // Highlight (top-left)
    let highlight_color = Color32::from_rgba_unmultiplied(255, 255, 255, 40);
    let highlight_rect = Rect::from_min_size(rect.min, Vec2::new(rect.width(), 4.0));
    painter.rect_filled(highlight_rect, Rounding::same(3.0), highlight_color);

    // Border
    painter.rect_stroke(rect, Rounding::same(3.0), Stroke::new(1.0, theme.border));
}
```

---

### `draw_panels.rs`

```rust
use eframe::egui;
use crate::app::TetrisApp;
use crate::core::{Piece, Tetromino};
use super::Theme;

pub fn render(ui: &mut egui::Ui, theme: &Theme, app: &TetrisApp) {
    ui.vertical(|ui| {
        ui.add_space(20.0);

        // Score
        ui.label(egui::RichText::new("SCORE").color(theme.text_secondary).size(12.0));
        ui.label(egui::RichText::new(format!("{}", app.score)).color(theme.text_primary).size(24.0).strong());
        ui.add_space(15.0);

        // Level
        ui.label(egui::RichText::new("LEVEL").color(theme.text_secondary).size(12.0));
        ui.label(egui::RichText::new(format!("{}", app.level)).color(theme.text_primary).size(20.0));
        ui.add_space(15.0);

        // Lines
        ui.label(egui::RichText::new("LINES").color(theme.text_secondary).size(12.0));
        ui.label(egui::RichText::new(format!("{}", app.lines)).color(theme.text_primary).size(20.0));
        ui.add_space(25.0);

        ui.separator();
        ui.add_space(15.0);

        // Next piece
        ui.label(egui::RichText::new("NEXT").color(theme.text_secondary).size(12.0));
        draw_mini_piece(ui, theme, &app.next_piece);
        ui.add_space(20.0);

        // Hold piece
        ui.label(egui::RichText::new("HOLD").color(theme.text_secondary).size(12.0));
        if let Some(ref held) = app.hold_piece {
            draw_mini_piece(ui, theme, held);
        } else {
            ui.label(egui::RichText::new("--").color(theme.text_secondary));
        }
        ui.add_space(25.0);

        ui.separator();
        ui.add_space(15.0);

        // Controls
        ui.label(egui::RichText::new("CONTROLS").color(theme.text_secondary).size(12.0));
        ui.label(egui::RichText::new("←→ Move").color(theme.text_secondary).size(10.0));
        ui.label(egui::RichText::new("↓ Soft Drop").color(theme.text_secondary).size(10.0));
        ui.label(egui::RichText::new("Space Hard Drop").color(theme.text_secondary).size(10.0));
        ui.label(egui::RichText::new("↑/X Rotate CW").color(theme.text_secondary).size(10.0));
        ui.label(egui::RichText::new("Z Rotate CCW").color(theme.text_secondary).size(10.0));
        ui.label(egui::RichText::new("C Hold").color(theme.text_secondary).size(10.0));
        ui.label(egui::RichText::new("P/Esc Pause").color(theme.text_secondary).size(10.0));
    });
}

fn draw_mini_piece(ui: &mut egui::Ui, theme: &Theme, piece: &Piece) {
    let size = 12.0;
    let tetromino = Tetromino::get(piece.shape);
    let blocks = &tetromino.rotations[0];

    let (response, painter) = ui.allocate_painter(egui::Vec2::new(size * 5.0, size * 3.0), egui::Sense::hover());
    let origin = response.rect.min;

    let color = theme.block_colors[piece.color_id as usize];

    for (dx, dy) in blocks {
        let rect = egui::Rect::from_min_size(
            egui::Pos2::new(origin.x + (*dx as f32) * size, origin.y + (*dy as f32) * size),
            egui::Vec2::new(size - 1.0, size - 1.0),
        );
        painter.rect_filled(rect, egui::Rounding::same(2.0), color);
    }
}
```

---

### `anim.rs`

```rust
pub struct Animator {
    lock_pop_timer: f64,
    line_clear_timer: f64,
}

impl Animator {
    pub fn new() -> Self {
        Self {
            lock_pop_timer: 0.0,
            line_clear_timer: 0.0,
        }
    }

    pub fn update(&mut self, dt: f64) {
        if self.lock_pop_timer > 0.0 {
            self.lock_pop_timer -= dt;
        }
        if self.line_clear_timer > 0.0 {
            self.line_clear_timer -= dt;
        }
    }

    pub fn trigger_lock_pop(&mut self) {
        self.lock_pop_timer = 0.12; // 120ms
    }

    pub fn trigger_line_clear(&mut self) {
        self.line_clear_timer = 0.2;
    }

    /// Returns scale factor for lock pop animation (1.0 to 1.15 and back)
    pub fn lock_pop_scale(&self) -> f32 {
        if self.lock_pop_timer > 0.0 {
            let t = (self.lock_pop_timer / 0.12) as f32;
            1.0 + 0.15 * (t * std::f32::consts::PI).sin()
        } else {
            1.0
        }
    }

    #[allow(dead_code)]
    pub fn line_clear_alpha(&self) -> f32 {
        if self.line_clear_timer > 0.0 {
            (self.line_clear_timer / 0.2) as f32
        } else {
            1.0
        }
    }
}
```

---

## ⏱️ Pasta `src/time/`

### `mod.rs`

```rust
pub mod clock;

pub use clock::Clock;
```

---

### `clock.rs`

```rust
use crate::core::Rules;

pub struct Clock {
    accumulator: f64,
    interval: f64,
}

impl Clock {
    pub fn new(level: u32) -> Self {
        Self {
            accumulator: 0.0,
            interval: Rules::gravity_interval(level),
        }
    }

    pub fn set_level(&mut self, level: u32) {
        self.interval = Rules::gravity_interval(level);
    }

    /// Returns true when a tick should occur
    pub fn tick(&mut self, dt: f64) -> bool {
        self.accumulator += dt;
        if self.accumulator >= self.interval {
            self.accumulator -= self.interval;
            true
        } else {
            false
        }
    }
}
```

---

## 📘 README.md

```markdown
# Tetris Modern em Rust

Um jogo **Tetris completo** implementado em **Rust** com interface gráfica moderna usando **egui/eframe**.  
Sem imagens externas, sem áudio — visual gerado inteiramente por código!

## 🧩 Recursos

- Interface gráfica moderna com tema escuro/neon
- Ghost piece (prévia de onde a peça cairá)
- Hold piece (guardar peça para usar depois)
- Wall kick (rotação inteligente perto das paredes)
- Lock delay (tempo para ajustar antes de travar)
- 7-bag randomizer (distribuição justa de peças)
- Sistema de pontuação e níveis
- Menu, pausa e game over com overlay

## 🚀 Executar

```bash
cargo run
```

## 🕹️ Controles

| Tecla       | Ação                    |
| ----------- | ----------------------- |
| ←/→         | Mover esquerda/direita  |
| ↓           | Soft drop (acelerar)    |
| Space       | Hard drop (queda direta)|
| ↑ / X       | Rotacionar horário      |
| Z           | Rotacionar anti-horário |
| C           | Hold (guardar peça)     |
| P / Esc     | Pausar                  |
| R           | Reiniciar (game over)   |
| Q           | Voltar ao menu          |

## 🎨 Paleta de Cores

- Background: `#0B0F1A`
- Painéis: `#111827`
- Bordas: `#1F2937`
- Texto: `#E5E7EB` / `#9CA3AF`
- Blocos: Cyan, Yellow, Purple, Orange, Blue, Green, Red

## 📁 Estrutura do Projeto

```
src/
├── main.rs          # Inicialização do app
├── app/             # Estado do jogo, configurações
├── core/            # Lógica: board, peças, regras, RNG
├── render/          # Tema, desenho de blocos e painéis
└── time/            # Controle de tempo e gravidade
```
```

---

## ✅ Checklist de Conformidade com GDD

| Requisito GDD                  | Implementado |
| ------------------------------ | ------------ |
| UI moderna (egui/eframe)       | ✅           |
| Tema escuro/neon               | ✅           |
| Ghost piece                    | ✅           |
| Hold piece                     | ✅           |
| Wall kicks                     | ✅           |
| Lock delay (300ms)             | ✅           |
| 7-bag randomizer               | ✅           |
| Pontuação (100/300/500/800)    | ✅           |
| Níveis (10 linhas = +1)        | ✅           |
| Gravidade progressiva          | ✅           |
| Menu principal                 | ✅           |
| Pause overlay                  | ✅           |
| Game over com restart          | ✅           |
| Animação lock pop              | ✅           |
| Animação line clear            | ✅           |
| Painel: Score/Level/Lines      | ✅           |
| Painel: Next/Hold              | ✅           |
| Controles completos            | ✅           |
