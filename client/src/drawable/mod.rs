use ggez::{Context, GameResult};

use lib::components::transform::Transform;

mod rect_drawable;

pub trait Drawable {
    fn draw(&self, ctx: &mut Context, transform: &Transform) -> GameResult<()>;
}
