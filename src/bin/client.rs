extern crate flexi_logger;
extern crate ggez;
#[macro_use]
extern crate log;
extern crate some_platformer;

use flexi_logger::Logger;
use ggez::{conf, Context, event, GameResult, graphics};
use ggez::graphics::{Color, DrawMode, Rect};
use some_platformer::Map;
use some_platformer::world::gameworld::GameWorld;
use std::{env, path};

struct MainState {
	map: Map,
}

impl ggez::event::EventHandler for MainState {
	fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
		graphics::clear(ctx);

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

	let state = &mut MainState {
		map: some_platformer::Map::default(),
	};

	event::run(ctx, state).unwrap();
}
