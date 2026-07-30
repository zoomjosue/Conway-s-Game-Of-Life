use std::fs::File;
use std::path::Path;

use gif::{Encoder, Frame, Repeat};

use conway_game_of_life::life::render;
use conway_game_of_life::{
    new_seeded_framebuffer, ALIVE_COLOR, DEAD_COLOR, FRAMEBUFFER_HEIGHT, FRAMEBUFFER_WIDTH,
};

const SCALE: usize = 5;
const FRAMES: usize = 90;
const FRAME_DELAY_CS: u16 = 9;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output_path = Path::new("docs/conway-preview.gif");
    std::fs::create_dir_all(output_path.parent().unwrap())?;

    let mut framebuffer = new_seeded_framebuffer();
    let mut output = File::create(output_path)?;

    let palette = [
        red(DEAD_COLOR),
        green(DEAD_COLOR),
        blue(DEAD_COLOR),
        red(ALIVE_COLOR),
        green(ALIVE_COLOR),
        blue(ALIVE_COLOR),
    ];

    let gif_width = (FRAMEBUFFER_WIDTH * SCALE) as u16;
    let gif_height = (FRAMEBUFFER_HEIGHT * SCALE) as u16;
    let mut encoder = Encoder::new(&mut output, gif_width, gif_height, &palette)?;
    encoder.set_repeat(Repeat::Infinite)?;

    for _ in 0..FRAMES {
        let pixels = scaled_indexed_pixels(&framebuffer.buffer);
        let mut frame = Frame::default();
        frame.width = gif_width;
        frame.height = gif_height;
        frame.delay = FRAME_DELAY_CS;
        frame.buffer = pixels.into();
        encoder.write_frame(&frame)?;

        render(&mut framebuffer, ALIVE_COLOR);
    }

    println!("GIF generado en {}", output_path.display());
    Ok(())
}

fn scaled_indexed_pixels(buffer: &[u32]) -> Vec<u8> {
    let gif_width = FRAMEBUFFER_WIDTH * SCALE;
    let gif_height = FRAMEBUFFER_HEIGHT * SCALE;
    let mut pixels = vec![0; gif_width * gif_height];

    for y in 0..gif_height {
        for x in 0..gif_width {
            let source_x = x / SCALE;
            let source_y = y / SCALE;
            let color = buffer[source_y * FRAMEBUFFER_WIDTH + source_x];
            pixels[y * gif_width + x] = if color == ALIVE_COLOR { 1 } else { 0 };
        }
    }

    pixels
}

fn red(color: u32) -> u8 {
    ((color >> 16) & 0xFF) as u8
}

fn green(color: u32) -> u8 {
    ((color >> 8) & 0xFF) as u8
}

fn blue(color: u32) -> u8 {
    (color & 0xFF) as u8
}
