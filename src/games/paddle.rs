use crate::vga_buffer::{Color, ColorCode, DoubleBuffer, ScreenChar, BUFFER_HEIGHT, BUFFER_WIDTH, WRITER};
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref PADDLE_GAME: Mutex<PaddleGame> = {
        let screen = Screen::new(BUFFER_WIDTH, BUFFER_HEIGHT);
        let game = PaddleGame::new(screen);
        Mutex::new(game)
    };
}

lazy_static! {
    static ref BUFFER: Mutex<DoubleBuffer> = {
        let color = ColorCode::new(Color::White, Color::Black);
        let char = ScreenChar {
            ascii_character: 0,
            color_code: color,
        };
        let buffer = DoubleBuffer {
            chars: [[char; BUFFER_WIDTH]; BUFFER_HEIGHT],
        };
        Mutex::new(buffer)
    };
}

fn char(char: u8, color: ColorCode) -> ScreenChar {
    ScreenChar {
        ascii_character: char,
        color_code: color,
    }
}

pub struct Screen {
    width: usize,
    height: usize,
}

impl Screen {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

struct Velocity {
    x: i8,
    y: i8,
    // speed
}

struct Ball {
    pos_x: i8,
    pos_y: i8,
    velocity: Velocity,
    color: ColorCode,
}

impl Ball {
    pub fn advance(&mut self, screen: &Screen) {
        self.pos_x += self.velocity.x;
        self.pos_y += self.velocity.y;

        // bounce off left wall
        if self.pos_x < 1 {
            self.velocity.x *= -1;
            self.pos_x += 2;
        }

        // bounce off right wall
        if self.pos_x > screen.width as i8 - 2 {
            self.velocity.x *= -1;
            self.pos_x -= 2;
        }

        // bounce off top wall
        if self.pos_y < 1 {
            self.velocity.y *= -1;
            self.pos_y += 2;
        }

        // bounce off bottom wall
        if self.pos_y > screen.height as i8 - 2 {
            self.velocity.y *= -1;
            self.pos_y -= 2;
        }
    }
}

pub struct PaddleGame {
    ticks: u64,
    screen: Screen,
    ball: Ball,
    text_color: ColorCode,
}

impl PaddleGame {
    pub fn new(screen: Screen) -> Self {
        let color = ColorCode::new(Color::Green, Color::Black);
        let ball = Ball {
            pos_x: 40,
            pos_y: 12,
            velocity: Velocity { x: 1, y: -1 },
            color,
        };

        let text_color = ColorCode::new(Color::White, Color::Black);

        PaddleGame {
            ticks: 0,
            screen,
            ball,
            text_color,
        }
    }

    pub fn clear_screen(&self) {
        let mut buffer = BUFFER.lock();

        for y in 0..self.screen.height {
            for x in 0..self.screen.width {
                buffer.chars[y][x] = char(b' ', self.text_color);
            }
        }
    }

    pub fn draw_screen_border(&self) {
        let mut buffer = BUFFER.lock();

        for y in 0..self.screen.height {
            // left edge
            buffer.chars[y][0] = char(b'|', self.text_color);
            // right edge
            buffer.chars[y][self.screen.width - 1] = char(b'|', self.text_color);
        }

        for x in 0..self.screen.width {
            // top edge
            buffer.chars[0][x] = char(b'-', self.text_color);
            // bottom edge
            buffer.chars[self.screen.height - 1][x] = char(b'-', self.text_color);
        }
    }

    pub fn draw_ball(&mut self) {
        let mut buffer = BUFFER.lock();

        let x = self.ball.pos_x as usize;
        let y = self.ball.pos_y as usize;
        buffer.chars[y][x] = char(b'o', self.ball.color);
    }

    pub fn paint_buffer(&self) {
        let mut writer = WRITER.lock();
        let buffer = BUFFER.lock();

        writer.write_double_buffer(&buffer);
    }

    pub fn redraw(&mut self) {
        self.ticks += 1;

        self.clear_screen();

        self.draw_screen_border();
        self.draw_ball();

        self.paint_buffer();

        self.ball.advance(&self.screen);
    }
}
