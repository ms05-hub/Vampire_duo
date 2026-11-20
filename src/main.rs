mod collision;
mod components;
mod concurrency;
mod enemy;
mod error;
mod player;
mod state;
mod ui;
mod weapon;
mod xp;

use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use bevy::prelude::*;

use collision::CollisionPlugin;
use concurrency::saver::SaveCmd;
use concurrency::stats::{GameStats, SharedStats};
use concurrency::{director, saver, ConcurrencyPlugin, SaverTx, SpawnRx};
use enemy::EnemyPlugin;
use error::GameError;
use player::PlayerPlugin;
use state::GameState;
use ui::UiPlugin;
use weapon::WeaponPlugin;
use xp::XpPlugin;

fn main() {
    if let Err(e) = run() {
        eprintln!("erreur fatale : {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), GameError> {
    let stop = Arc::new(AtomicBool::new(false));
    let (spawn_rx, director_handle) = director::start(Arc::clone(&stop))?;
    let (save_tx, saver_handle) = saver::start(PathBuf::from("highscore.txt"))?;
    let stats = SharedStats(Arc::new(Mutex::new(GameStats::default())));

    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .insert_resource(SpawnRx(Mutex::new(spawn_rx)))
        .insert_resource(SaverTx(save_tx.clone()))
        .insert_resource(stats)
        .add_plugins((
            PlayerPlugin,
            EnemyPlugin,
            WeaponPlugin,
            CollisionPlugin,
            XpPlugin,
            UiPlugin,
            ConcurrencyPlugin,
        ));
    // Bloque jusqu'à la fermeture de la fenêtre.
    let _ = app.run();

    // Arrêt propre des threads.
    stop.store(true, Ordering::Relaxed);
    save_tx.send(SaveCmd::Quit).map_err(|_| GameError::ChannelClosed)?;
    director_handle.join().map_err(|_| GameError::ThreadPanicked)?;
    saver_handle.join().map_err(|_| GameError::ThreadPanicked)?;
    Ok(())
}
