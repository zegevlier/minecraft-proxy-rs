#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum State {
    Handshaking,
    Status,
    Login,
    Play
}

#[derive(Clone)]
pub struct Status {
    pub compress: u32,
    pub state: State,
}

impl Status {
    pub fn new() -> Status {
        Status {
            compress: 0,
            state: State::Handshaking,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Direction {
    Serverbound,
    Clientbound,
}