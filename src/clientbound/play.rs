use crate::packet::{Packet, Parsable};

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

    fn get_printable(&self) -> (&str, String) {
        (
            "SPAWN_ENTITY",
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
            ),
        )
    }
}

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

    fn get_printable(&self) -> (&str, String) {
        (
            "SPAWN_XP_ORB",
            format!(
                "{} {} {} {} {}",
                self.entity_id, self.x, self.y, self.z, self.count
            ),
        )
    }
}

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

    fn get_printable(&self) -> (&str, String) {
        (
            "SPAWN_LIVING_ENTITY",
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
            ),
        )
    }
}
