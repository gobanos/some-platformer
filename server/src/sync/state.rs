use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::net::SocketAddr;

use super::G2CSender;

/// Shorthand for the shared handle to the state
pub type StateHandle = Arc<Mutex<State>>;

/// The shared state, to allow task to communicate together
pub struct State {
    pub peers: HashMap<SocketAddr, G2CSender>,
}

impl State {
    pub fn new() -> Self {
        State {
            peers: HashMap::new(),
        }
    }
}
