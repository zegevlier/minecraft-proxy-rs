use crate::packet::{Packet, Parsable};
use crate::State;

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

    fn parse_packet(mut packet: Packet) -> Result<StatusRequest, ()> {
        let protocol_version = packet.decode_varint()?;
        let server_address = packet.decode_string()?;
        let server_port = packet.decode_ushort()?;
        let next_state = match packet.decode_varint()? {
            1 => State::Status,
            2 => State::Login,
            _ => return Err(()),
        };
        return Ok(StatusRequest {
            protocol_version: protocol_version,
            server_address: server_address,
            server_port: server_port,
            next_state: next_state,
        });
    }

    fn to_str(&self) -> String {
        format!(
            "[HANDSHAKE] {} {}:{} {:?}",
            self.protocol_version, self.server_address, self.server_port, self.next_state
        )
    }
}
