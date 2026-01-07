use eframe::egui::{self, Color32, Pos2, Rect, Rounding, Stroke, Vec2};
use crate::app::TetrisApp;
use crate::core::{Piece, Tetromino};
use super::Theme;

pub fn render(ui: &mut egui::Ui, theme: &Theme, app: &TetrisApp) {
    let painter = ui.painter();
    let panel_rect = ui.available_rect_before_wrap();
    
    // Panel background with subtle gradient
    painter.rect_filled(panel_rect, Rounding::same(8.0), theme.panel);
    painter.rect_stroke(panel_rect, Rounding::same(8.0), Stroke::new(1.0, theme.border));

    ui.vertical(|ui| {
        ui.add_space(20.0);

        // Score section with emphasis
        draw_stat_section(ui, theme, "SCORE", &format!("{}", app.score), 28.0, true);
        ui.add_space(15.0);

        // Level
        draw_stat_section(ui, theme, "LEVEL", &format!("{}", app.level), 22.0, false);
        ui.add_space(12.0);

        // Lines
        draw_stat_section(ui, theme, "LINES", &format!("{}", app.lines), 22.0, false);
        ui.add_space(20.0);

        // Divider
        draw_divider(ui, theme);
        ui.add_space(15.0);

        // Next piece
        ui.label(egui::RichText::new("NEXT").color(theme.text_secondary).size(12.0));
        ui.add_space(5.0);
        draw_mini_piece(ui, theme, &app.next_piece);
        ui.add_space(15.0);

        // Hold piece
        ui.label(egui::RichText::new("HOLD").color(theme.text_secondary).size(12.0));
        ui.add_space(5.0);
        if let Some(ref held) = app.hold_piece {
            if app.can_hold {
                draw_mini_piece(ui, theme, held);
            } else {
                draw_mini_piece_faded(ui, theme, held);
            }
        } else {
            ui.label(egui::RichText::new("—").color(theme.text_secondary).size(14.0));
        }
        ui.add_space(20.0);

        // Divider
        draw_divider(ui, theme);
        ui.add_space(15.0);

        // Controls header
        ui.label(egui::RichText::new("CONTROLS").color(theme.text_secondary).size(11.0));
        ui.add_space(8.0);
        
        // Control hints with icons
        draw_control_hint(ui, theme, "←→", "Move");
        draw_control_hint(ui, theme, "↓", "Soft Drop");
        draw_control_hint(ui, theme, "Space", "Hard Drop");
        draw_control_hint(ui, theme, "↑/X", "Rotate CW");
        draw_control_hint(ui, theme, "Z", "Rotate CCW");
        draw_control_hint(ui, theme, "C", "Hold");
        draw_control_hint(ui, theme, "P/Esc", "Pause");
    });
}

fn draw_stat_section(ui: &mut egui::Ui, theme: &Theme, label: &str, value: &str, value_size: f32, highlight: bool) {
    ui.label(egui::RichText::new(label).color(theme.text_secondary).size(11.0));
    
    let value_text = if highlight {
        egui::RichText::new(value)
            .color(theme.text_primary)
            .size(value_size)
            .strong()
    } else {
        egui::RichText::new(value)
            .color(theme.text_primary)
            .size(value_size)
    };
    
    ui.label(value_text);
}

fn draw_divider(ui: &mut egui::Ui, theme: &Theme) {
    let (response, painter) = ui.allocate_painter(Vec2::new(ui.available_width(), 1.0), egui::Sense::hover());
    let rect = response.rect;
    
    // Gradient divider effect
    let center = rect.center().x;
    let half_width = rect.width() * 0.4;
    
    painter.line_segment(
        [
            Pos2::new(center - half_width, rect.center().y),
            Pos2::new(center + half_width, rect.center().y)
        ],
        Stroke::new(1.0, theme.border),
    );
}

fn draw_control_hint(ui: &mut egui::Ui, theme: &Theme, key: &str, action: &str) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(key).color(theme.text_primary).size(10.0).strong());
        ui.label(egui::RichText::new(action).color(theme.text_secondary).size(10.0));
    });
}

fn draw_mini_piece(ui: &mut egui::Ui, theme: &Theme, piece: &Piece) {
    draw_mini_piece_with_alpha(ui, theme, piece, 1.0);
}

fn draw_mini_piece_faded(ui: &mut egui::Ui, theme: &Theme, piece: &Piece) {
    draw_mini_piece_with_alpha(ui, theme, piece, 0.4);
}

fn draw_mini_piece_with_alpha(ui: &mut egui::Ui, theme: &Theme, piece: &Piece, alpha: f32) {
    let size = 14.0;
    let tetromino = Tetromino::get(piece.shape);
    let blocks = &tetromino.rotations[0];

    let (response, painter) = ui.allocate_painter(Vec2::new(size * 5.0, size * 3.0), egui::Sense::hover());
    let origin = response.rect.min + Vec2::new(4.0, 0.0);

    let base_color = theme.block_colors[piece.color_id as usize];
    let color = Color32::from_rgba_unmultiplied(
        base_color.r(),
        base_color.g(), 
        base_color.b(),
        (255.0 * alpha) as u8,
    );

    for (dx, dy) in blocks {
        let x = origin.x + (*dx as f32) * size;
        let y = origin.y + (*dy as f32) * size;
        
        let rect = Rect::from_min_size(
            Pos2::new(x, y),
            Vec2::new(size - 2.0, size - 2.0),
        );

        // Mini shadow
        let shadow_rect = rect.translate(Vec2::new(1.0, 1.0));
        painter.rect_filled(shadow_rect, Rounding::same(2.0), Color32::from_rgba_unmultiplied(0, 0, 0, (40.0 * alpha) as u8));
        
        // Block
        painter.rect_filled(rect, Rounding::same(3.0), color);
        
        // Mini highlight
        let highlight_rect = Rect::from_min_size(
            rect.min + Vec2::new(1.0, 1.0),
            Vec2::new(rect.width() - 2.0, 2.0),
        );
        painter.rect_filled(highlight_rect, Rounding::same(1.0), Color32::from_rgba_unmultiplied(255, 255, 255, (40.0 * alpha) as u8));
    }
}
