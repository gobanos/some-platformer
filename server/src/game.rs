use std::time::Duration;

use sync::CRx;
use sync::shared::SharedHandle;

use lib::sync::message::{Client, Server};

pub struct Game {
    state: SharedHandle,
    receiver: CRx,
}

impl Game {
    pub fn new(receiver: CRx, state: SharedHandle) -> Self {
        Game { state, receiver }
    }

    pub fn update(&mut self, _elapsed_time: Duration) {
        while let Ok((msg, author)) = self.receiver.try_recv() {
            info!("Game got message: {:?} from {:?}", msg, author);

            match msg {
                Client::Test => for (&addr, tx) in &self.state.lock().unwrap().peers {
                    if addr != author {
                        tx.unbounded_send(Server::Test).unwrap();
                    }
                },
                Client::Ping(_) => unreachable!(), // the ping is handled by the peer
            }
        }
    }
}
