pub mod state;
pub mod settings;

use eframe::egui;
use crate::core::{Board, Piece, Rules, BagRandomizer};
use crate::render::{Theme, draw_board, draw_panels, Animator};
use crate::render::theme::ThemeType;
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

    fn get_theme(&self) -> Theme {
        Theme::new(self.settings.theme)
    }
}

impl eframe::App for TetrisApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();
        let theme = self.get_theme();

        // Set global style based on theme
        let mut style = (*ctx.style()).clone();
        style.visuals.window_fill = theme.background;
        style.visuals.panel_fill = theme.panel;
        style.visuals.widgets.noninteractive.fg_stroke.color = theme.text_primary;
        ctx.set_style(style);

        match self.state {
            GameState::Menu => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(100.0);
                        ui.label(egui::RichText::new("🎮 TETRIS MODERN")
                            .size(32.0)
                            .color(theme.text_primary)
                            .strong());
                        ui.add_space(10.0);
                        ui.label(egui::RichText::new("Rust Edition")
                            .size(14.0)
                            .color(theme.text_secondary));
                        ui.add_space(40.0);

                        if ui.add(egui::Button::new(
                            egui::RichText::new("▶ Play").size(16.0)
                        ).min_size(egui::Vec2::new(120.0, 40.0))).clicked() {
                            self.reset();
                        }
                        ui.add_space(10.0);
                        if ui.add(egui::Button::new(
                            egui::RichText::new("⚙ Settings").size(16.0)
                        ).min_size(egui::Vec2::new(120.0, 40.0))).clicked() {
                            self.state = GameState::Settings;
                        }
                        ui.add_space(10.0);
                        if ui.add(egui::Button::new(
                            egui::RichText::new("✖ Quit").size(16.0)
                        ).min_size(egui::Vec2::new(120.0, 40.0))).clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                });
            }

            GameState::Settings => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(60.0);
                        ui.label(egui::RichText::new("⚙ SETTINGS")
                            .size(28.0)
                            .color(theme.text_primary)
                            .strong());
                        ui.add_space(40.0);

                        // Settings container
                        egui::Frame::none()
                            .fill(theme.panel)
                            .rounding(egui::Rounding::same(8.0))
                            .inner_margin(20.0)
                            .show(ui, |ui| {
                                ui.set_min_width(280.0);

                                // Theme selection
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new("Theme:")
                                        .color(theme.text_primary)
                                        .size(14.0));
                                    ui.add_space(20.0);
                                    egui::ComboBox::from_id_salt("theme_select")
                                        .selected_text(match self.settings.theme {
                                            ThemeType::Neon => "🌙 Neon",
                                            ThemeType::Classic => "🎮 Classic",
                                            ThemeType::Minimal => "✨ Minimal",
                                        })
                                        .show_ui(ui, |ui| {
                                            ui.selectable_value(&mut self.settings.theme, ThemeType::Neon, "🌙 Neon");
                                            ui.selectable_value(&mut self.settings.theme, ThemeType::Classic, "🎮 Classic");
                                            ui.selectable_value(&mut self.settings.theme, ThemeType::Minimal, "✨ Minimal");
                                        });
                                });
                                ui.add_space(15.0);

                                // Ghost piece toggle
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new("Ghost Piece:")
                                        .color(theme.text_primary)
                                        .size(14.0));
                                    ui.add_space(8.0);
                                    ui.checkbox(&mut self.settings.show_ghost, "");
                                });
                                ui.add_space(10.0);

                                // Grid toggle
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new("Show Grid:")
                                        .color(theme.text_primary)
                                        .size(14.0));
                                    ui.add_space(20.0);
                                    ui.checkbox(&mut self.settings.show_grid, "");
                                });
                                ui.add_space(10.0);

                                // Colorblind mode toggle
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new("Colorblind Mode:")
                                        .color(theme.text_primary)
                                        .size(14.0));
                                    ui.add_space(8.0);
                                    ui.checkbox(&mut self.settings.colorblind_mode, "");
                                    if self.settings.colorblind_mode {
                                        ui.label(egui::RichText::new("(coming soon)")
                                            .color(theme.text_secondary)
                                            .size(10.0));
                                    }
                                });
                            });

                        ui.add_space(30.0);

                        if ui.add(egui::Button::new(
                            egui::RichText::new("← Back to Menu").size(14.0)
                        ).min_size(egui::Vec2::new(140.0, 36.0))).clicked() {
                            self.state = GameState::Menu;
                        }
                    });
                });

                ctx.input(|i| {
                    if i.key_pressed(egui::Key::Escape) {
                        self.state = GameState::Menu;
                    }
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
                    }
                }

                // Lock delay (processed every frame, not just on gravity ticks)
                if !self.board.can_move(&self.current_piece, 0, 1) {
                    // Piece is on the ground
                    if let Some(timer) = &mut self.lock_timer {
                        *timer -= dt;
                        if *timer <= 0.0 {
                            self.lock_piece();
                        }
                    } else {
                        self.lock_timer = Some(0.5); // 500ms lock delay
                    }
                } else {
                    // Piece is in the air, reset lock timer
                    self.lock_timer = None;
                }

                // Animation updates
                self.animator.update(dt);

                // Render
                egui::CentralPanel::default().show(ctx, |ui| {
                    draw_board::render(ui, &theme, &self.board, &self.current_piece, &self.settings, &self.animator);
                });

                egui::SidePanel::right("info_panel")
                    .min_width(140.0)
                    .show(ctx, |ui| {
                        draw_panels::render(ui, &theme, self);
                    });
            }

            GameState::Paused => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(150.0);
                        ui.label(egui::RichText::new("⏸ PAUSED")
                            .size(32.0)
                            .color(theme.text_primary)
                            .strong());
                        ui.add_space(30.0);

                        if ui.add(egui::Button::new(
                            egui::RichText::new("▶ Resume").size(16.0)
                        ).min_size(egui::Vec2::new(120.0, 40.0))).clicked() {
                            self.state = GameState::Playing;
                        }
                        ui.add_space(10.0);
                        if ui.add(egui::Button::new(
                            egui::RichText::new("⚙ Settings").size(16.0)
                        ).min_size(egui::Vec2::new(120.0, 40.0))).clicked() {
                            self.state = GameState::Settings;
                        }
                        ui.add_space(10.0);
                        if ui.add(egui::Button::new(
                            egui::RichText::new("🏠 Menu").size(16.0)
                        ).min_size(egui::Vec2::new(120.0, 40.0))).clicked() {
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
                        ui.add_space(100.0);
                        ui.label(egui::RichText::new("💀 GAME OVER")
                            .size(32.0)
                            .color(theme.text_primary)
                            .strong());
                        ui.add_space(30.0);
                        
                        // Score display
                        egui::Frame::none()
                            .fill(theme.panel)
                            .rounding(egui::Rounding::same(8.0))
                            .inner_margin(20.0)
                            .show(ui, |ui| {
                                ui.label(egui::RichText::new(format!("Score: {}", self.score))
                                    .size(22.0)
                                    .color(theme.text_primary)
                                    .strong());
                                ui.add_space(5.0);
                                ui.label(egui::RichText::new(format!("Level: {}", self.level))
                                    .size(16.0)
                                    .color(theme.text_secondary));
                                ui.label(egui::RichText::new(format!("Lines: {}", self.lines))
                                    .size(16.0)
                                    .color(theme.text_secondary));
                            });
                        
                        ui.add_space(30.0);

                        if ui.add(egui::Button::new(
                            egui::RichText::new("🔄 Restart").size(16.0)
                        ).min_size(egui::Vec2::new(120.0, 40.0))).clicked() {
                            self.reset();
                        }
                        ui.add_space(10.0);
                        if ui.add(egui::Button::new(
                            egui::RichText::new("🏠 Menu").size(16.0)
                        ).min_size(egui::Vec2::new(120.0, 40.0))).clicked() {
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
