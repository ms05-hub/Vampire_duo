//! Thread de sauvegarde : l'I/O disque ne doit jamais bloquer la boucle de jeu.
use std::path::PathBuf;
use std::sync::mpsc::{self, Sender};
use std::thread::{self, JoinHandle};

use crate::error::GameError;

pub enum SaveCmd {
    HighScore(u32),
    Quit,
}

pub fn start(path: PathBuf) -> Result<(Sender<SaveCmd>, JoinHandle<()>), GameError> {
    let (tx, rx) = mpsc::channel();
    let handle = thread::Builder::new()
        .name("saver".into())
        .spawn(move || {
            for cmd in rx {
                match cmd {
                    SaveCmd::HighScore(score) => {
                        if let Err(e) = std::fs::write(&path, score.to_string()) {
                            eprintln!("sauvegarde impossible : {e}");
                        }
                    }
                    SaveCmd::Quit => break,
                }
            }
        })
        .map_err(GameError::ThreadSpawn)?;
    Ok((tx, handle))
}
