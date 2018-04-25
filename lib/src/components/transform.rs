use nalgebra::{Isometry2, Point2, Vector2};
use specs::{Component, VecStorage};
use types::Rect;

// Component that handles the position/size/rotation of a game entity
pub struct Transform {
    pub isometry: Isometry2<f32>,
    pub size: Point2<f32>,
}

impl Transform {
    pub fn new_with_isometry(isometry: Isometry2<f32>, size: Point2<f32>) -> Self {
        Transform { isometry, size }
    }

    pub fn new(position: Vector2<f32>, size: Point2<f32>) -> Self {
        Transform {
            isometry: Isometry2::new(position, 0.),
            size,
        }
    }

    pub fn new_empty() -> Self {
        Transform {
            isometry: Isometry2::new(Vector2::new(0., 0.), 0.),
            size: Point2::new(0., 0.),
        }
    }

    pub fn as_rect(&self) -> Rect {
        Rect::new(
            self.isometry.translation.vector.x - self.size.x / 2.,
            self.isometry.translation.vector.y - self.size.y / 2.,
            self.size.x,
            self.size.y,
        )
    }
}

impl Component for Transform {
    type Storage = VecStorage<Self>;
}
