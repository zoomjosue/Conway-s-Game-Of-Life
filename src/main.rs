use minifb::{Key, KeyRepeat, Scale, ScaleMode, Window, WindowOptions};

use conway_game_of_life::life::render;
use conway_game_of_life::{
    new_seeded_framebuffer, reset_world, ALIVE_COLOR, FRAMEBUFFER_HEIGHT, FRAMEBUFFER_WIDTH,
};

fn main() {
    let mut framebuffer = new_seeded_framebuffer();

    let mut window = Window::new(
        "Conway's Game of Life",
        FRAMEBUFFER_WIDTH,
        FRAMEBUFFER_HEIGHT,
        WindowOptions {
            scale: Scale::X4,
            scale_mode: ScaleMode::Stretch,
            ..WindowOptions::default()
        },
    )
    .expect("No se pudo crear la ventana");

    window.set_target_fps(11);

    let mut paused = false;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_pressed(Key::Space, KeyRepeat::No) {
            paused = !paused;
        }

        if window.is_key_pressed(Key::R, KeyRepeat::No) {
            reset_world(&mut framebuffer);
        }

        if !paused {
            render(&mut framebuffer, ALIVE_COLOR);
        }

        window
            .update_with_buffer(&framebuffer.buffer, FRAMEBUFFER_WIDTH, FRAMEBUFFER_HEIGHT)
            .expect("No se pudo actualizar la ventana");
    }
}
