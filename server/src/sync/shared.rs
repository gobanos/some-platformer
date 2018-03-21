use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::net::SocketAddr;

use super::Tx;

/// Shorthand for the shared handle to the state
pub type SharedHandle = Arc<Mutex<Shared>>;

/// The shared state, to allow task to communicate together
pub struct Shared {
    pub peers: HashMap<SocketAddr, Tx>,
}

impl Shared {
    pub fn new() -> Self {
        Shared {
            peers: HashMap::new(),
        }
    }
}
