use crate::packet::Parsable;
use crate::types::{Direction, State};
use maplit::hashmap;
use std::collections::HashMap;

type Functions = HashMap<Direction, HashMap<State, HashMap<i32, Box<dyn Parsable + Send>>>>;

fn add_to_functions(
    functions: &mut Functions,
    direction: Direction,
    state: State,
    id: i32,
    func: Box<dyn Parsable + Send>,
) {
    functions
        .get_mut(&direction)
        .unwrap()
        .get_mut(&state)
        .unwrap()
        .insert(id, func);
}

pub fn get_functions() -> Functions {
    let mut functions: Functions = hashmap! {
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
    };

    add_to_functions(
        &mut functions,
        Direction::Serverbound,
        State::Handshaking,
        0x00,
        Box::new(crate::serverbound::handshaking::Handshake::empty()),
    );

    add_to_functions(
        &mut functions,
        Direction::Serverbound,
        State::Login,
        0x00,
        Box::new(crate::serverbound::login::LoginStart::empty()),
    );

    add_to_functions(
        &mut functions,
        Direction::Clientbound,
        State::Login,
        0x01,
        Box::new(crate::serverbound::login::LoginStart::empty()),
    );

    functions
}
