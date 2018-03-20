extern crate specs;

use components::transform::Transform;
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

    // Draws the RectDrawable to the context
    // TODO: Actually draw something on the context
    // TODO: Move to client
//    pub fn draw(&self, ctx: &mut Context, tr: &Transform) -> GameResult<()> {
//        set_color(ctx, self.color)?;
//        rectangle(ctx, DrawMode::Fill, tr.as_rect())
//    }
}

impl Component for RectDrawable {
    type Storage = VecStorage<Self>;
}
