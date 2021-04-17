use crate::{
    clientbound,
    packet::Parsable,
    serverbound,
    types::{Direction, State},
};
use maplit::hashmap;
use std::collections::HashMap;

type Functions = HashMap<Direction, HashMap<State, HashMap<i32, Box<dyn Parsable + Send>>>>;

struct Funcs {
    f: Functions,
}

impl Funcs {
    fn new() -> Self {
        Self {
            f: hashmap! {
                Direction::Serverbound => hashmap! {
                    State::Handshaking => hashmap! {},
                    State::Status => hashmap! {},
                    State::Login => hashmap! {},
                    State::Play => hashmap! {},
                },
                Direction::Clientbound => hashmap! {
                    State::Handshaking => hashmap! {},
                    State::Status => hashmap! {},
                    State::Login => hashmap! {},
                    State::Play => hashmap! {},
                },
            },
        }
    }

    fn add(&mut self, direction: Direction, state: State, id: i32, func: Box<dyn Parsable + Send>) {
        self.f
            .get_mut(&direction)
            .unwrap()
            .get_mut(&state)
            .unwrap()
            .insert(id, func);
    }
}

pub fn get_functions() -> Functions {
    let mut functions = Funcs::new();

    // handshaking
    // sb
    functions.add(
        Direction::Serverbound,
        State::Handshaking,
        0x00,
        Box::new(serverbound::handshaking::Handshake::empty()),
    );
    // cb

    // status
    // cb
    functions.add(
        Direction::Clientbound,
        State::Status,
        0x00,
        Box::new(clientbound::status::StatusResponse::empty()),
    );

    functions.add(
        Direction::Clientbound,
        State::Status,
        0x01,
        Box::new(clientbound::status::StatusPong::empty()),
    );

    // sb
    functions.add(
        Direction::Serverbound,
        State::Status,
        0x00,
        Box::new(serverbound::status::StatusRequest::empty()),
    );

    functions.add(
        Direction::Serverbound,
        State::Status,
        0x01,
        Box::new(serverbound::status::StatusPing::empty()),
    );

    // login
    // cb
    functions.add(
        Direction::Clientbound,
        State::Login,
        0x00,
        Box::new(clientbound::login::Disconnect::empty()),
    );

    functions.add(
        Direction::Clientbound,
        State::Login,
        0x01,
        Box::new(clientbound::login::EncRequest::empty()),
    );

    functions.add(
        Direction::Clientbound,
        State::Login,
        0x02,
        Box::new(clientbound::login::LoginSuccess::empty()),
    );

    functions.add(
        Direction::Clientbound,
        State::Login,
        0x03,
        Box::new(clientbound::login::SetCompression::empty()),
    );

    functions.add(
        Direction::Clientbound,
        State::Login,
        0x04,
        Box::new(clientbound::login::PluginRequest::empty()),
    );

    // sb
    functions.add(
        Direction::Serverbound,
        State::Login,
        0x00,
        Box::new(serverbound::login::LoginStart::empty()),
    );

    functions.add(
        Direction::Serverbound,
        State::Login,
        0x01,
        Box::new(serverbound::login::EncResponse::empty()),
    );

    functions.add(
        Direction::Serverbound,
        State::Login,
        0x02,
        Box::new(serverbound::login::PluginResponse::empty()),
    );

    // play
    // cb
    functions.add(
        Direction::Clientbound,
        State::Play,
        0x00,
        Box::new(clientbound::play::SpawnEntity::empty()),
    );

    functions.add(
        Direction::Clientbound,
        State::Play,
        0x01,
        Box::new(clientbound::play::SpawnXpOrb::empty()),
    );

    functions.add(
        Direction::Clientbound,
        State::Play,
        0x02,
        Box::new(clientbound::play::SpawnLivingEntity::empty()),
    );

    functions.f
}
