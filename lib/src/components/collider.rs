use ncollide::world::CollisionObjectHandle;
use specs::{Component, VecStorage};

pub struct Collider {
    pub collision_object_handle: CollisionObjectHandle,
}

impl Component for Collider {
    type Storage = VecStorage<Self>;
}

impl Collider {
    pub fn new(handle: CollisionObjectHandle) -> Self {
        Collider {
            collision_object_handle: handle,
        }
    }
}
