use eframe::egui::Color32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeType {
    Neon,
    Classic,
    Minimal,
}

pub struct Theme {
    pub background: Color32,
    pub background_gradient: Color32,
    pub panel: Color32,
    pub border: Color32,
    pub text_primary: Color32,
    pub text_secondary: Color32,
    pub grid_line: Color32,
    pub block_colors: [Color32; 8],
    pub glow_color: Color32,
}

impl Theme {
    pub fn new(theme_type: ThemeType) -> Self {
        match theme_type {
            ThemeType::Neon => Self::neon(),
            ThemeType::Classic => Self::classic(),
            ThemeType::Minimal => Self::minimal(),
        }
    }

    /// Default Neon theme - dark background with vibrant colors
    fn neon() -> Self {
        Self {
            background: Color32::from_rgb(0x0B, 0x0F, 0x1A),
            background_gradient: Color32::from_rgb(0x0F, 0x17, 0x2A),
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
            glow_color: Color32::from_rgba_unmultiplied(0x00, 0xF0, 0xF0, 30),
        }
    }

    /// Classic theme - retro game boy style
    fn classic() -> Self {
        Self {
            background: Color32::from_rgb(0x1A, 0x1A, 0x2E),
            background_gradient: Color32::from_rgb(0x16, 0x21, 0x3E),
            panel: Color32::from_rgb(0x0F, 0x3D, 0x3E),
            border: Color32::from_rgb(0x00, 0x7A, 0x7A),
            text_primary: Color32::from_rgb(0xE5, 0xFF, 0xDE),
            text_secondary: Color32::from_rgb(0x68, 0xB0, 0xAB),
            grid_line: Color32::from_rgba_unmultiplied(0x00, 0x7A, 0x7A, 60),
            block_colors: [
                Color32::TRANSPARENT,
                Color32::from_rgb(0x00, 0xD4, 0xD4),       // I - Teal
                Color32::from_rgb(0xE0, 0xE0, 0x40),       // O - Yellow
                Color32::from_rgb(0xB0, 0x40, 0xE0),       // T - Purple
                Color32::from_rgb(0xE0, 0x90, 0x30),       // L - Orange
                Color32::from_rgb(0x40, 0x60, 0xE0),       // J - Blue
                Color32::from_rgb(0x40, 0xE0, 0x40),       // S - Green
                Color32::from_rgb(0xE0, 0x40, 0x40),       // Z - Red
            ],
            glow_color: Color32::from_rgba_unmultiplied(0x00, 0xD4, 0xD4, 25),
        }
    }

    /// Minimal theme - clean, modern, muted colors
    fn minimal() -> Self {
        Self {
            background: Color32::from_rgb(0x1E, 0x1E, 0x2E),
            background_gradient: Color32::from_rgb(0x28, 0x28, 0x38),
            panel: Color32::from_rgb(0x2D, 0x2D, 0x3D),
            border: Color32::from_rgb(0x45, 0x45, 0x55),
            text_primary: Color32::from_rgb(0xCD, 0xD6, 0xF4),
            text_secondary: Color32::from_rgb(0x6C, 0x70, 0x86),
            grid_line: Color32::from_rgba_unmultiplied(0x45, 0x45, 0x55, 50),
            block_colors: [
                Color32::TRANSPARENT,
                Color32::from_rgb(0x89, 0xDC, 0xEB),       // I - Sky
                Color32::from_rgb(0xF9, 0xE2, 0xAF),       // O - Peach
                Color32::from_rgb(0xCB, 0xA6, 0xF7),       // T - Mauve
                Color32::from_rgb(0xFA, 0xB3, 0x87),       // L - Flamingo
                Color32::from_rgb(0x89, 0xB4, 0xFA),       // J - Blue
                Color32::from_rgb(0xA6, 0xE3, 0xA1),       // S - Green
                Color32::from_rgb(0xF3, 0x8B, 0xA8),       // Z - Pink
            ],
            glow_color: Color32::from_rgba_unmultiplied(0xCD, 0xD6, 0xF4, 20),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::neon()
    }
}
