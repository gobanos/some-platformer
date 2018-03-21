extern crate flexi_logger;
extern crate some_platformer_lib;
#[macro_use]
extern crate log;

mod sync;
mod game;

use sync::peer::Peer;
use sync::Codec;
use sync::shared::Shared;

use game::Game;

use some_platformer_lib::{futures, tokio};

use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};
use std::thread;

use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;


use flexi_logger::Logger;


fn main() {
    Logger::with_env_or_str("some_platformer_lib=warn,some_platformer_server=warn")
        .start()
        .unwrap_or_else(|e| panic!("Logger initialization failed with {}", e));

    let state = Arc::new(Mutex::new(Shared::new()));

    let game = Game::new();
    thread::spawn(|| game_loop(game));

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

fn game_loop(mut game: Game) {
    let frame_budget = Duration::new(1, 0) / 60;
    let mut last_frame = SystemTime::now();
    for i in 0u64.. {
        let end_frame = last_frame + frame_budget;
        let start_frame = SystemTime::now();

        game.update(start_frame);

        let game_updated = SystemTime::now();
        let sleep_time = end_frame.duration_since(game_updated).unwrap_or_default();

        if i % 200 == 0 {
            let frame_duration = game_updated.duration_since(start_frame).unwrap();
            let percentage = (1.0 - (frame_duration.subsec_nanos() as f32 / frame_budget.subsec_nanos() as f32)) * 100.0;
            let overflow = percentage < 0.0;
            info!("FRAME {} took {:?} ({:0.2}% {})", i, frame_duration,
                if overflow { -percentage } else { percentage },
                if overflow { "over" } else { "idle" },
            );
        }

        last_frame = end_frame;
        thread::sleep(sleep_time);
    }
}