use some_platformer_lib::sync::message::{Client, Server};

use std::time::SystemTime;
use std::net::SocketAddr;

use futures::sync::mpsc;

use tokio::io;
use tokio::prelude::*;

use super::shared::SharedHandle;
use super::Rx;
use super::Codec;

/// A future that processes the broadcast logic for a connection
pub struct Peer {
    /// The TCP socket wrapped with the `Lines` codec.
    lines: Codec,

    /// Handle to the shared chat state.
    state: SharedHandle,

    /// Receive half of the message channel
    ///
    /// This is used to received messages from peers. When a message is received
    /// off of this `Rx`, it will be written to the socket.
    rx: Rx,

    /// Client socket address.
    ///
    /// The socket address is used as the key in the `peers` HashMap. The
    /// address is saved so that the `Peer` drop implementation can clean up its
    /// entry.
    addr: SocketAddr,
}

impl Peer {
    pub fn new(state: SharedHandle, lines: Codec) -> Self {
        // Get the client socket address
        let addr = lines.peer_addr().unwrap();

        // Create a channel for this peer
        let (tx, rx) = mpsc::unbounded();

        // Add an entry for this `Peer` in the shared state map.
        state.lock().unwrap().peers.insert(addr, tx);

        Peer {
            lines,
            state,
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
            println!("Received line {:?}", line);

            if let Some(message) = line {
                // TODO: send message to server world,
                // then server world dispatch messages to all clients.
                let message = match message {
                    Client::Ping(t) => Server::Pong {
                        client: t,
                        server: SystemTime::now(),
                    },
                    Client::Test => Server::Test,
                };

                // Now, send the line to all other peers
                for tx in self.state.lock().unwrap().peers.values() {
                    // The send only fails if the rx half has been
                    // dropped, however this is impossible as the
                    // `tx` half will be removed from the map
                    // before the `rx` is dropped.
                    tx.unbounded_send(message.clone()).unwrap();
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