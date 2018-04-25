use specs::World;

// Trait implemented by all the entities of the game.
pub trait GameEntity {
    type Entity;

    // Allows the world to register the GameEntity as a SPECS entity, thus adding it to the
    // simulation.
    fn add_to_world(self, world: &mut World) -> Self::Entity;
}
