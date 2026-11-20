//! Tout le code multi-thread du projet vit ici.
pub mod director;
pub mod saver;
pub mod stats;

use std::sync::mpsc::{Receiver, Sender};
use std::sync::Mutex;

use bevy::prelude::*;

use crate::state::GameState;
use director::SpawnCmd;
use saver::SaveCmd;
use stats::SharedStats;

/// `Receiver` n'est pas `Sync` : on l'enveloppe dans un `Mutex` pour en faire une ressource Bevy.
#[derive(Resource)]
pub struct SpawnRx(pub Mutex<Receiver<SpawnCmd>>);

#[derive(Resource)]
pub struct SaverTx(pub Sender<SaveCmd>);

pub struct ConcurrencyPlugin;

impl Plugin for ConcurrencyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), save_on_game_over);
    }
}

fn save_on_game_over(stats: Res<SharedStats>, saver: Res<SaverTx>) {
    let Ok(s) = stats.0.lock() else { return };
    if saver.0.send(SaveCmd::HighScore(s.kills)).is_err() {
        warn!("thread de sauvegarde indisponible");
    }
}
