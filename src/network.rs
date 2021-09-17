// This is the event loopm the client API will interact with.
mod eventloop;
// The core module defines the `ComposedBehaviour` and `ComposedEvent`
mod core;

// Exposed API.
pub(crate) mod client;
