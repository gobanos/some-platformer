use collision::collision_handling::{CollisionHandler, CollisionLayer};
use components::collider::Collider;
use components::rect_drawable::RectDrawable;
use components::transform::Transform;
use entities::game_entity::GameEntity;
use ggez::graphics::Color;
use nalgebra::{Isometry2, Point2, Vector2};
use ncollide::shape::{Cuboid2, ShapeHandle2};
use ncollide::world::{CollisionObjectHandle, GeometricQueryType};
use specs::{Entity, World};

pub struct TestBlock {
	entity: Option<Entity>,
}

impl TestBlock {
	pub fn new() -> Self {
		TestBlock {
			entity: None,
		}
	}
}

impl GameEntity for TestBlock {
	fn add_to_world(&mut self, world: &mut World) {
		let isometry: Isometry2<f32> = Isometry2::new(Vector2::new(100., 400.), 0.);

		let width: f32 = 32.;
		let height: f32 = 32.;

		let shape: ShapeHandle2<f32> = ShapeHandle2::new(Cuboid2::new(Vector2::new(width, height)));

		let collision_object_handle: Option<CollisionObjectHandle>;
		{
			let mut collision_handler = world.write_resource::<CollisionHandler>();
			let collision_group = collision_handler.get_collision_group(CollisionLayer::Normal);

			let coh = collision_handler.world.add(
				isometry,
				shape,
				collision_group,
				GeometricQueryType::Contacts(0., 0.),
				(),
			);

			collision_object_handle = Some(coh);
		}

		let entity: Entity = world
			.create_entity()
			// TODO: remove Hardcoded position
			.with(Transform::new(
				isometry,
				Point2::new(width, height),
			))
			.with(RectDrawable::new(Color::new(1., 0., 0., 1.)))
			.with(Collider::new(collision_object_handle.unwrap()))
			.build();

		self.entity = Some(entity);
	}
}