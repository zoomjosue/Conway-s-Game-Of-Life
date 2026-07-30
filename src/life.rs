use crate::framebuffer::Framebuffer;

pub fn render(framebuffer: &mut Framebuffer, alive_color: u32) {
    let mut next_frame = vec![framebuffer.background_color(); framebuffer.buffer.len()];

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let alive = is_alive(framebuffer, x as isize, y as isize, alive_color);
            let neighbors = count_live_neighbors(framebuffer, x as isize, y as isize, alive_color);

            let survives = alive && (neighbors == 2 || neighbors == 3);
            let is_born = !alive && neighbors == 3;

            if survives || is_born {
                next_frame[y * framebuffer.width + x] = alive_color;
            }
        }
    }

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            framebuffer.set_current_color(next_frame[y * framebuffer.width + x]);
            framebuffer.point(x, y);
        }
    }
}

fn is_alive(framebuffer: &Framebuffer, x: isize, y: isize, alive_color: u32) -> bool {
    framebuffer.get_color(x, y) == alive_color
}

fn count_live_neighbors(framebuffer: &Framebuffer, x: isize, y: isize, alive_color: u32) -> usize {
    let mut neighbors = 0;

    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }

            if is_alive(framebuffer, x + dx, y + dy, alive_color) {
                neighbors += 1;
            }
        }
    }

    neighbors
}

#[cfg(test)]
mod tests {
    use super::render;
    use crate::framebuffer::Framebuffer;

    const DEAD: u32 = 0x000000;
    const ALIVE: u32 = 0xFFFFFF;

    fn framebuffer(width: usize, height: usize, cells: &[(usize, usize)]) -> Framebuffer {
        let mut framebuffer = Framebuffer::new(width, height);
        framebuffer.set_background_color(DEAD);
        framebuffer.clear();
        framebuffer.set_current_color(ALIVE);

        for &(x, y) in cells {
            framebuffer.point(x, y);
        }

        framebuffer
    }

    #[test]
    fn live_cell_with_fewer_than_two_neighbors_dies() {
        let mut framebuffer = framebuffer(3, 3, &[(1, 1)]);

        render(&mut framebuffer, ALIVE);

        assert_eq!(framebuffer.get_color(1, 1), DEAD);
    }

    #[test]
    fn live_cell_with_two_neighbors_survives() {
        let mut framebuffer = framebuffer(3, 3, &[(1, 1), (0, 1), (2, 1)]);

        render(&mut framebuffer, ALIVE);

        assert_eq!(framebuffer.get_color(1, 1), ALIVE);
    }

    #[test]
    fn live_cell_with_more_than_three_neighbors_dies() {
        let mut framebuffer = framebuffer(3, 3, &[(1, 1), (0, 1), (2, 1), (1, 0), (1, 2)]);

        render(&mut framebuffer, ALIVE);

        assert_eq!(framebuffer.get_color(1, 1), DEAD);
    }

    #[test]
    fn dead_cell_with_exactly_three_neighbors_is_born() {
        let mut framebuffer = framebuffer(3, 3, &[(0, 1), (1, 0), (2, 1)]);

        render(&mut framebuffer, ALIVE);

        assert_eq!(framebuffer.get_color(1, 1), ALIVE);
    }
}
