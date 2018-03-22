extern crate flexi_logger;
pub extern crate some_platformer_lib;
#[macro_use]
extern crate log;

mod sync;
mod game;

use sync::peer::Peer;
use sync::{CTx, Codec};
use sync::shared::{Shared, SharedHandle};

use game::Game;

pub use some_platformer_lib as lib;

use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};
use std::thread;
use std::sync::mpsc;

use lib::tokio::net::{TcpListener, TcpStream};
use lib::tokio::prelude::*;

use flexi_logger::Logger;

fn main() {
    Logger::with_env_or_str("some_platformer_lib=debug,some_platformer_server=debug")
        .start()
        .unwrap_or_else(|e| panic!("Logger initialization failed with {}", e));

    let state = Arc::new(Mutex::new(Shared::new()));

    let (sender, receiver) = mpsc::channel();

    let game = Game::new(receiver, state.clone());
    thread::spawn(|| game_loop(game));

    let addr = "0.0.0.0:3000".parse().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();

    let server = listener
        .incoming()
        .for_each(move |socket| {
            debug!("accepted socket; addr={:?}", socket.peer_addr().unwrap());

            process(socket, state.clone(), sender.clone());

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
    lib::tokio::run(server);
}

/// Builds a new task for the incoming stream
fn process(socket: TcpStream, state: SharedHandle, sender: CTx) {
    // Wrap the socket with the `Lines` codec that we wrote above
    let lines = Codec::new(socket);

    let peer = Peer::new(state, sender, lines).map_err(|err| {
        error!("error: {:?}", err);
        ()
    });

    // Spawn the task
    lib::tokio::spawn(peer);
}

fn game_loop(mut game: Game) {
    let frame_budget = Duration::new(1, 0) / 60;

    let mut last_frame = SystemTime::now();
    let mut last_start_frame = last_frame;
    loop {
        let end_frame = last_frame + frame_budget;
        let start_frame = SystemTime::now();

        game.update(
            start_frame
                .duration_since(last_start_frame)
                .unwrap_or_default(),
        );

        let sleep_time = end_frame
            .duration_since(SystemTime::now())
            .unwrap_or_default();

        last_frame = end_frame;
        last_start_frame = start_frame;
        thread::sleep(sleep_time);
    }
}
