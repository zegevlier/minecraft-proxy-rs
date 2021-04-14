use crate::cipher::Cipher;
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum State {
    Handshaking,
    Status,
    Login,
    Play,
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
