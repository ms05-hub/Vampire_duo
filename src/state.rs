use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Playing,
    // Justifié : sera utilisé par xp.rs (écran de choix d'upgrade, à implémenter).
    #[allow(dead_code)]
    LevelUp,
    GameOver,
}
