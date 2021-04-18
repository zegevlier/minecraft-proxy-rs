use crate::packet::{Packet, Parsable};

// 0x00
#[derive(Clone)]
pub struct SpawnEntity {
    entity_id: i32,
    object_uuid: u128,
    r#type: i32,
    x: f64,
    y: f64,
    z: f64,
    pitch: u8,
    yaw: u8,
    data: i32,
    velocity_x: i16,
    velocity_y: i16,
    velocity_z: i16,
}

impl Parsable for SpawnEntity {
    fn empty() -> Self {
        Self {
            entity_id: 0,
            object_uuid: 0,
            r#type: 0,
            x: 0f64,
            y: 0f64,
            z: 0f64,
            pitch: 0,
            yaw: 0,
            data: 0,
            velocity_x: 0,
            velocity_y: 0,
            velocity_z: 0,
        }
    }

    fn parse_packet(&mut self, mut packet: Packet) -> Result<(), ()> {
        self.entity_id = packet.decode_varint()?;
        self.object_uuid = packet.decode_uuid()?;
        self.r#type = packet.decode_varint()?;
        self.x = packet.decode_double()?;
        self.y = packet.decode_double()?;
        self.z = packet.decode_double()?;
        self.pitch = packet.read(1)?[0];
        self.yaw = packet.read(1)?[0];
        self.data = packet.decode_int()?;
        self.velocity_x = packet.decode_short()?;
        self.velocity_y = packet.decode_short()?;
        self.velocity_z = packet.decode_short()?;
        return Ok(());
    }

    fn get_printable(&self) -> String {
        format!(
            "{} {:x} {} {} {} {} {} {} {} {} {} {}",
            self.entity_id,
            self.object_uuid,
            self.r#type,
            self.x,
            self.y,
            self.z,
            self.pitch,
            self.yaw,
            self.data,
            self.velocity_x,
            self.velocity_y,
            self.velocity_z
        )
    }
}

//0x01
#[derive(Clone)]
pub struct SpawnXpOrb {
    entity_id: i32,
    x: f64,
    y: f64,
    z: f64,
    count: i16,
}

impl Parsable for SpawnXpOrb {
    fn empty() -> Self {
        Self {
            entity_id: 0,
            x: 0f64,
            y: 0f64,
            z: 0f64,
            count: 0,
        }
    }

    fn parse_packet(&mut self, mut packet: Packet) -> Result<(), ()> {
        self.entity_id = packet.decode_varint()?;
        self.x = packet.decode_double()?;
        self.y = packet.decode_double()?;
        self.z = packet.decode_double()?;
        self.count = packet.decode_short()?;
        return Ok(());
    }

    fn get_printable(&self) -> String {
        format!(
            "{} {} {} {} {}",
            self.entity_id, self.x, self.y, self.z, self.count
        )
    }
}

//0x02
#[derive(Clone)]
pub struct SpawnLivingEntity {
    entity_id: i32,
    object_uuid: u128,
    r#type: i32,
    x: f64,
    y: f64,
    z: f64,
    yaw: u8,
    pitch: u8,
    head_pitch: u8,
    velocity_x: i16,
    velocity_y: i16,
    velocity_z: i16,
}

impl Parsable for SpawnLivingEntity {
    fn empty() -> Self {
        Self {
            entity_id: 0,
            object_uuid: 0,
            r#type: 0,
            x: 0f64,
            y: 0f64,
            z: 0f64,
            yaw: 0,
            pitch: 0,
            head_pitch: 0,
            velocity_x: 0,
            velocity_y: 0,
            velocity_z: 0,
        }
    }

    fn parse_packet(&mut self, mut packet: Packet) -> Result<(), ()> {
        self.entity_id = packet.decode_varint()?;
        self.object_uuid = packet.decode_uuid()?;
        self.r#type = packet.decode_varint()?;
        self.x = packet.decode_double()?;
        self.y = packet.decode_double()?;
        self.z = packet.decode_double()?;
        self.yaw = packet.read(1)?[0];
        self.pitch = packet.read(1)?[0];
        self.head_pitch = packet.read(1)?[0];
        self.velocity_x = packet.decode_short()?;
        self.velocity_y = packet.decode_short()?;
        self.velocity_z = packet.decode_short()?;
        return Ok(());
    }

