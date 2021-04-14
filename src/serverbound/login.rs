use crate::packet::{Packet, Parsable};

#[derive(Clone)]
pub struct LoginStart {
    username: String,
}

impl Parsable for LoginStart {
    fn empty() -> Self {
        Self {
            username: "".into(),
        }
    }

    fn parse_packet(&mut self, mut packet: Packet) -> Result<(), ()> {
        self.username = packet.decode_string()?;
        return Ok(());
    }

    fn to_str(&self) -> String {
        format!("[LOGIN_START] {}", self.username,)
    }
}
