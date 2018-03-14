extern crate specs;

use components::transform::Transform;
use ggez::Context;
use ggez::graphics::{rectangle, set_color, Color, DrawMode};
use specs::Component;
use specs::VecStorage;

// A RectDrawable component allows an entity to be drawn as a rectangle to the screen. This is
// convenient for quick debugging and prototype implementation of something.
pub struct RectDrawable {
    color: Color,
}

impl RectDrawable {
    // Creates a new instance of the RectDrawable component, given the color.
    pub fn new(color: Color) -> Self {
        RectDrawable { color }
    }

    // Draws the RectDrawable to the context
    // TODO: Actually draw something on the context
    pub fn draw(&self, ctx: &mut Context, tr: &Transform) {
        set_color(ctx, self.color);
        rectangle(ctx, DrawMode::Fill, tr.as_rect());
    }
}

impl Component for RectDrawable {
    type Storage = VecStorage<Self>;
}
