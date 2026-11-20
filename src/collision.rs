use bevy::prelude::*;

use crate::components::{Enemy, Health, Player, Projectile, Radius};
use crate::concurrency::stats::{record_kill, SharedStats};
use crate::state::GameState;

pub struct CollisionPlugin;

/// Dégâts par seconde infligés par un ennemi au contact.
const CONTACT_DPS: f32 = 10.0;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (projectile_hits, enemy_contact).run_if(in_state(GameState::Playing)),
        );
    }
}

fn projectile_hits(
    mut commands: Commands,
    stats: Res<SharedStats>,
    projectiles: Query<(Entity, &Transform, &Radius, &Projectile)>,
    mut enemies: Query<(Entity, &Transform, &Radius, &mut Health), With<Enemy>>,
) {
    // Double boucle : suffisant pour quelques centaines d'entités.
    for (p_entity, p_tf, p_r, proj) in &projectiles {
        for (e_entity, e_tf, e_r, mut hp) in &mut enemies {
            if hp.0 <= 0.0 {
                continue;
            }
            let dist = p_tf
                .translation
                .truncate()
                .distance(e_tf.translation.truncate());
            if dist < p_r.0 + e_r.0 {
                hp.0 -= proj.damage;
                commands.entity(p_entity).try_despawn();
                if hp.0 <= 0.0 {
                    commands.entity(e_entity).try_despawn();
                    if let Err(e) = record_kill(&stats) {
                        warn!("stats indisponibles : {e}");
                    }
                }
                break;
            }
        }
    }
}

fn enemy_contact(
    time: Res<Time>,
    mut player: Query<(&Transform, &Radius, &mut Health), With<Player>>,
    enemies: Query<(&Transform, &Radius), With<Enemy>>,
) {
    let Ok((p_tf, p_r, mut hp)) = player.single_mut() else {
        return;
    };
    for (e_tf, e_r) in &enemies {
        let dist = p_tf
            .translation
            .truncate()
            .distance(e_tf.translation.truncate());
        if dist < p_r.0 + e_r.0 {
            hp.0 -= CONTACT_DPS * time.delta_secs();
        }
    }
}
