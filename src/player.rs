use bevy::prelude::*;

use crate::components::{Health, Player, Radius, Speed};
use crate::state::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(
                Update,
                (move_player, camera_follow, check_death)
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn((
        Player,
        Health(100.0),
        Speed(220.0),
        Radius(16.0), // à ajuster à la taille affichée
        Sprite {
            image: asset_server.load("player.png"),
            custom_size: Some(Vec2::splat(48.0)), // taille à l'écran
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));
}

fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player: Query<(&mut Transform, &Speed, &mut Sprite), With<Player>>,
) {
    let Ok((mut tf, speed, mut sprite)) = player.single_mut() else {
        return;
    };
    let mut dir = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
        dir.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
        dir.y -= 1.0;
    }
    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
        dir.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
        dir.x += 1.0;
    }
    if dir.x != 0.0 {
        sprite.flip_x = dir.x < 0.0;
    }
    tf.translation += (dir.normalize_or_zero() * speed.0 * time.delta_secs()).extend(0.0);
}

fn camera_follow(
    player: Query<&Transform, With<Player>>,
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    let (Ok(p), Ok(mut c)) = (player.single(), camera.single_mut()) else {
        return;
    };
    c.translation.x = p.translation.x;
    c.translation.y = p.translation.y;
}

fn check_death(
    player: Query<&Health, With<Player>>,
    mut next: ResMut<NextState<GameState>>,
) {
    let Ok(hp) = player.single() else { return };
    if hp.0 <= 0.0 {
        next.set(GameState::GameOver);
    }
}
