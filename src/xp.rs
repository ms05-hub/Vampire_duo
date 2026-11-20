//! TODO (Dev B) : gemmes d'XP lâchées par les ennemis, niveaux,
//! passage à GameState::LevelUp, choix de 3 upgrades parmi
//! (+dégâts, +vitesse projectile, +vie max, ...) en modifiant WeaponStats.
use bevy::prelude::*;

pub struct XpPlugin;

impl Plugin for XpPlugin {
    fn build(&self, _app: &mut App) {}
}
