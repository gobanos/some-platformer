use specs::World;

// Trait implemented by all the entities of the game.
pub trait GameEntity {
    // Allows the world to register the GameEntity as a SPECS entity, thus adding it to the
    // simulation.
    fn add_to_world(&mut self, world: &mut World);
}
