extern crate specs;

use specs::Component;
use specs::VecStorage;

use types::Color;

// A RectDrawable component allows an entity to be drawn as a rectangle to the screen. This is
// convenient for quick debugging and prototype implementation of something.
pub struct RectDrawable {
    pub color: Color,
}

impl RectDrawable {
    // Creates a new instance of the RectDrawable component, given the color.
    pub fn new(color: Color) -> Self {
        RectDrawable { color }
    }
}

impl Component for RectDrawable {
    type Storage = VecStorage<Self>;
}
