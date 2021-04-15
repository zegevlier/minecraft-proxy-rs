use crate::packet::{Packet, Parsable};
use crate::types::{State, Status};
use crate::utils;
use hex::encode;
use std::convert::TryInto;

#[derive(Clone)]
pub struct EncRequest {
    server_id: String,
    public_key_length: i32,
    public_key: Vec<u8>,
    verify_token_length: i32,
    verify_token: Vec<u8>,
}

impl Parsable for EncRequest {
    fn empty() -> Self {
        Self {
            server_id: "".into(),
            public_key_length: 0,
            public_key: Vec::new(),
            verify_token_length: 0,
            verify_token: Vec::new(),
        }
    }

    fn parse_packet(&mut self, mut packet: Packet) -> Result<(), ()> {
        self.server_id = packet.decode_string()?;
        self.public_key_length = packet.decode_varint()?;
        self.public_key = packet.read(self.public_key_length as usize)?;
        self.verify_token_length = packet.decode_varint()?;
        self.verify_token = packet.read(self.verify_token_length as usize)?;
        return Ok(());
    }

    fn get_printable(&self) -> (&str, String) {
        (
            "ENC_REQUEST",
            format!(
                "{} {} {} {}",
                // self.server_id,
                self.public_key_length,
                utils::make_string_fixed_length(encode(self.public_key.clone()), 20),
                self.verify_token_length,
                utils::make_string_fixed_length(encode(self.verify_token.clone()), 20)
            ),
        )
    }
}

#[derive(Clone)]
pub struct SetCompression {
    threshold: i32,
}

impl Parsable for SetCompression {
    fn empty() -> Self {
        Self { threshold: 0 }
    }

    fn parse_packet(&mut self, mut packet: Packet) -> Result<(), ()> {
        self.threshold = packet.decode_varint()?;
        return Ok(());
    }

    fn get_printable(&self) -> (&str, String) {
        ("SET_COMPRESSION", format!("{}", self.threshold,))
    }

    fn status_updating(&self) -> bool {
        true
    }

    fn update_status(&self, status: &mut Status) -> Result<(), ()> {
        status.compress = self.threshold as u32;
        Ok(())
    }
}

#[derive(Clone)]
pub struct LoginSuccess {
    uuid: u128,
    username: String,
}

impl Parsable for LoginSuccess {
    fn empty() -> Self {
        Self {
            uuid: 0,
            username: "".into(),
        }
    }

    fn parse_packet(&mut self, mut packet: Packet) -> Result<(), ()> {
        self.uuid = u128::from_be_bytes(packet.read(16)?.try_into().unwrap());
        self.username = packet.decode_string()?;
        return Ok(());
    }

    fn get_printable(&self) -> (&str, String) {
        (
            "LOGIN_SUCCESS",
            format!("{:x} {}", self.uuid, self.username,),
        )
    }

    fn status_updating(&self) -> bool {
        true
    }

    fn update_status(&self, status: &mut Status) -> Result<(), ()> {
        status.state = State::Play;
        log::debug!("State updated to {}", status.state);
        Ok(())
    }
}

#[derive(Clone)]
pub struct Disconnect {
    reason: String,
}

impl Parsable for Disconnect {
    fn empty() -> Self {
        Self { reason: "".into() }
    }

    fn parse_packet(&mut self, mut packet: Packet) -> Result<(), ()> {
        self.reason = packet.decode_string()?;
        return Ok(());
    }

    fn get_printable(&self) -> (&str, String) {
        ("LOGIN_DISCONNECT", format!("{}", self.reason))
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
