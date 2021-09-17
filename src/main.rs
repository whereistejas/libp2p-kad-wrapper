#![allow(unused_imports, unused_variables, dead_code)]
mod network;

use network::client::Client;

// Ideally, speaking this file should only interact with the Client API defined in
// network/client.rs

fn main() {
    let mut client = Client::new(false);

    client.listen(None).expect("The peer is unable to listen.");
}
