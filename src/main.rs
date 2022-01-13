use minifb::{Key, Window, WindowOptions};

struct MinifbCanvas {
    width: usize,
    height: usize,
    buffer: Vec<u32>,
    window: Window,
}

impl MinifbCanvas {
    pub fn new(width: usize, height: usize) -> Self {
        let mut window = Window::new("Canvas", width, height, WindowOptions::default()).unwrap();

        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        Self {
            width,
            height,
            buffer: vec![0; width * height],
            window,
        }
    }

    pub fn draw_box(&mut self, rect: Box) {
        let start = rect.x + self.width * rect.y;

        for h in 0..rect.height {
            let coord = start + h * self.width;

            self.buffer[coord..(coord + rect.width)].fill(0xff_ff_ff_ff);
        }
    }

    pub fn draw(&mut self) {
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            self.refresh();
        }
    }

    pub fn refresh(&mut self) {
        self.window
            .update_with_buffer(&self.buffer, self.width, self.height)
            .unwrap();
    }
}

#[derive(Debug, Clone, Copy)]
struct Box {
    width: usize,
    height: usize,
    x: usize,
    y: usize,
    color: usize,
}

impl Box {
    pub fn new(width: usize, height: usize, x: usize, y: usize, color: usize) -> Self {
        Self {
            width,
            height,
            x,
            y,
            color,
        }
    }
}

fn main() {
    let mut canvas = MinifbCanvas::new(100, 100);

    canvas.draw_box(Box::new(50, 50, 20, 10, 10));

    canvas.draw();
}
