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

    // Gradient background (fake gradient using multiple rectangles)
    let board_rect = Rect::from_min_size(
        Pos2::new(BOARD_OFFSET.x, BOARD_OFFSET.y),
        Vec2::new(BOARD_WIDTH as f32 * CELL_SIZE, BOARD_HEIGHT as f32 * CELL_SIZE),
    );
    
    // Draw gradient background (3 bands)
    let gradient_steps = 3;
    let step_height = board_rect.height() / gradient_steps as f32;
    for i in 0..gradient_steps {
        let t = i as f32 / (gradient_steps - 1) as f32;
        let color = lerp_color(theme.background, theme.background_gradient, t);
        let rect = Rect::from_min_size(
            Pos2::new(board_rect.min.x, board_rect.min.y + i as f32 * step_height),
            Vec2::new(board_rect.width(), step_height + 1.0),
        );
        painter.rect_filled(rect, Rounding::ZERO, color);
    }
    
    // Border with glow effect
    painter.rect_stroke(
        board_rect.expand(2.0), 
        Rounding::same(4.0), 
        Stroke::new(4.0, theme.glow_color)
    );
    painter.rect_stroke(board_rect, Rounding::same(2.0), Stroke::new(2.0, theme.border));

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

    // Locked blocks with line clear animation
    let line_alpha = animator.line_clear_alpha();
    for y in 0..BOARD_HEIGHT {
        for x in 0..BOARD_WIDTH {
            let cell = board.grid[y][x];
            if cell != 0 {
                draw_block(painter, theme, x as i32, y as i32, cell, line_alpha);
            }
        }
    }

    // Ghost piece
    if settings.show_ghost {
        let drop_dist = board.drop_distance(piece);
        for (px, py) in piece.blocks() {
            let gy = py + drop_dist as i32;
            if gy >= 0 && gy < BOARD_HEIGHT as i32 {
                draw_ghost_block(painter, theme, px, gy, piece.color_id);
            }
        }
    }

    // Current piece with lock pop animation
    let scale = animator.lock_pop_scale();
    for (px, py) in piece.blocks() {
        if py >= 0 && py < BOARD_HEIGHT as i32 {
            draw_block_scaled(painter, theme, px, py, piece.color_id, 1.0, scale, true);
        }
    }
}

fn lerp_color(a: Color32, b: Color32, t: f32) -> Color32 {
    Color32::from_rgb(
        (a.r() as f32 + (b.r() as f32 - a.r() as f32) * t) as u8,
        (a.g() as f32 + (b.g() as f32 - a.g() as f32) * t) as u8,
        (a.b() as f32 + (b.b() as f32 - a.b() as f32) * t) as u8,
    )
}

fn draw_block(painter: &egui::Painter, theme: &Theme, x: i32, y: i32, color_id: u8, alpha: f32) {
    draw_block_scaled(painter, theme, x, y, color_id, alpha, 1.0, false);
}

fn draw_ghost_block(painter: &egui::Painter, theme: &Theme, x: i32, y: i32, color_id: u8) {
    let base_color = theme.block_colors[color_id as usize];
    let color = Color32::from_rgba_unmultiplied(
        base_color.r(),
        base_color.g(),
        base_color.b(),
        50,
    );

    let rect = Rect::from_min_size(
        Pos2::new(BOARD_OFFSET.x + x as f32 * CELL_SIZE + 2.0, BOARD_OFFSET.y + y as f32 * CELL_SIZE + 2.0),
        Vec2::new(CELL_SIZE - 4.0, CELL_SIZE - 4.0),
    );

    // Dashed outline effect for ghost
    painter.rect_stroke(rect, Rounding::same(3.0), Stroke::new(2.0, color));
}

fn draw_block_scaled(
    painter: &egui::Painter,
    theme: &Theme,
    x: i32,
    y: i32,
    color_id: u8,
    alpha: f32,
    scale: f32,
    is_active: bool,
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

    // Glow effect for active piece
    if is_active && scale > 1.0 {
        let glow_rect = rect.expand(4.0);
        painter.rect_filled(
            glow_rect, 
            Rounding::same(6.0), 
            Color32::from_rgba_unmultiplied(base_color.r(), base_color.g(), base_color.b(), 30)
        );
    }

    // Shadow (offset darker rectangle)
    let shadow_rect = rect.translate(Vec2::new(2.0, 2.0));
    painter.rect_filled(
        shadow_rect, 
        Rounding::same(3.0), 
        Color32::from_rgba_unmultiplied(0, 0, 0, (80.0 * alpha) as u8)
    );

    // Base block with rounded corners
    painter.rect_filled(rect, Rounding::same(4.0), color);

    // Inner highlight (3D effect - top edge)
    let highlight_color = Color32::from_rgba_unmultiplied(255, 255, 255, (60.0 * alpha) as u8);
    let highlight_rect = Rect::from_min_size(
        rect.min + Vec2::new(2.0, 2.0),
        Vec2::new(rect.width() - 4.0, 4.0),
    );
    painter.rect_filled(highlight_rect, Rounding::same(2.0), highlight_color);

    // Left edge highlight
    let left_highlight = Rect::from_min_size(
        rect.min + Vec2::new(2.0, 4.0),
        Vec2::new(3.0, rect.height() - 8.0),
    );
    painter.rect_filled(left_highlight, Rounding::same(1.0), Color32::from_rgba_unmultiplied(255, 255, 255, (30.0 * alpha) as u8));

    // Bottom/right shadow edge (3D effect)
    let bottom_shadow_color = Color32::from_rgba_unmultiplied(0, 0, 0, (40.0 * alpha) as u8);
    let bottom_rect = Rect::from_min_size(
        Pos2::new(rect.min.x + 2.0, rect.max.y - 4.0),
        Vec2::new(rect.width() - 4.0, 3.0),
    );
    painter.rect_filled(bottom_rect, Rounding::same(1.0), bottom_shadow_color);

    // Border
    painter.rect_stroke(
        rect, 
        Rounding::same(4.0), 
        Stroke::new(1.0, Color32::from_rgba_unmultiplied(
            (theme.border.r() as f32 * alpha) as u8,
            (theme.border.g() as f32 * alpha) as u8,
            (theme.border.b() as f32 * alpha) as u8,
            255
        ))
    );
}
