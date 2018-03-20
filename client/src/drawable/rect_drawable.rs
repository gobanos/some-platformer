use super::Drawable;
use some_platformer_lib::components::rect_drawable::RectDrawable;
use some_platformer_lib::components::transform::Transform;

use some_platformer_lib::types;

use ggez::{Context, GameResult};
use ggez::graphics::{set_color, rectangle, DrawMode, Color, Rect};

impl Drawable for RectDrawable {
    fn draw(&self, ctx: &mut Context, transform: &Transform) -> GameResult<()> {
        set_color(ctx, to_color(self.color))?;
        rectangle(ctx, DrawMode::Fill, to_rect(transform.as_rect()))
    }
}

fn to_color(color: types::Color) -> Color {
    Color::new(color.x, color.y, color.z, color.w)
}

fn to_rect(rect: types::Rect) -> Rect {
    Rect::new(rect.x, rect.y, rect.z, rect.w)
}