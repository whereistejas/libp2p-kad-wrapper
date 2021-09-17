// Pull imports from the parent module
use super::*;
// The core module gives use the core objects that will interact with the eventloop.
use super::{core, eventloop};

// TODO:
// For testing purposes, we will need a way to trigger events on the client side, a mock.

struct Client {
    // Is this node a bootnode, if yes, skip boot-loading
    bootnode: bool,
}

trait ClientInterface {
    // Start listening on a given MultiAddr
    fn listen(&mut self, addr: Multiaddr) -> Result<(), BoxedError>;

    // Dial a specific peer.
    fn dial(&mut self, addr: Multiaddr) -> Result<(), BoxedError>;

    // The best thing to do here would be to give a path here.
    // NOTE: In the specific context of the spartan-farmer, we will want the ability to define
    // specific pieces/chunks of the plot here.
    fn provide(&mut self, filename: String) -> Result<(), BoxedError>;
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

impl ClientInterface for Client {
    fn listen(&mut self, addr: Multiaddr) -> Result<(), BoxedError> {
        todo!()
    }

    fn provide(&mut self, filename: String) -> Result<(), BoxedError> {
        todo!()
    }

    fn dial(&mut self, addr: Multiaddr) -> Result<(), BoxedError> {
        todo!()
    }
}

