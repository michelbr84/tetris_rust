use crate::render::theme::ThemeType;

#[derive(Debug, Clone)]
pub struct Settings {
    pub show_ghost: bool,
    pub show_grid: bool,
    pub colorblind_mode: bool,
    pub theme: ThemeType,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            show_ghost: true,
            show_grid: true,
            colorblind_mode: false,
            theme: ThemeType::Neon,
        }
    }
}
