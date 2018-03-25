use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::net::SocketAddr;

use super::Tx;

/// Shorthand for the shared handle to the state
pub type StateHandle = Arc<Mutex<State>>;

/// The shared state, to allow task to communicate together
pub struct State {
    pub peers: HashMap<SocketAddr, Tx>,
}

impl State {
    pub fn new() -> Self {
        State {
            peers: HashMap::new(),
        }
    }
}
