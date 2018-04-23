use components::moving::{GravityAffected, Moving};
use components::rect_drawable::RectDrawable;
use components::transform::Transform;
use entities::game_entity::GameEntity;
use nalgebra::{Point2, Vector2};
use specs::{Entity, World};
use types::Color;

pub struct PlayerEntity(Entity);

pub struct Player {
	position: Point2<f32>,
	size: Point2<f32>,
	color: Color,
}

impl Default for Player {
	fn default() -> Self {
		Player {
			position: Point2::new(200.0, 100.0),
			size: Point2::new(32.0, 32.0),
			color: Color::new(0.0, 1.0, 0.0, 1.0),
		}
	}
}

impl Player {
	pub fn new(position: Point2<f32>, size: Point2<f32>, color: Color) -> Self {
		Player {
			position,
			size,
			color,
		}
	}
}

impl GameEntity for Player {
	type Entity = PlayerEntity;

	fn add_to_world(self, world: &mut World) -> Self::Entity {
		let entity: Entity = world
			.create_entity()
			.with(Transform::new(
				Vector2::new(self.position.x, self.position.y),
				self.size,
			))
			.with(RectDrawable::new(self.color))
			.with(Moving::new())
			.with(GravityAffected::new())
			.build();

		PlayerEntity(entity)
	}
}
