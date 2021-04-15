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

    fn get_printable(&self) -> (&str, String) {
        ("STATUS_RESPONSE", format!("{}", self.json_response))
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

    fn get_printable(&self) -> (&str, String) {
        ("STATUS_PONG", format!("{}", self.payload))
    }

    fn status_updating(&self) -> bool {
        true
    }

    fn update_status(&self, status: &mut Status) -> Result<(), ()> {
        status.state = State::Handshaking;
        log::debug!("State updated to {}", status.state);
        Ok(())
    }
}
