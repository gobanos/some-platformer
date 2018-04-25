use components::moving::{GravityAffected, Moving};
use components::transform::Transform;
use nalgebra::{Translation2, Vector2};
use resources::delta_time::DeltaTime;
use specs::{Fetch, Join, ReadStorage, System, WriteStorage};

// A system updating the transform of a moving entity
pub struct SysMoving {}

impl<'a> System<'a> for SysMoving {
    type SystemData = (WriteStorage<'a, Transform>, ReadStorage<'a, Moving>);

    // Applies the system to change transform components from moving objects
    fn run(&mut self, (mut transform, moving): Self::SystemData) {
        for (tr, mov) in (&mut transform, &moving).join() {
            // Change the position of the transform
            tr.isometry
                .append_translation_mut(&Translation2::from_vector(mov.velocity.vector));
        }
    }
}

// A system updating the moving component of an entity affected by gravity
pub struct SysMovingGravity {
    gravity_vec: Vector2<f32>,
}

impl SysMovingGravity {
    pub fn new() -> Self {
        SysMovingGravity {
            gravity_vec: Vector2::new(0., 9.81),
        }
    }

    pub fn new_custom(x: f32, y: f32) -> Self {
        SysMovingGravity {
            gravity_vec: Vector2::new(x, y),
        }
    }
}

impl<'a> System<'a> for SysMovingGravity {
    type SystemData = (
        WriteStorage<'a, Moving>,
        ReadStorage<'a, GravityAffected>,
        Fetch<'a, DeltaTime>,
    );

    fn run(&mut self, (mut moving, gravity_affected, delta_time): Self::SystemData) {
        for (mov, _gravity) in (&mut moving, &gravity_affected).join() {
            // Change the velocity of the moving object
            mov.velocity.vector.x += self.gravity_vec.x;
            mov.velocity.vector.y += self.gravity_vec.y * delta_time.delta_ms;
        }
    }
}
