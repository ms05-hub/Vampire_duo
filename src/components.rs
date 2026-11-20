//! Composants partagés : c'est le "contrat" entre les deux développeurs.
//! Toute modification ici doit être discutée à deux.
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Health(pub f32);

#[derive(Component)]
pub struct Speed(pub f32);

/// Rayon de collision (cercle).
#[derive(Component)]
pub struct Radius(pub f32);

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Projectile {
    pub damage: f32,
    pub lifetime: Timer,
}
