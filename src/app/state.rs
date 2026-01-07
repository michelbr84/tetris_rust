#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Menu,
    Settings,
    Playing,
    Paused,
    GameOver,
}
