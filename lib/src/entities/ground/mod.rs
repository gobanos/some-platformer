use components::rect_drawable::RectDrawable;
use components::transform::Transform;
use entities::game_entity::GameEntity;
use nalgebra::{Point2, Vector2};
use specs::{Entity, World};
use types::Color;

pub struct GroundEntity(Entity);

pub struct Ground {
    position: Point2<f32>,
    size: Point2<f32>,
    color: Color,
}

impl Default for Ground {
    fn default() -> Self {
        Ground {
            position: Point2::new(0.0, 0.0),
            size: Point2::new(32.0, 32.0),
            color: Color::new(1.0, 0.0, 0.0, 1.0),
        }
    }
}

impl Ground {
    pub fn new(position: Point2<f32>, size: Point2<f32>, color: Color) -> Self {
        Ground {
            position,
            size,
            color,
        }
    }
}

impl GameEntity for Ground {
    type Entity = GroundEntity;

    fn add_to_world(self, world: &mut World) -> Self::Entity {
        let entity: Entity = world
            .create_entity()
            .with(Transform::new(
                Vector2::new(self.position.x, self.position.y),
                self.size,
            ))
            .with(RectDrawable::new(self.color))
            .build();

        GroundEntity(entity)
    }
}
