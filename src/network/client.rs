// Pull imports from the parent module
use super::*;
// The core module gives use the core objects that will interact with the eventloop.
use super::{core, eventloop};

// Logging
use log::info;

// TODO:
// For testing purposes, we will need a way to trigger events on the client side, a mock.

pub struct ClientConfig {
    // If this node a bootstrap_node skip boot-loading
    pub bootstrap_node: bool,
}

pub struct Client {
    // TODO: This field should go into the Event Loop module.
    swarm: Swarm<core::ComposedBehaviour>,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            swarm: core::create_peer(),
        }
    }
}

impl Client {
    pub fn new() -> Self {
        Self {
            swarm: core::create_peer(),
        }
    }

    pub fn with_config(config: ClientConfig) -> Self {
        if config.bootstrap_node {
            // TODO: Add stuff to do with the bootstrap_node parameter.
            info!("This is a bootstrap node.");
        }

        Self {
            swarm: core::create_peer(),
        }
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
        ClientEvent::StartListening(addr) => {
            // Here, we should pass the `addr` to the EventLoop.
            Ok(())
        }
        ClientEvent::DialAnother(addr) => todo!(),
        ClientEvent::StartProviding => todo!(),
        ClientEvent::SeekSomething => todo!(),
    }
}
