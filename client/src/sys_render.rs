use lib::components::rect_drawable::RectDrawable;
use lib::components::transform::Transform;
use lib::specs::{Join, ReadStorage, System};

use ggez::Context;
use drawable::Drawable;

// The SysRender system draws all the RectDrawables with their Transforms on the screen
pub struct SysRender<'c> {
    ctx: &'c mut Context,
}

impl<'c> SysRender<'c> {
    pub fn new(context: &'c mut Context) -> Self {
        SysRender { ctx: context }
    }
}

impl<'a, 'c> System<'a> for SysRender<'c> {
    type SystemData = (ReadStorage<'a, Transform>, ReadStorage<'a, RectDrawable>);

    // Called when the system has to be executed
    fn run(&mut self, (transform, rect_drawable): Self::SystemData) {
        // Displays all the rect_drawable
        for (trans, r_draw) in (&transform, &rect_drawable).join() {
            // TODO: Actually display the drawable to the context
            r_draw.draw(self.ctx, trans).unwrap();
        }

        // TODO: Display all the drawable
    }
}
