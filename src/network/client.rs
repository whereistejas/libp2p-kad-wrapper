// Pull imports from the parent module
use super::*;
// The core module gives use the core objects that will interact with the eventloop.
use super::{core, eventloop};

// TODO:
// For testing purposes, we will need a way to trigger events on the client side, a mock.

pub struct Client {
    // Is this node a bootnode, if yes, skip boot-loading
    bootnode: bool,
}

impl Client {
    pub fn new(bootnode: bool) -> Self {
        Self { bootnode }
    }

    pub fn listen(&mut self, addr: Option<Multiaddr>) -> Result<(), BoxedError> {
        handle_client_event(self, ClientEvent::StartListening(addr))
    }

    fn provide(&mut self, filename: String) -> Result<(), BoxedError> {
        todo!()
    }

    fn dial(&mut self, addr: Multiaddr) -> Result<(), BoxedError> {
        todo!()
    }
}

enum ClientEvent {
    // This event will be triggered when we spin up a new farmer node.
    StartListening(Option<Multiaddr>),
    // This event will be triggered when, ummm... I don't know, yet.
    DialAnother(Option<Multiaddr>),
    // Start providing your plot to the K-DHT network.
    StartProviding,
    // Find something on the K-DHT network.
    SeekSomething,
}

fn handle_client_event(client: &mut Client, event: ClientEvent) -> Result<(), BoxedError> {
    match event {
        ClientEvent::StartListening(addr) => todo!(),
        ClientEvent::DialAnother(addr) => todo!(),
        ClientEvent::StartProviding => todo!(),
        ClientEvent::SeekSomething => todo!(),
    }
}
