use crate::packet::{Packet, Parsable};
use crate::types::Status;
use base64::decode;
use hex::encode;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

use crate::utils;

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

    fn get_printable(&self) -> (&str, String) {
        ("LOGIN_START", format!("{}", self.username,))
    }
}

#[derive(Clone)]
pub struct EncResponse {
    shared_secret_length: i32,
    shared_secret: Vec<u8>,
    verify_token_length: i32,
    verify_token: Vec<u8>,
}

impl Parsable for EncResponse {
    fn empty() -> Self {
        Self {
            shared_secret_length: 0,
            shared_secret: Vec::new(),
            verify_token_length: 0,
            verify_token: Vec::new(),
        }
    }

    fn parse_packet(&mut self, mut packet: Packet) -> Result<(), ()> {
        self.shared_secret_length = packet.decode_varint()?;
        self.shared_secret = packet.read(self.shared_secret_length as usize)?;
        self.verify_token_length = packet.decode_varint()?;
        self.verify_token = packet.read(self.verify_token_length as usize)?;
        return Ok(());
    }

    fn get_printable(&self) -> (&str, String) {
        (
            "ENC_RESPONSE",
            format!(
                "{} {} {} {}",
                self.shared_secret_length,
                utils::make_string_fixed_length(encode(self.shared_secret.clone()), 20),
                self.verify_token_length,
                utils::make_string_fixed_length(encode(self.verify_token.clone()), 20)
            ),
        )
    }

    fn state_updating(&self) -> bool {
        true
    }

    fn update_status(&self, state: &mut Status) -> Result<(), ()> {
        let path_str = if cfg!(windows) {
            let appdata = std::env::var("APPDATA").unwrap();
            Path::new(&appdata)
                .join(".minecraft/logs/latest.log")
                .to_str()
                .unwrap()
                .to_string()
        } else {
            let home = std::env::var("HOME").unwrap();
            Path::new(&home)
                .join(".minecraft/logs/latest.log")
                .to_str()
                .unwrap()
                .to_string()
        };
        let path = Path::new(&path_str);
        // println!("{:?}", &path.to_str());

        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        let mut secret_key = String::new();

        for line in reader.lines() {
            let line = match line {
                Ok(line) => line,
                Err(_) => continue,
            };
            if line.contains("[STDOUT]: Secret Key: ") {
                secret_key = line
                    .split("[STDOUT]: Secret Key: ")
                    .nth(1)
                    .unwrap()
                    .to_string()
                    .replace("\n", "")
                    .replace("\r", "");
            }
        }

        state.client_cipher.enable(&decode(&secret_key).unwrap());
        state.server_cipher.enable(&decode(&secret_key).unwrap());

        log::debug!("Updated cipher with secret key {}", secret_key);
        Ok(())
    }
}
