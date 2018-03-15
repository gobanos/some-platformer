extern crate flexi_logger;
extern crate ggez;
#[macro_use]
extern crate log;
extern crate futures;
extern crate some_platformer_lib;
extern crate tokio;

use flexi_logger::Logger;
use ggez::{conf, event, graphics, Context, GameResult};
use ggez::graphics::{Color, DrawMode, Rect};
use some_platformer_lib::entities::player::player::Player;
use some_platformer_lib::Map;
use some_platformer_lib::world::gameworld::GameWorld;
use std::{env, path};

use std::thread;
use std::sync::mpsc;

use tokio::net::TcpStream;
use tokio::prelude::*;

type SyncToGame = i32;
type GameToSync = i32;

struct MainState<'a, 'b> {
    map: Map,
    world: GameWorld<'a, 'b>,
}

impl<'a, 'b> ggez::event::EventHandler for MainState<'a, 'b> {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
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

    let (sync_sender, _game_receiver) = mpsc::channel();
    let (_game_sender, sync_receiver) = mpsc::channel();
    thread::spawn(move || sync(sync_sender, sync_receiver));

    let state = &mut MainState {
        map: some_platformer_lib::Map::default(),
        world: game_world,
    };

    event::run(ctx, state).unwrap();
}

fn sync(_sender: mpsc::Sender<SyncToGame>, _receiver: mpsc::Receiver<GameToSync>) {
    let addr = "127.0.0.1:3000".parse().unwrap();

    // Simulate connections with server
    loop {
        let stream = TcpStream::connect(&addr).then(|_stream| {
            // TODO: communicate with server !
            Ok(())
        });

        tokio::run(stream);

        thread::sleep_ms(10000);
    }
}
