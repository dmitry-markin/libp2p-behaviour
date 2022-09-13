use libp2p::{
    mdns::{Mdns, MdnsEvent},
    NetworkBehaviour,
};

use std::{fmt::Debug, marker::PhantomData};

mod behaviour;
use behaviour::CustomBehaviour;

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "BadOutEvent")]
struct BadBehaviour<T: 'static + Send> {
    mdns: Mdns,
    custom: CustomBehaviour<T>,
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
