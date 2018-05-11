use lib::sync::message::{Client, Server};

use std::net::SocketAddr;
use std::time::SystemTime;

use lib::futures::sync::mpsc;

use lib::tokio::io;
use lib::tokio::prelude::*;

use super::state::StateHandle;
use super::{C2GSender, Codec, G2CReceiver};

/// A future that processes the broadcast logic for a connection
pub struct Peer {
    /// The TCP socket wrapped with the `Lines` codec.
    lines: Codec,

    /// Handle to the shared chat state.
    state: StateHandle,

    /// Transmission halt of the game channel
    game: C2GSender,

    /// Receive half of the message channel
    ///
    /// This is used to received messages from peers. When a message is received
    /// off of this `Rx`, it will be written to the socket.
    rx: G2CReceiver,

    /// Client socket address.
    ///
    /// The socket address is used as the key in the `peers` HashMap. The
    /// address is saved so that the `Peer` drop implementation can clean up its
    /// entry.
    addr: SocketAddr,
}

impl Peer {
    pub fn new(state: StateHandle, game: C2GSender, lines: Codec) -> Self {
        // Get the client socket address
        let addr = lines.peer_addr().unwrap();

        // Create a channel for this peer
        let (tx, rx) = mpsc::unbounded();

        // Add an entry for this `Peer` in the shared state map.
        state.lock().unwrap().peers.insert(addr, tx);

        Peer {
            lines,
            state,
            game,
            rx,
            addr,
        }
    }
}

impl Drop for Peer {
    fn drop(&mut self) {
        self.state.lock().unwrap().peers.remove(&self.addr);
    }
}

impl Future for Peer {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        // Receive all messages from peers.

        // Polling an `UnboundedReceiver` cannot fail, so `unwrap`
        // here is safe.
        while let Async::Ready(Some(v)) = self.rx.poll().unwrap() {
            // Buffer the line. Once all lines are buffered,
            // they will be flushed to the socket (right
            // below).
            self.lines.buffer(&v)?;
        }

        // Flush the write buffer to the socket
        let _ = self.lines.poll_flush()?;

        // Read new lines from the socket
        while let Async::Ready(line) = self.lines.poll()? {
            debug!("Received line {:?}", line);

            if let Some(message) = line {
                if let Client::Ping(t) = message {
                    let response = Server::Pong {
                        client: t,
                        server: SystemTime::now(),
                    };

                    self.state.lock().unwrap().peers[&self.addr]
                        .unbounded_send(response)
                        .unwrap();
                } else {
                    self.game.send((message, self.addr)).unwrap();
                }
            } else {
                // EOF was reached. The remote client has disconnected.
                // There is nothing more to do.
                return Ok(Async::Ready(()));
            }
        }

        // As always, it is important to not just return `NotReady`
        // without ensuring an inner future also returned `NotReady`.
        //
        // We know we got a `NotReady` from either `self.rx` or
        // `self.lines`, so the contract is respected.
        Ok(Async::NotReady)
    }
}
