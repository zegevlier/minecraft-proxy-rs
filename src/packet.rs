use crate::types::Status;
use dyn_clone::DynClone;
use std::convert::TryInto;

#[derive(Debug)]
pub struct Packet {
    data: Vec<u8>,
}

impl Packet {
    pub fn new() -> Packet {
        Packet { data: Vec::new() }
    }

    pub fn from(packet_data: Vec<u8>) -> Packet {
        Packet { data: packet_data }
    }

    pub fn push(&mut self, data: u8) {
        self.data.push(data)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn get_slice(&self) -> &[u8] {
        self.data.as_slice()
    }

    pub fn get(&self) -> Vec<u8> {
        self.data.clone()
    }

    pub fn clear(&mut self) {
        self.data = Vec::new();
    }

    pub fn read(&mut self, amount: usize) -> Result<Vec<u8>, ()> {
        if self.data.len() < amount {
            return Err(());
        }
        let to_be_returned = self.data.drain(0..amount);
        let read_value = to_be_returned.collect::<Vec<u8>>();
        Ok(read_value)
    }

    pub fn set(&mut self, value: Vec<u8>) {
        self.data = value;
    }

    pub fn decode_varint(&mut self) -> Result<i32, ()> {
        let mut num_read = 0;
        let mut result: i32 = 0;
        let mut read: u8;
        loop {
            read = self.read(1)?[0];
            let value: i32 = (read & 0b01111111) as i32;
            result |= value << (7 * num_read);

            num_read += 1;
            if num_read > 5 {
                return Err(());
            }
            if (read & 0b10000000) == 0 {
                break;
            }
        }
        return Ok(result);
    }

    pub fn decode_varlong(&mut self) -> Result<i64, ()> {
        let mut num_read = 0;
        let mut result: i64 = 0;
        let mut read: u8;
        loop {
            read = self.read(1)?[0];
            let value: i64 = (read & 0b01111111) as i64;
            result |= value << (7 * num_read);

            num_read += 1;
            if num_read > 10 {
                return Err(());
            }
            if (read & 0b10000000) == 0 {
                break;
            }
        }
        return Ok(result);
    }

    pub fn decode_string(&mut self) -> Result<String, ()> {
        let string_length = self.decode_varint()?;
        return Ok(String::from_utf8(self.read(string_length.try_into().unwrap())?).unwrap());
    }

    pub fn decode_ushort(&mut self) -> Result<u16, ()> {
        Ok(u16::from_be_bytes(self.read(2)?.try_into().unwrap()))
    }

    pub fn decode_long(&mut self) -> Result<i64, ()> {
        Ok(i64::from_be_bytes(self.read(8)?.try_into().unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_varint() {
        let mut packet = Packet::new();
        let values = vec![
            (vec![0x00], 0),
            (vec![0x01], 1),
            (vec![0x02], 2),
            (vec![0x7f], 127),
            (vec![0x80, 0x01], 128),
            (vec![0xff, 0x01], 255),
            (vec![0xff, 0xff, 0x7f], 2097151),
            (vec![0xff, 0xff, 0xff, 0xff, 0x07], 2147483647),
            (vec![0xff, 0xff, 0xff, 0xff, 0x0f], -1),
            (vec![0x80, 0x80, 0x80, 0x80, 0x08], -2147483648),
        ];
        for (p, v) in values {
            packet.set(p);
            assert_eq!(packet.decode_varint().unwrap(), v);
            packet.clear()
        }
    }

    #[test]
    fn test_varlong() {
        let mut packet = Packet::new();
        let values = vec![
            (vec![0x00], 0),
            (vec![0x01], 1),
            (vec![0x02], 2),
            (vec![0x7f], 127),
            (vec![0x80, 0x01], 128),
            (vec![0xff, 0x01], 255),
            (vec![0xff, 0xff, 0xff, 0xff, 0x07], 2147483647),
            (
                vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f],
                9223372036854775807,
            ),
            (
                vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01],
                -1,
            ),
            (
                vec![0x80, 0x80, 0x80, 0x80, 0xf8, 0xff, 0xff, 0xff, 0xff, 0x01],
                -2147483648,
            ),
            (
                vec![0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01],
                -9223372036854775808,
            ),
        ];
        for (p, v) in values {
            packet.set(p);
            assert_eq!(packet.decode_varlong().unwrap(), v);
            packet.clear()
        }
    }
}

pub trait Parsable: DynClone {
    fn empty() -> Self
    where
        Self: Sized;

    fn parse_packet(&mut self, packet: Packet) -> Result<(), ()>;

    fn get_printable(&self) -> (&str, String);

    fn update_status(&self, _status: &mut Status) -> Result<(), ()> {
        Ok(())
    }

    fn status_updating(&self) -> bool {
        false
    }
}

dyn_clone::clone_trait_object!(Parsable);
