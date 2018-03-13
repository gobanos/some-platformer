use specs::{Dispatcher, DispatcherBuilder, World};

// The basic struct of the game. Contains everything to simulate an instance of the game.
pub struct GameWorld<'a, 'b> {
	// SPECS's world
	entity_world: World,
	dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> GameWorld<'a, 'b> {
	// Creates a new instance of the GameWorld
	pub fn new() -> Self {
		let dispatcher: Dispatcher = DispatcherBuilder::new()
			.build();

		GameWorld {
			entity_world: World::new(),
			dispatcher,
		}
	}
}