extern crate flexi_logger;
extern crate some_platformer_lib;
#[macro_use]
extern crate log;

use some_platformer_lib::{bytes, futures, tokio};
use some_platformer_lib::sync::codec::Lines;

use tokio::io;
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;

use futures::sync::mpsc;

use bytes::{BufMut, Bytes};

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use flexi_logger::Logger;

/// Shorthand for the transmit half of the message channel
type Tx = mpsc::UnboundedSender<Bytes>;

/// Shorthand for the receive half of the message channel
type Rx = mpsc::UnboundedReceiver<Bytes>;

/// Shorthand for the shared handle to the state
type SharedHandle = Arc<Mutex<Shared>>;

/// The shared state, to allow task to communicate together
struct Shared {
    peers: HashMap<SocketAddr, Tx>,
}

impl Shared {
    fn new() -> Self {
        Shared {
            peers: HashMap::new(),
        }
    }
}

/// A future that processes the broadcast logic for a connection
struct Peer {
    /// The TCP socket wrapped with the `Lines` codec.
    lines: Lines,

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
    fn new(state: SharedHandle, lines: Lines) -> Self {
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
            self.lines.buffer(&v);
        }

        // Flush the write buffer to the socket
        let _ = self.lines.poll_flush()?;

        // Read new lines from the socket
        while let Async::Ready(line) = self.lines.poll()? {
            println!("Received line {:?}", line);

            if let Some(message) = line {
                // Append the peer's name to the front of the line:
                let mut line = message.clone();
                line.put("\r\n");

                // We're using `Bytes`, which allows zero-copy clones
                // (by storing the data in an Arc internally)?
                //
                // However, before cloning, we must freeze the data.
                // This converts it from mutable -> immutable?
                // allowing zero-copy cloning?
                let line = line.freeze();

                // Now, send the line to all other peers
                for (_addr, tx) in &self.state.lock().unwrap().peers {
                    // The send only fails if the rx half has been
                    // dropped, however this is impossible as the
                    // `tx` half will be removed from the map
                    // before the `rx` is dropped.
                    tx.unbounded_send(line.clone()).unwrap();
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

fn main() {
    Logger::with_env_or_str("some_platformer_lib=warn;server=warn")
        .start()
        .unwrap_or_else(|e| panic!("Logger initialization failed with {}", e));

    let state = Arc::new(Mutex::new(Shared::new()));

    let addr = "127.0.0.1:3000".parse().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();

    let server = listener
        .incoming()
        .for_each(move |socket| {
            debug!("accepted socket; addr={:?}", socket.peer_addr().unwrap());

            process(socket, state.clone());

            Ok(())
        })
        .map_err(|err| {
            debug!("accept error = {:?}", err);
        });

    info!("server running on localhost:3000");

    // Start the server
    //
    // This does a few things:
    //
    // * Start the Tokio runtime (reactor, threadpool, etc...)
    // * Spawns the `server` task onto the runtime.
    // * Blocks the current thread until the runtime becomes idle, i.e.
    //   spawned tasks have completed
    tokio::run(server);
}

/// Builds a new task for the incoming stream
fn process(socket: TcpStream, state: Arc<Mutex<Shared>>) {
    // Wrap the socket with the `Lines` codec that we wrote above
    let lines = Lines::new(socket);

    let peer = Peer::new(state, lines).map_err(|err| {
        println!("error: {:?}", err);
        ()
    });

    // Spawn the task
    tokio::spawn(peer);
}
