use nalgebra::Translation2;
use specs::{Component, Join, VecStorage};

/// A component that allows an entity to move
pub struct Moving {
	/// The current velocity of the moving object
	pub velocity: Translation2<f32>,
}

impl Moving {
	/// Creates a new Moving component - having its velocity set at 0
	pub fn new() -> Self {
		Moving {
			velocity: Translation2::new(0., 0.),
		}
	}

	/// Applies a force to the moving entity
	pub fn apply_force(&mut self, force: Translation2<f32>) {
		self.velocity.vector.x += force.vector.x;
		self.velocity.vector.y += force.vector.y;
	}
}

impl Component for Moving {
	type Storage = VecStorage<Self>;
}

/// A component that makes an entity affected by the gravity
pub struct GravityAffected {}

impl GravityAffected {
	/// Creates a new GravityAffected marker
	pub fn new() -> Self {
		GravityAffected {}
	}
}

impl Component for GravityAffected {
	type Storage = VecStorage<Self>;
}