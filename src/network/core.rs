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
use libp2p::{PeerId, Swarm};

// Stuff needed for general plumbing.
type BoxedError = Box<dyn std::error::Error>;

#[derive(NetworkBehaviour)]
#[behaviour(event_process = false, out_event = "ComposedEvent")]
struct ComposedBehaviour {
    kademlia: Kademlia<MemoryStore>,
}

enum ComposedEvent {
    Kademlia(KademliaEvent),
}

impl From<KademliaEvent> for ComposedEvent {
    fn from(event: KademliaEvent) -> Self {
        ComposedEvent::Kademlia(event)
    }
}

fn create_peer() -> Swarm<ComposedBehaviour> {
    // Generate IDs.
    let key = identity::Keypair::generate_ed25519();
    let peerid = PeerId::from_public_key(key.public());

    // Generate NOISE authentication keys.
    let auth_keys = Keypair::<X25519Spec>::new().into_authentic(&key).unwrap();

    // Create secure-TCP transport that uses tokio under the hood.
    let transport = TokioTcpConfig::new()
        .upgrade(upgrade::Version::V1)
        .authenticate(NoiseConfig::xx(auth_keys).into_authenticated())
        .multiplex(mplex::MplexConfig::new())
        .boxed();

    let behaviour = ComposedBehaviour {
        kademlia: Kademlia::new(peerid.clone(), MemoryStore::new(peerid.clone())),
    };

    Swarm::new(transport, behaviour, peerid.clone())
}
