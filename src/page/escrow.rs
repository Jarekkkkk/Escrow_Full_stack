#![allow(clippy::wildcard_imports)]
#![allow(dead_code, unused_variables)]

use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

pub fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    Model {
        foo: String::from("escrow_model_foos"),
    }
}

// ------ ------
//     Model
// ------ ------
pub struct Model {
    foo: String,
}

// ------ ------
//     Update
// ------ ------

pub enum Msg {
    EscrowInit,
}

pub fn update(msg: Msg, model: &mut Model, _: &impl Orders<Msg>) {
    match msg {
        Msg::EscrowInit => log!("escrow init"),
    }
}

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model) -> Node<Msg> {
    div!("escrow")
}
