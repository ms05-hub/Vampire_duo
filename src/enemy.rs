use bevy::prelude::*;

use crate::components::{Enemy, Health, Player, Radius, Speed};
use crate::concurrency::SpawnRx;
use crate::state::GameState;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (apply_spawn_cmds, chase_player).run_if(in_state(GameState::Playing)),
        );
    }
}

/// Lit les ordres du thread Director et crée les ennemis.
fn apply_spawn_cmds(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    rx: Res<SpawnRx>,
    player: Query<&Transform, With<Player>>,
) {
    let Ok(player_tf) = player.single() else {
        return;
    };
    let Ok(rx) = rx.0.lock() else { return };

    let image: Handle<Image> = asset_server.load("enemy.png");
    const ENEMY_SIZE: f32 = 90.0;
    for _ in 0..20 {
        let Ok(cmd) = rx.try_recv() else { break };
        let offset = Vec2::from_angle(cmd.angle) * 700.0;
        commands.spawn((
            Enemy,
            Health(cmd.health),
            Speed(80.0),
            Radius(ENEMY_SIZE), // à ajuster à la taille affichée
            Sprite {
                image: image.clone(),
                custom_size: Some(Vec2::splat(ENEMY_SIZE)),
                ..default()
            },
            Transform::from_translation(player_tf.translation + offset.extend(0.0)),
        ));
    }
}

fn chase_player(
    time: Res<Time>,
    player: Query<&Transform, With<Player>>,
    mut enemies: Query<(&mut Transform, &Speed, &mut Sprite), (With<Enemy>, Without<Player>)>,
) {
    let Ok(player_tf) = player.single() else {
        return;
    };
    for (mut tf, speed, mut sprite) in &mut enemies {
        let dir = (player_tf.translation - tf.translation).normalize_or_zero();
        if dir.x != 0.0 {
            sprite.flip_x = dir.x < 0.0;
        }
        tf.translation += dir * speed.0 * time.delta_secs();
    }
}
