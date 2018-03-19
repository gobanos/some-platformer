use components::moving::{GravityAffected, Moving};
use components::rect_drawable::RectDrawable;
use components::transform::Transform;
use entities::game_entity::GameEntity;
use ggez::graphics::Color;
use nalgebra::Point2;
use specs::{Entity, World};

pub struct Player {
    entity: Option<Entity>,
}

impl Player {
    pub fn new() -> Self {
        Player { entity: None }
    }
}

impl GameEntity for Player {
    fn add_to_world(&mut self, world: &mut World) {
        let entity: Entity = world
			.create_entity()
			// TODO: remove Hardcoded position
			.with(Transform::new(
				Point2::new(100., 100.),
				Point2::new(32., 32.),
				0.,
			))
			.with(RectDrawable::new(Color::new(0., 1., 0., 1.)))
			.with(Moving::new())
			.with(GravityAffected::new())
			.build();

        self.entity = Some(entity);
    }
}
