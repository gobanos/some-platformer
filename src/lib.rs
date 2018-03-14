// TODO: Handle the ggez import in a better way. Server shouldn't need the ggez dependency!
extern crate ggez;
extern crate nalgebra;
extern crate specs;
extern crate time;

use std::collections::HashMap;

pub mod world;
pub mod entities;
pub mod components;
pub mod systems;
pub mod resources;

// TODO: move this out of lib.rs
const DEBUG_MAP: &[(i32, i32)] = &[
	(-6, 4), // stair
	(-6, 3),
	(-5, 3),
	(-5, 2),
	(-4, 2),
	(-4, 1),
	(-3, 1),
	(-3, 0),
	(-2, 0), // ground
	(-1, 0),
	(0, 0),
	(1, 0),
	(2, 0),
	(3, 0),
	(4, 0),
	(5, 0),
	(5, 1), // wall
	(5, 2),
	(5, 3),
	(5, 4),
];

#[derive(Debug, Clone)]
pub struct Block {}

#[derive(Debug, Clone)]
pub struct Map {
	pub elements: HashMap<(i32, i32), Block>,
}

impl Default for Map {
	// debug map
	fn default() -> Self {
		Map {
			elements: DEBUG_MAP.iter().map(|&p| (p, Block {})).collect(),
		}
	}
}
