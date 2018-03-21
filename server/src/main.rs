extern crate flexi_logger;
extern crate some_platformer_lib;
#[macro_use]
extern crate log;

use some_platformer_lib::{futures, tokio};

use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;

use std::sync::{Arc, Mutex};

use flexi_logger::Logger;

mod sync;

use sync::peer::Peer;
use sync::Codec;
use sync::shared::Shared;

fn main() {
    Logger::with_env_or_str("some_platformer_lib=warn,some_platformer_server=warn")
        .start()
        .unwrap_or_else(|e| panic!("Logger initialization failed with {}", e));

    let state = Arc::new(Mutex::new(Shared::new()));

    let addr = "0.0.0.0:3000".parse().unwrap();
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

    info!("server running on port 3000");

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
    let lines = Codec::new(socket);

    let peer = Peer::new(state, lines).map_err(|err| {
        println!("error: {:?}", err);
        ()
    });

    // Spawn the task
    tokio::spawn(peer);
}
