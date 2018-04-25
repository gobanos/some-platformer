use ncollide::world::{CollisionGroups, CollisionWorld2};
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
pub enum CollisionLayer {
    Normal,
}

pub struct CollisionHandler {
    pub world: CollisionWorld2<f32, ()>,
    pub collision_groups: HashMap<CollisionLayer, CollisionGroups>,
}

impl CollisionHandler {
    pub fn new() -> Self {
        let collision_groups: HashMap<CollisionLayer, CollisionGroups> = HashMap::new();

        CollisionHandler {
            world: CollisionWorld2::new(0.02),
            collision_groups,
        }
    }

    pub fn get_collision_group(&mut self, layer: CollisionLayer) -> CollisionGroups {
        let mut must_include: bool = false;
        let collision_group = match self.collision_groups.get(&layer) {
            Some(group) => *group,
            None => {
                must_include = true;
                CollisionGroups::new()
            }
        };

        if must_include {
            self.collision_groups.insert(layer, collision_group);
        }

        return collision_group;
    }
}
