use crate::packet::{Packet, Parsable};
use crate::State;

#[derive(Clone)]
pub struct StatusRequest {
    protocol_version: i32,
    server_address: String,
    server_port: u16,
    next_state: State,
}

impl Parsable for StatusRequest {
    fn empty() -> Self {
        StatusRequest {
            protocol_version: 0,
            server_address: "".into(),
            server_port: 0,
            next_state: State::Handshaking,
        }
    }

    fn parse_packet(&mut self, mut packet: Packet) -> Result<(), ()> {
        self.protocol_version = packet.decode_varint()?;
        self.server_address = packet.decode_string()?;
        self.server_port = packet.decode_ushort()?;
        self.next_state = match packet.decode_varint()? {
            1 => State::Status,
            2 => State::Login,
            _ => return Err(()),
        };
        return Ok(());
    }

    fn to_str(&self) -> String {
        format!(
            "[HANDSHAKE] {} {}:{} {:?}",
            self.protocol_version, self.server_address, self.server_port, self.next_state
        )
    }
}
