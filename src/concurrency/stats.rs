//! Statistiques partagées entre threads : `Arc<Mutex<_>>`.
use std::sync::{Arc, Mutex};

use bevy::prelude::*;

use crate::error::GameError;

#[derive(Default)]
pub struct GameStats {
    pub kills: u32,
}

#[derive(Resource, Clone)]
pub struct SharedStats(pub Arc<Mutex<GameStats>>);

pub fn record_kill(stats: &SharedStats) -> Result<(), GameError> {
    let mut guard = stats.0.lock().map_err(|_| GameError::LockPoisoned)?;
    guard.kills += 1;
    Ok(())
}
