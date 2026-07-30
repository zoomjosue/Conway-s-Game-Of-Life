use crate::framebuffer::Framebuffer;

#[derive(Clone, Copy)]
pub struct Cell {
    pub x: isize,
    pub y: isize,
}

const BLOCK: &[Cell] = &[
    Cell { x: 0, y: 0 },
    Cell { x: 1, y: 0 },
    Cell { x: 0, y: 1 },
    Cell { x: 1, y: 1 },
];

const BEE_HIVE: &[Cell] = &[
    Cell { x: 1, y: 0 },
    Cell { x: 2, y: 0 },
    Cell { x: 0, y: 1 },
    Cell { x: 3, y: 1 },
    Cell { x: 1, y: 2 },
    Cell { x: 2, y: 2 },
];

const LOAF: &[Cell] = &[
    Cell { x: 1, y: 0 },
    Cell { x: 2, y: 0 },
    Cell { x: 0, y: 1 },
    Cell { x: 3, y: 1 },
    Cell { x: 1, y: 2 },
    Cell { x: 3, y: 2 },
    Cell { x: 2, y: 3 },
];

const BOAT: &[Cell] = &[
    Cell { x: 0, y: 0 },
    Cell { x: 1, y: 0 },
    Cell { x: 0, y: 1 },
    Cell { x: 2, y: 1 },
    Cell { x: 1, y: 2 },
];

const TUB: &[Cell] = &[
    Cell { x: 1, y: 0 },
    Cell { x: 0, y: 1 },
    Cell { x: 2, y: 1 },
    Cell { x: 1, y: 2 },
];

const BLINKER: &[Cell] = &[
    Cell { x: 1, y: 0 },
    Cell { x: 1, y: 1 },
    Cell { x: 1, y: 2 },
];

const TOAD: &[Cell] = &[
    Cell { x: 1, y: 0 },
    Cell { x: 2, y: 0 },
    Cell { x: 3, y: 0 },
    Cell { x: 0, y: 1 },
    Cell { x: 1, y: 1 },
    Cell { x: 2, y: 1 },
];

const BEACON: &[Cell] = &[
    Cell { x: 0, y: 0 },
    Cell { x: 1, y: 0 },
    Cell { x: 0, y: 1 },
    Cell { x: 3, y: 2 },
    Cell { x: 2, y: 3 },
    Cell { x: 3, y: 3 },
];

const PULSAR: &[Cell] = &[
    Cell { x: 2, y: 0 },
    Cell { x: 3, y: 0 },
    Cell { x: 4, y: 0 },
    Cell { x: 8, y: 0 },
    Cell { x: 9, y: 0 },
    Cell { x: 10, y: 0 },
    Cell { x: 0, y: 2 },
    Cell { x: 5, y: 2 },
    Cell { x: 7, y: 2 },
    Cell { x: 12, y: 2 },
    Cell { x: 0, y: 3 },
    Cell { x: 5, y: 3 },
    Cell { x: 7, y: 3 },
    Cell { x: 12, y: 3 },
    Cell { x: 0, y: 4 },
    Cell { x: 5, y: 4 },
    Cell { x: 7, y: 4 },
    Cell { x: 12, y: 4 },
    Cell { x: 2, y: 5 },
    Cell { x: 3, y: 5 },
    Cell { x: 4, y: 5 },
    Cell { x: 8, y: 5 },
    Cell { x: 9, y: 5 },
    Cell { x: 10, y: 5 },
    Cell { x: 2, y: 7 },
    Cell { x: 3, y: 7 },
    Cell { x: 4, y: 7 },
    Cell { x: 8, y: 7 },
    Cell { x: 9, y: 7 },
    Cell { x: 10, y: 7 },
    Cell { x: 0, y: 8 },
    Cell { x: 5, y: 8 },
    Cell { x: 7, y: 8 },
    Cell { x: 12, y: 8 },
    Cell { x: 0, y: 9 },
    Cell { x: 5, y: 9 },
    Cell { x: 7, y: 9 },
    Cell { x: 12, y: 9 },
    Cell { x: 0, y: 10 },
    Cell { x: 5, y: 10 },
    Cell { x: 7, y: 10 },
    Cell { x: 12, y: 10 },
    Cell { x: 2, y: 12 },
    Cell { x: 3, y: 12 },
    Cell { x: 4, y: 12 },
    Cell { x: 8, y: 12 },
    Cell { x: 9, y: 12 },
    Cell { x: 10, y: 12 },
];

const PENTA_DECATHLON: &[Cell] = &[
    Cell { x: 1, y: 0 },
    Cell { x: 2, y: 0 },
    Cell { x: 0, y: 1 },
    Cell { x: 3, y: 1 },
    Cell { x: 1, y: 2 },
    Cell { x: 2, y: 2 },
    Cell { x: 1, y: 3 },
    Cell { x: 2, y: 3 },
    Cell { x: 1, y: 4 },
    Cell { x: 2, y: 4 },
    Cell { x: 1, y: 5 },
    Cell { x: 2, y: 5 },
    Cell { x: 0, y: 6 },
    Cell { x: 3, y: 6 },
    Cell { x: 1, y: 7 },
    Cell { x: 2, y: 7 },
];

