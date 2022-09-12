use libp2p::{
    mdns::{Mdns, MdnsEvent},
    NetworkBehaviour,
};

use std::fmt::Debug;

mod behaviour;
use behaviour::CustomBehaviour;

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "BadOutEvent")]
struct BadBehaviour {
    mdns: Mdns,
    custom: CustomBehaviour,
}

#[derive(Debug)]
enum BadOutEvent {
    Mdns(MdnsEvent),
    None,
}

impl From<MdnsEvent> for BadOutEvent {
    fn from(e: MdnsEvent) -> Self {
        Self::Mdns(e)
    }
}

impl From<void::Void> for BadOutEvent {
    fn from(_e: void::Void) -> Self {
        Self::None
    }
}

fn main() {
    println!("Hello, world!");
}
