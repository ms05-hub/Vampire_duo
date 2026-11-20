//! Thread "Director" : décide du rythme et de la force des ennemis,
//! indépendamment du framerate. Communication par canal (mpsc).
use std::f32::consts::TAU;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, TrySendError};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use rand::Rng;

use crate::error::GameError;

pub struct SpawnCmd {
    pub angle: f32,
    pub health: f32,
}

pub fn start(stop: Arc<AtomicBool>) -> Result<(Receiver<SpawnCmd>, JoinHandle<()>), GameError> {
    // Canal borné : si le jeu est en pause, on jette les ordres au lieu de saturer la mémoire.
    let (tx, rx) = mpsc::sync_channel(64);

    let handle = thread::Builder::new()
        .name("director".into())
        .spawn(move || {
            let start = Instant::now();
            let mut rng = rand::rng();
            while !stop.load(Ordering::Relaxed) {
                let t = start.elapsed().as_secs_f32();
                let cmd = SpawnCmd {
                    angle: rng.random_range(0.0..TAU),
                    health: 10.0 + t * 0.2,
                };
                if let Err(TrySendError::Disconnected(_)) = tx.try_send(cmd) {
                    break; // le jeu a fermé le canal
                }
                let delay = (0.8 - t * 0.005).max(0.05);
                thread::sleep(Duration::from_secs_f32(delay));
            }
        })
        .map_err(GameError::ThreadSpawn)?;

    Ok((rx, handle))
}
