extern crate some_platformer_lib as lib;

extern crate flexi_logger;
#[macro_use]
extern crate log;

mod sync;
use sync::{CTx, Codec};
use sync::peer::Peer;
use sync::state::{State, StateHandle};

mod game;
use game::Game;

use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::time::{Duration, SystemTime};
use std::thread;

use lib::tokio::net::{TcpListener, TcpStream};
use lib::tokio::prelude::*;

use flexi_logger::Logger;

fn main() {
    // Setup logger to debug by default (can be overwritten by `RUST_LOG` env variable
    Logger::with_env_or_str("some_platformer_lib=debug,some_platformer_server=debug")
        .start()
        .expect("Logger initialization failed");

    // Initialize the game state (the struct holding the connected players)
    // and wrap it with Arc/Mutex, for thread sync.
    let state = Arc::new(Mutex::new(State::new()));

    // Create the peer -> game channel
    let (sender, receiver) = mpsc::channel();

    // Make the game and spawn it to a new (dedicated) thread
    let game = Game::new(receiver, state.clone());
    thread::spawn(|| game_loop(game));

    // Open a TCP listener on port 3000, allowing all connections
    let addr = "0.0.0.0:3000".parse().expect("invalid addr");
    let listener = TcpListener::bind(&addr).expect("failed to bind port, maybe try another ?");

    // Setup server logic: on each new connection, we launch a new task
    // handling communication with the client
    let server = listener
        .incoming()
        .for_each(move |socket| {
            debug!("accepted socket; addr={:?}", socket.peer_addr().unwrap());

            spawn_peer(socket, state.clone(), sender.clone());
            Ok(())
        })
        .map_err(|err| {
            // All task must return a `()` error type
            // to force error handling
            error!("accept error = {:?}", err);
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
    lib::tokio::run(server);
}

/// Builds a new task for the incoming stream
/// the task will live until client disconnect
/// and will handle/forward client messages
fn spawn_peer(socket: TcpStream, state: StateHandle, sender: CTx) {
    // Wrap the socket with the `Lines` codec
    // which will encode/decode message for and from the client
    let lines = Codec::new(socket);

    // Create the peer to manage the client logic
    let peer = Peer::new(state, sender, lines).map_err(|err| {
        error!("peer error = {:?}", err);
        ()
    });

    // Spawn the task on tokio executor
    lib::tokio::spawn(peer);
}

/// Run the game loop
fn game_loop(mut game: Game) {
    // The `maximum` duration for a frame
    let frame_budget = Duration::new(1, 0) / 60;

    // Start time of the last frame
    let mut last_frame_start = SystemTime::now();

    // End time of the last frame
    let mut last_frame_end = last_frame_start;

    loop {
        // Compute current frame start & end (based on last frame)
        let frame_start = SystemTime::now();
        let frame_end = last_frame_end + frame_budget;

        game.update(
            frame_start
                .duration_since(last_frame_start)
                .unwrap_or_default(),
        );

        // Update last frame infos
        last_frame_end = frame_end;
        last_frame_start = frame_start;

        // Sleep until the computed frame end
        let sleep_time = frame_end
            .duration_since(SystemTime::now())
            .unwrap_or_default();
        thread::sleep(sleep_time);
    }
}
