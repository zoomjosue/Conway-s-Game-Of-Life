pub mod framebuffer;
pub mod life;
pub mod patterns;

pub const FRAMEBUFFER_WIDTH: usize = 160;
pub const FRAMEBUFFER_HEIGHT: usize = 100;
pub const DEAD_COLOR: u32 = 0x001F3F;
pub const ALIVE_COLOR: u32 = 0xFF8C00;

use framebuffer::Framebuffer;
use patterns::seed_world;

pub fn new_seeded_framebuffer() -> Framebuffer {
    let mut framebuffer = Framebuffer::new(FRAMEBUFFER_WIDTH, FRAMEBUFFER_HEIGHT);
    reset_world(&mut framebuffer);
    framebuffer
}

pub fn reset_world(framebuffer: &mut Framebuffer) {
    framebuffer.set_background_color(DEAD_COLOR);
    framebuffer.clear();
    seed_world(framebuffer, ALIVE_COLOR);
}
