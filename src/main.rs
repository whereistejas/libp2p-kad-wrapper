#![allow(unused_imports, unused_variables, dead_code)]
mod network;

use network::client::{Client, ClientConfig};

// Ideally, speaking this file should only interact with the Client API defined in
// network/client.rs

fn main() {
    let config = ClientConfig {
        bootstrap_node: false,
    };

    let mut client = Client::with_config(config);

    client.listen(None).expect("The peer is unable to listen.");
}
