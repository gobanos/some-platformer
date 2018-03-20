use some_platformer_lib::world::gameworld::GameWorld as GW;
use some_platformer_lib::specs::RunNow;
use ggez::Context;
use sys_render::SysRender;

use std::ops::{Deref, DerefMut};

pub struct GameWorld<'a, 'b>(GW<'a, 'b>);

impl<'a, 'b> GameWorld<'a, 'b> {
    pub fn new() -> Self {
        GameWorld(GW::new())
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        let mut render_system: SysRender = SysRender::new(ctx);
        render_system.run_now(&self.0.entity_world.res);
    }
}

impl<'a, 'b> Deref for GameWorld<'a, 'b> {
    type Target = GW<'a, 'b>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, 'b> DerefMut for GameWorld<'a, 'b> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}