use crate::packet::{Packet, Parsable};
use crate::types::Status;

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

    fn to_str(&self) -> String {
        format!(
            "[ECN_REQUEST] {} {} {:x?} {} {:x?}",
            self.server_id,
            self.public_key_length,
            self.public_key,
            self.verify_token_length,
            self.verify_token
        )
    }
}


#[derive(Clone)]
pub struct SetCompression {
    threshold: i32,
}

impl Parsable for SetCompression {
    fn empty() -> Self {
        Self {
            threshold: 0
        }
    }

    fn parse_packet(&mut self, mut packet: Packet) -> Result<(), ()> {
        self.threshold = packet.decode_varint()?;
        return Ok(());
    }

    fn to_str(&self) -> String {
        format!(
            "[SET_COMPRESSION] {}",
            self.threshold,
        )
    }

    fn state_updating(&self) -> bool {
        true
    }

    fn update_state(&self, state: &mut Status) -> Result<(), ()> {
        state.compress = self.threshold as u32;
        Ok(())
    }
}