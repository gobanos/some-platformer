use collision::collision_handling::CollisionHandler;
use components::collider::Collider;
use components::moving::{GravityAffected, Moving};
use components::rect_drawable::RectDrawable;
use components::transform::Transform;
use entities::game_entity::GameEntity;
use resources::delta_time::DeltaTime;
use specs::{Dispatcher, DispatcherBuilder, World};
use systems::sys_colliding::SysCollide;
use systems::sys_moving::{SysMoving, SysMovingGravity};

// The basic struct of the game. Contains everything to simulate an instance of the game.
pub struct GameWorld<'a, 'b> {
    // SPECS's world
    pub entity_world: World,
    // The dispatcher that contains all the logic systems of the game
    logic_dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> GameWorld<'a, 'b> {
    // Creates a new instance of the GameWorld
    pub fn new() -> Self {
        // Registers all the components in the World
        let mut world: World = World::new();
        world.register::<Transform>();
        world.register::<RectDrawable>();
        world.register::<Moving>();
        world.register::<GravityAffected>();
        world.register::<Collider>();

        let collision_handler: CollisionHandler = CollisionHandler::new();

        world.add_resource(DeltaTime::new());
        world.add_resource(collision_handler);

        // Creates the systems
        let sys_moving_gravity = SysMovingGravity::new();
        let sys_moving = SysMoving {};
        let sys_moving_collide = SysCollide {};

        // Creates the dispatcher, registering the systems
        let logic_dispatcher: Dispatcher = DispatcherBuilder::new()
			.add(sys_moving_gravity, "sys_moving_gravity", &[])
			// TODO: Add sys_moving_collision
			.add(sys_moving, "sys_moving", &["sys_moving_gravity"])
			.add(sys_moving_collide, "sys_moving_colliding", &["sys_moving"])
			.build();

        // Creates the actual GameWorld
        GameWorld {
            entity_world: world,
            logic_dispatcher,
        }
    }

    pub fn update(&mut self) {
        {
            let mut delta_time = self.entity_world.write_resource::<DeltaTime>();
            delta_time.update();
        }

        // Updates the game's logic
        self.logic_dispatcher.dispatch(&self.entity_world.res);
    }

    pub fn add_game_entity<T: GameEntity>(&mut self, entity: T) {
        entity.add_to_world(&mut self.entity_world);
    }
}
