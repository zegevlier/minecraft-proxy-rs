use crate::{
    clientbound,
    packet::Parsable,
    serverbound,
    types::{Direction, State},
};
use maplit::hashmap;
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum Func {
    Handshake,
    StatusResponse,
    StatusPong,
    StatusRequest,
    StatusPing,
    Disconnect,
    EncRequest,
    LoginSuccess,
    SetCompression,
    PluginRequest,
    LoginStart,
    EncResponse,
    PluginResponse,
    SpawnEntity,
    SpawnXpOrb,
    SpawnLivingEntity,
    SpawnPainting,
    SpawnPlayer,
}

impl Func {
    pub fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

pub struct Functions {
    map: HashMap<Direction, HashMap<State, HashMap<i32, Func>>>,
    list: HashMap<Func, Box<dyn Parsable + Send>>,
}

impl Functions {
    fn new() -> Self {
        Self {
            map: hashmap! {
                Direction::Clientbound => hashmap! {
                    State::Handshaking => hashmap! {},
                    State::Status => hashmap! {
                        0x00 => Func::StatusResponse,
                        0x01 => Func::StatusPong
                    },
                    State::Login => hashmap! {
                        0x00 => Func::Disconnect,
                        0x01 => Func::EncRequest,
                        0x02 => Func::LoginSuccess,
                        0x03 => Func::SetCompression,
                        0x04 => Func::PluginRequest
                    },
                    State::Play => hashmap! {
                        0x00 => Func::SpawnEntity,
                        0x01 => Func::SpawnXpOrb,
                        0x02 => Func::SpawnLivingEntity,
                        0x03 => Func::SpawnPainting,
                        0x04 => Func::SpawnPlayer
                    },
                },
                Direction::Serverbound => hashmap! {
                    State::Handshaking => hashmap! {
                        0x00 => Func::Handshake.into(),
                    },
                    State::Status => hashmap! {
                        0x00 => Func::StatusRequest.into(),
                        0x01 => Func::StatusPing.into(),
                    },
                    State::Login => hashmap! {
                        0x00 => Func::LoginStart,
                        0x01 => Func::EncResponse,
                        0x02 => Func::PluginResponse,
                    },
                    State::Play => hashmap! {},
                },

            },
            list: HashMap::new(),
        }
    }

    fn add(&mut self, id: Func, func: Box<dyn Parsable + Send>) {
        self.list.insert(id, func);
    }

    pub fn get_name(&self, direction: &Direction, state: &State, pid: &i32) -> Option<&Func> {
        match self
            .map
            .get(direction)
            .unwrap()
            .get(state)
            .unwrap()
            .get(pid)
        {
            Some(id) => Some(id),
            None => None,
        }
    }

    pub fn get(&self, id: &Func) -> Option<&Box<dyn Parsable + Send>> {
        match self.list.get(id) {
            Some(func) => Some(func),
            None => None,
        }
    }
}

pub fn get_functions() -> Functions {
    let mut functions = Functions::new();

    // Handshaking
    // Serverbound
    functions.add(
        Func::Handshake,
        Box::new(serverbound::handshaking::Handshake::empty()),
    );

    // Status
    // Clientbound
    functions.add(
        Func::StatusResponse,
        Box::new(clientbound::status::StatusResponse::empty()),
    );

    functions.add(
        Func::StatusPong,
        Box::new(clientbound::status::StatusPong::empty()),
    );

    // Serverbound
    functions.add(
        Func::StatusRequest,
        Box::new(serverbound::status::StatusRequest::empty()),
    );

    functions.add(
        Func::StatusPing,
        Box::new(serverbound::status::StatusPing::empty()),
    );

    // Login
    // Clientbound
    functions.add(
        Func::Disconnect,
        Box::new(clientbound::login::Disconnect::empty()),
    );

    functions.add(
        Func::EncRequest,
        Box::new(clientbound::login::EncRequest::empty()),
    );

    functions.add(
        Func::LoginSuccess,
        Box::new(clientbound::login::LoginSuccess::empty()),
    );

    functions.add(
        Func::SetCompression,
        Box::new(clientbound::login::SetCompression::empty()),
    );

    functions.add(
        Func::PluginRequest,
        Box::new(clientbound::login::PluginRequest::empty()),
    );

    // Serverbound
    functions.add(
        Func::LoginStart,
        Box::new(serverbound::login::LoginStart::empty()),
    );

    functions.add(
        Func::EncResponse,
        Box::new(serverbound::login::EncResponse::empty()),
    );

    functions.add(
        Func::PluginResponse,
        Box::new(serverbound::login::PluginResponse::empty()),
    );

    // Play
    // Clientbound
    functions.add(
        Func::SpawnEntity,
        Box::new(clientbound::play::SpawnEntity::empty()),
    );

    functions.add(
        Func::SpawnXpOrb,
        Box::new(clientbound::play::SpawnXpOrb::empty()),
    );

    functions.add(
        Func::SpawnLivingEntity,
        Box::new(clientbound::play::SpawnLivingEntity::empty()),
    );

    functions.add(
        Func::SpawnPainting,
        Box::new(clientbound::play::SpawnPainting::empty()),
    );

    functions.add(
        Func::SpawnPlayer,
        Box::new(clientbound::play::SpawnPlayer::empty()),
    );

    functions
}