    fn get_printable(&self) -> String {
        format!(
            "{} {:x} {} {} {} {} {} {} {} {} {} {}",
            self.entity_id,
            self.object_uuid,
            self.r#type,
            self.x,
            self.y,
            self.z,
            self.yaw,
            self.pitch,
            self.head_pitch,
            self.velocity_x,
            self.velocity_y,
            self.velocity_z
        )
    }
}

//0x03
#[derive(Clone, Debug)]
enum FacingDirection {
    North,
    South,
    East,
    West,
}
#[derive(Clone)]
pub struct SpawnPainting {
    entity_id: i32,
    object_uuid: u128,
    motive: i32,
    x: i64,
    y: i64,
    z: i64,
    direction: FacingDirection,
}

impl Parsable for SpawnPainting {
    fn empty() -> Self {
        Self {
            entity_id: 0,
            object_uuid: 0,
            motive: 0,
            x: 0,
            y: 0,
            z: 0,
            direction: FacingDirection::North,
        }
    }

    fn parse_packet(&mut self, mut packet: Packet) -> Result<(), ()> {
        self.entity_id = packet.decode_varint()?;
        self.object_uuid = packet.decode_uuid()?;
        self.motive = packet.decode_varint()?;
        let position = packet.decode_position()?;
        self.x = position.0;
        self.y = position.1;
        self.z = position.2;
        match packet.read(1)?[0] {
            0x00 => self.direction = FacingDirection::South,
            0x01 => self.direction = FacingDirection::West,
            0x02 => self.direction = FacingDirection::North,
            0x03 => self.direction = FacingDirection::East,
            _ => return Err(()),
        }
        return Ok(());
    }

    fn get_printable(&self) -> String {
        format!(
            "{} {:x} {} {} {} {} {:?}",
            self.entity_id, self.object_uuid, self.motive, self.x, self.y, self.z, self.direction,
        )
    }
}

//0x04
#[derive(Clone)]
pub struct SpawnPlayer {
    entity_id: i32,
    player_uuid: u128,
    x: f64,
    y: f64,
    z: f64,
    yaw: u8,
    pitch: u8,
}

impl Parsable for SpawnPlayer {
    fn empty() -> Self {
        Self {
            entity_id: 0,
            player_uuid: 0,
            x: 0f64,
            y: 0f64,
            z: 0f64,
            yaw: 0,
            pitch: 0,
        }
    }

    fn parse_packet(&mut self, mut packet: Packet) -> Result<(), ()> {
        self.entity_id = packet.decode_varint()?;
        self.player_uuid = packet.decode_uuid()?;
        self.x = packet.decode_double()?;
        self.y = packet.decode_double()?;
        self.z = packet.decode_double()?;
        self.yaw = packet.read(1)?[0];
        self.pitch = packet.read(1)?[0];
        return Ok(());
    }

    fn get_printable(&self) -> String {
        format!(
            "{} {:x} {} {} {} {} {}",
            self.entity_id, self.player_uuid, self.x, self.y, self.z, self.yaw, self.pitch,
        )
    }
}

#[derive(Clone, Debug)]
enum DiggingStatus {
    Started,
    Cancelled,
    Finished,
}
//0x07
#[derive(Clone)]
pub struct AckPlayerDigging {
    x: i64,
    y: i64,
    z: i64,
    block: i32,
    status: DiggingStatus,
    successful: bool,
}

impl Parsable for AckPlayerDigging {
    fn empty() -> Self {
        Self {
            x: 0,
            y: 0,
            z: 0,
            block: 0,
            status: DiggingStatus::Started,
            successful: false,
        }
    }

    fn parse_packet(&mut self, mut packet: Packet) -> Result<(), ()> {
        let position = packet.decode_position()?;
        self.x = position.0;
        self.y = position.1;
        self.z = position.2;
        self.block = packet.decode_varint()?;
        self.status = match packet.decode_varint()? {
            0x00 => DiggingStatus::Started,
            0x01 => DiggingStatus::Cancelled,
            0x02 => DiggingStatus::Finished,
            _ => return Err(()),
        };
        self.successful = packet.decode_bool()?;
        return Ok(());
    }

    fn get_printable(&self) -> String {
        format!(
            "{} {} {} {} {:?} {}",
            self.x, self.y, self.z, self.block, self.status, self.successful,
        )
    }
}
