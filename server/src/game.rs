use sync::C2GReceiver;
use sync::state::StateHandle;

use std::time::Duration;

use lib::sync::message::{Client, Server};
use lib::world::gameworld::GameWorld;

/// The game handle server logic:
/// - Processing client messages
/// - Update the world
/// - Dispatch server messages to client
pub struct Game<'a, 'b> {
    // Handle to peer list
    state: StateHandle,

    // Receiver of the client -> server channel
    receiver: C2GReceiver,

    // The world state
    world: GameWorld<'a, 'b>,
}

impl<'a, 'b> Game<'a, 'b> {
    pub fn new(receiver: C2GReceiver, state: StateHandle) -> Self {
        Game {
            state,
            receiver,
            world: GameWorld::new(),
        }
    }

    /// Update the game state
    pub fn update(&mut self, _elapsed_time: Duration) {
        // Poll messages from clients
        while let Ok((msg, author)) = self.receiver.try_recv() {
            debug!("Game got a message from {:?}: {:?}", author, msg);

            match msg {
                // The `Test` message is dispatched to all peers but author
                Client::Test => for (&addr, tx) in &self.state.lock().unwrap().peers {
                    if addr != author {
                        tx.unbounded_send(Server::Test).unwrap();
                    }
                },
                Client::Ping(_) => unreachable!(), // the ping is handled by the peer
            }
        }

        // Update the world state
        self.world.update();
    }
}
