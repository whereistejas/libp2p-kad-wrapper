// Stuff for Kademlia
use libp2p::kad::{record::store::MemoryStore, Kademlia, KademliaEvent};

// Stuff for defining composed behaviour
use libp2p::NetworkBehaviour;

// Stuff needed to create the swarm
use libp2p::core::{upgrade, Transport};
use libp2p::identity;
use libp2p::mplex;
use libp2p::noise::{Keypair, NoiseConfig, X25519Spec};
use libp2p::tcp::TokioTcpConfig;
use libp2p::{Multiaddr, PeerId, Swarm};

// Stuff needed for general plumbing.
type BoxedError = Box<dyn std::error::Error>;

// This is the event loop the client API will interact with.
mod eventloop;
// The core module defines the `ComposedBehaviour` and `ComposedEvent`
mod core;

// Exposed API.
pub mod client;
