use crate::packet::{Packet, Parsable};
use crate::{State, Status};

#[derive(Clone)]
pub struct StatusResponse {
    json_response: String,
}

impl Parsable for StatusResponse {
    fn empty() -> Self {
        Self {
            json_response: "".into(),
        }
    }

    fn parse_packet(&mut self, mut packet: Packet) -> Result<(), ()> {
        self.json_response = packet.decode_string()?;
        return Ok(());
    }

    fn to_str(&self) -> String {
        format!("[STATUS_RESPONSE] {}", self.json_response)
    }
}

#[derive(Clone)]
pub struct StatusPong {
    payload: i64,
}

impl Parsable for StatusPong {
    fn empty() -> Self {
        Self { payload: 0 }
    }

    fn parse_packet(&mut self, mut packet: Packet) -> Result<(), ()> {
        self.payload = packet.decode_long()?;
        return Ok(());
    }

    fn to_str(&self) -> String {
        format!("[STATUS_PONG] {}", self.payload)
    }

    fn state_updating(&self) -> bool {
        true
    }

    fn update_state(&self, state: &mut Status) -> Result<(), ()> {
        state.state = State::Handshaking;
        Ok(())
    }
}
