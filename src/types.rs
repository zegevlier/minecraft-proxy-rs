use crate::cipher::Cipher;
use std::fmt;
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum State {
    Handshaking,
    Status,
    Login,
    Play,
}

impl fmt::Display for State {
    fn fmt(&self,  f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Handshaking => write!(f, "Handshaking"),
            Self::Status => write!(f, "Status"),
            Self::Login => write!(f, "Login"),
            Self::Play => write!(f, "Play"),
        }
        
    }
}

// #[derive(Clone)]
pub struct Status {
    pub compress: u32,
    pub state: State,
    pub client_cipher: Cipher,
    pub server_cipher: Cipher,
}

impl Status {
    pub fn new() -> Status {
        Status {
            compress: 0,
            state: State::Handshaking,
            client_cipher: Cipher::new(),
            server_cipher: Cipher::new(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Direction {
    Serverbound,
    Clientbound,
}

impl fmt::Display for Direction {
    fn fmt(&self,  f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Serverbound => write!(f, "C>S"),
            Self::Clientbound => write!(f, "S>C")
        }
        
    }
}