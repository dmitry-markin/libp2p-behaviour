use libp2p::{NetworkBehaviour, mdns::{Mdns, MdnsEvent}};

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "GoodOutEvent")]
struct GoodBehaviour {
    mdns: Mdns,
}

enum GoodOutEvent {
    Mdns(MdnsEvent),
}

impl From<MdnsEvent> for GoodOutEvent {
    fn from(v: MdnsEvent) -> Self {
        Self::Mdns(v)
    }
}

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "BadOutEvent<T>")]
struct BadBehaviour<T> {
    mdns: Mdns,
    template: Option<T>,
}

enum BadOutEvent<T> {
    Mdns(MdnsEvent),
    Template(T),
}

impl<T> From<MdnsEvent> for BadOutEvent<T> {
    fn from(v: MdnsEvent) -> Self {
        Self::Mdns(v)
    }
}

fn main() {
    println!("Hello, world!");
}
