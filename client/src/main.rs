extern crate flexi_logger;
extern crate ggez;
#[macro_use]
extern crate log;
extern crate some_platformer_lib;

use bytes::{BufMut, Bytes};
use flexi_logger::Logger;
use futures::sync::mpsc as ampsc;
use ggez::{conf, Context, event, GameResult, graphics};
use ggez::event::{Keycode, Mod};
use ggez::graphics::{Color, DrawMode, Rect};
use some_platformer_lib::{bytes, futures, tokio};
use some_platformer_lib::entities::player::Player;
use some_platformer_lib::entities::test_block::TestBlock;
use some_platformer_lib::Map;
use some_platformer_lib::sync::codec::Lines;
use some_platformer_lib::world::gameworld::GameWorld;
use std::{env, path};
use std::sync::mpsc as smpsc;
use std::thread;
use tokio::io;
use tokio::net::TcpStream;
use tokio::prelude::*;

/// Shorthand for the transmit half of the message channel
type ATx = ampsc::UnboundedSender<Bytes>;

/// Shorthand for the receive half of the message channel
type ARx = ampsc::UnboundedReceiver<Bytes>;

/// Shorthand for the transmit half of the message channel
type STx = smpsc::Sender<Bytes>;

/// Shorthand for the receive half of the message channel
type SRx = smpsc::Receiver<Bytes>;

struct MainState<'a, 'b> {
	map: Map,
	world: GameWorld<'a, 'b>,
	tx: ATx,
	rx: SRx,
}

impl<'a, 'b> ggez::event::EventHandler for MainState<'a, 'b> {
	fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
		// Poll sync messages
		while let Ok(msg) = self.rx.try_recv() {
			println!("game got message {:?}", msg);
		}

		self.world.update();
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
		graphics::clear(ctx);

		// TODO: Create a `TileRenderer` component, handle the map elsewhere :)
		// draw map
		graphics::set_color(ctx, Color::from_rgb(255, 0, 0))?;
		for (&(x, y), _) in &self.map.elements {
			graphics::rectangle(
				ctx,
				DrawMode::Fill,
				// draw for -10 to 10 -> 40px per block
				Rect::new((x + 10) as f32 * 40.0, (14 - y) as f32 * 40.0, 40.0, 40.0),
			)?;
		}

		// draws the RenderSystem
		self.world.draw(ctx);

		graphics::present(ctx);
		Ok(())
	}

	/// A keyboard button was pressed.
	fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
		match keycode {
			Keycode::Escape => ctx.quit().expect("Should never fail"),
			Keycode::Return => self.tx.unbounded_send(Bytes::from("SPAWN\r\n")).unwrap(),
			_ => (),
		}
	}
}

/// A future that processes the broadcast logic for a connection
struct Peer {
	/// The TCP socket wrapped with the `Lines` codec.
	lines: Lines,

	/// Send half of the message channel
	///
	/// This is used to send messages to game.
	tx: STx,

	/// Receive half of the message channel
	///
	/// This is used to received messages from game. When a message is received
	/// off of this `ARx`, it will be written to the socket.
	rx: ARx,
}

impl Peer {
	fn new(lines: Lines, tx: STx, rx: ARx) -> Self {
		Peer { lines, tx, rx }
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

				self.tx.send(line).unwrap();
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
	Logger::with_env_or_str("some_platformer=warn")
		.start()
		.unwrap_or_else(|e| panic!("Logger initialization failed with {}", e));

	let c = conf::Conf::new();
	let ctx = &mut Context::load_from_conf("some_platformer", "gobanos", c).unwrap();

	// We add the CARGO_MANIFEST_DIR/resources do the filesystems paths so
	// we we look in the cargo project for files.
	if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
		let mut path = path::PathBuf::from(manifest_dir);
		path.push("resources");
		ctx.filesystem.mount(&path, true);
	}

	info!("{}", graphics::get_renderer_info(ctx).unwrap());

	let mut game_world: GameWorld = GameWorld::new();

	// TODO: Remove player being instantiated here ...
	let mut player: Player = Player::new();
	game_world.add_game_entity(&mut player);

	let mut test_block: TestBlock = TestBlock::new();
	game_world.add_game_entity(&mut test_block);

	// sync to game uses sync channel
	let (sync_sender, game_receiver) = smpsc::channel();

	// game to sync uses async channel
	let (game_sender, sync_receiver) = ampsc::unbounded();

	thread::spawn(move || sync(sync_sender, sync_receiver));

	let state = &mut MainState {
		map: some_platformer_lib::Map::default(),
		world: game_world,
		tx: game_sender,
		rx: game_receiver,
	};

	event::run(ctx, state).unwrap();
}

fn sync(sender: STx, receiver: ARx) {
	let addr = "127.0.0.1:3000".parse().unwrap();

	let stream = TcpStream::connect(&addr).then(|stream| {
		match stream {
			Ok(socket) => process(socket, sender, receiver),
			Err(err) => error!("failed to connect to server: {:?}", err),
		}
		Ok(())
	});

	tokio::run(stream);
}

fn process(socket: TcpStream, tx: STx, rx: ARx) {
	// Wrap the socket with the `Lines` codec that we wrote above
	let lines = Lines::new(socket);

	let connection = Peer::new(lines, tx, rx).map_err(|err| {
		error!("failed to read line: {:?}", err);
		()
	});

	// Spawn the task
	tokio::spawn(connection);
}
