use std::fmt;

#[derive(Debug)]
pub enum GameError {
    Io(std::io::Error),
    ThreadSpawn(std::io::Error),
    ThreadPanicked,
    ChannelClosed,
    LockPoisoned,
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "erreur d'E/S : {e}"),
            Self::ThreadSpawn(e) => write!(f, "impossible de lancer un thread : {e}"),
            Self::ThreadPanicked => write!(f, "un thread a paniqué"),
            Self::ChannelClosed => write!(f, "canal fermé"),
            Self::LockPoisoned => write!(f, "mutex empoisonné"),
        }
    }
}

impl std::error::Error for GameError {}

impl From<std::io::Error> for GameError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}
