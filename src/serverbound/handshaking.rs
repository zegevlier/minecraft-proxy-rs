use crate::packet::{Packet, Parsable};
use crate::{State, Status};

#[derive(Clone)]
pub struct Handshake {
    protocol_version: i32,
    server_address: String,
    server_port: u16,
    next_state: State,
}

impl Parsable for Handshake {
    fn empty() -> Self {
        Handshake {
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

    fn get_printable(&self) -> (&str, String) {
        (
            "HANDSHAKE",
            format!(
                "{} {}:{} {:?}",
                self.protocol_version, self.server_address, self.server_port, self.next_state
            ),
        )
    }

    fn state_updating(&self) -> bool {
        true
    }

    fn update_status(&self, status: &mut Status) -> Result<(), ()> {
        status.state = self.next_state.clone();
        log::debug!("State updated to {}", status.state);
        Ok(())
    }
}
