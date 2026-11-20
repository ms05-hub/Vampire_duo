use bevy::prelude::*;

use crate::components::{Enemy, Player, Projectile, Radius, Velocity};
use crate::state::GameState;

pub struct WeaponPlugin;

/// Caractéristiques de l'unique arme (modifiées par les upgrades).
#[derive(Resource)]
pub struct WeaponStats {
    pub damage: f32,
    pub projectile_speed: f32,
}

#[derive(Resource)]
struct FireTimer(Timer);

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WeaponStats {
            damage: 10.0,
            projectile_speed: 450.0,
        })
        .insert_resource(FireTimer(Timer::from_seconds(0.7, TimerMode::Repeating)))
        .add_systems(
            Update,
            (auto_fire, move_projectiles, expire_projectiles)
                .run_if(in_state(GameState::Playing)),
        );
    }
}

/// Tire sur l'ennemi le plus proche à intervalle régulier.
fn auto_fire(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<FireTimer>,
    stats: Res<WeaponStats>,
    player: Query<&Transform, With<Player>>,
    enemies: Query<&Transform, With<Enemy>>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }
    let Ok(player_tf) = player.single() else {
        return;
    };
    let origin = player_tf.translation.truncate();
    let target = enemies
        .iter()
        .map(|t| t.translation.truncate())
        .min_by(|a, b| {
            a.distance_squared(origin)
                .total_cmp(&b.distance_squared(origin))
        });
    let Some(target) = target else { return };
    let dir = (target - origin).normalize_or_zero();

    commands.spawn((
        Projectile {
            damage: stats.damage,
            lifetime: Timer::from_seconds(2.0, TimerMode::Once),
        },
        Velocity(dir * stats.projectile_speed),
        Radius(5.0),
        Sprite::from_color(Color::srgb(1.0, 0.9, 0.2), Vec2::splat(10.0)),
        Transform::from_translation(origin.extend(0.5)),
    ));
}

fn move_projectiles(time: Res<Time>, mut q: Query<(&mut Transform, &Velocity), With<Projectile>>) {
    for (mut tf, vel) in &mut q {
        tf.translation += (vel.0 * time.delta_secs()).extend(0.0);
    }
}

fn expire_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    mut q: Query<(Entity, &mut Projectile)>,
) {
    for (e, mut p) in &mut q {
        if p.lifetime.tick(time.delta()).finished() {
            commands.entity(e).try_despawn();
        }
    }
}
