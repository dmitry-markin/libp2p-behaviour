use libp2p::{
    mdns::{Mdns, MdnsEvent},
    NetworkBehaviour,
};

use std::fmt::Debug;

mod behaviour;
use behaviour::CustomBehaviour;

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "BadOutEvent<T>")]
struct BadBehaviour<T: 'static> {
    mdns: Mdns,
    custom: CustomBehaviour<T>,
}

#[derive(Debug)]
enum BadOutEvent<T> {
    Mdns(MdnsEvent),
    Template(T),
    None,
}

impl<T> From<MdnsEvent> for BadOutEvent<T> {
    fn from(e: MdnsEvent) -> Self {
        Self::Mdns(e)
    }
}

impl<T> From<void::Void> for BadOutEvent<T> {
    fn from(_e: void::Void) -> Self {
        Self::None
    }
}

fn main() {
    println!("Hello, world!");
}