const GLIDER: &[Cell] = &[
    Cell { x: 1, y: 0 },
    Cell { x: 2, y: 1 },
    Cell { x: 0, y: 2 },
    Cell { x: 1, y: 2 },
    Cell { x: 2, y: 2 },
];

const LWSS: &[Cell] = &[
    Cell { x: 1, y: 0 },
    Cell { x: 4, y: 0 },
    Cell { x: 0, y: 1 },
    Cell { x: 0, y: 2 },
    Cell { x: 4, y: 2 },
    Cell { x: 0, y: 3 },
    Cell { x: 1, y: 3 },
    Cell { x: 2, y: 3 },
    Cell { x: 3, y: 3 },
];

const MWSS: &[Cell] = &[
    Cell { x: 2, y: 0 },
    Cell { x: 0, y: 1 },
    Cell { x: 4, y: 1 },
    Cell { x: 5, y: 2 },
    Cell { x: 0, y: 3 },
    Cell { x: 5, y: 3 },
    Cell { x: 1, y: 4 },
    Cell { x: 2, y: 4 },
    Cell { x: 3, y: 4 },
    Cell { x: 4, y: 4 },
    Cell { x: 5, y: 4 },
];

const HWSS: &[Cell] = &[
    Cell { x: 2, y: 0 },
    Cell { x: 3, y: 0 },
    Cell { x: 0, y: 1 },
    Cell { x: 5, y: 1 },
    Cell { x: 6, y: 2 },
    Cell { x: 0, y: 3 },
    Cell { x: 6, y: 3 },
    Cell { x: 1, y: 4 },
    Cell { x: 2, y: 4 },
    Cell { x: 3, y: 4 },
    Cell { x: 4, y: 4 },
    Cell { x: 5, y: 4 },
    Cell { x: 6, y: 4 },
];

pub fn seed_world(framebuffer: &mut Framebuffer, alive_color: u32) {
    framebuffer.set_current_color(alive_color);

    add_pattern(framebuffer, 6, 6, PULSAR);
    add_pattern(framebuffer, 31, 7, PENTA_DECATHLON);
    add_pattern(framebuffer, 45, 7, &rotate(PENTA_DECATHLON, 1));
    add_pattern(framebuffer, 65, 7, LWSS);
    add_pattern(framebuffer, 90, 8, MWSS);
    add_pattern(framebuffer, 119, 8, HWSS);

    add_pattern(framebuffer, 14, 35, GLIDER);
    add_pattern(framebuffer, 29, 39, &rotate(GLIDER, 1));
    add_pattern(framebuffer, 45, 34, &rotate(GLIDER, 2));
    add_pattern(framebuffer, 59, 39, &rotate(GLIDER, 3));

    add_pattern(framebuffer, 77, 34, TOAD);
    add_pattern(framebuffer, 91, 34, BEACON);
    add_pattern(framebuffer, 105, 34, BLINKER);
    add_pattern(framebuffer, 119, 34, BLOCK);
    add_pattern(framebuffer, 131, 34, BEE_HIVE);
    add_pattern(framebuffer, 145, 34, LOAF);

    add_pattern(framebuffer, 9, 69, BOAT);
    add_pattern(framebuffer, 22, 70, TUB);
    add_pattern(framebuffer, 38, 68, LWSS);
    add_pattern(framebuffer, 59, 66, &rotate(LWSS, 1));
    add_pattern(framebuffer, 82, 67, MWSS);
    add_pattern(framebuffer, 109, 66, HWSS);
    add_pattern(framebuffer, 141, 70, GLIDER);
}

fn add_pattern(framebuffer: &mut Framebuffer, origin_x: isize, origin_y: isize, pattern: &[Cell]) {
    for cell in pattern {
        let x = origin_x + cell.x;
        let y = origin_y + cell.y;

        if x >= 0 && y >= 0 {
            framebuffer.point(x as usize, y as usize);
        }
    }
}

fn rotate(pattern: &[Cell], turns: usize) -> Vec<Cell> {
    let mut rotated = pattern.to_vec();

    for _ in 0..turns {
        rotated = rotate90(&rotated);
    }

    rotated
}

fn rotate90(pattern: &[Cell]) -> Vec<Cell> {
    let mut rotated: Vec<Cell> = pattern
        .iter()
        .map(|cell| Cell {
            x: -cell.y,
            y: cell.x,
        })
        .collect();

    let min_x = rotated.iter().map(|cell| cell.x).min().unwrap_or(0);
    let min_y = rotated.iter().map(|cell| cell.y).min().unwrap_or(0);

    for cell in &mut rotated {
        cell.x -= min_x;
        cell.y -= min_y;
    }

    rotated
}
