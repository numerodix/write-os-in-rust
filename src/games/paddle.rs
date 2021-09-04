use crate::vga_buffer::{Color, ColorCode, BUFFER_HEIGHT, BUFFER_WIDTH, WRITER};
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref PADDLE_GAME: Mutex<PaddleGame> = {
        let screen = Screen::new(BUFFER_WIDTH, BUFFER_HEIGHT);
        let game = PaddleGame::new(screen);
        Mutex::new(game)
    };
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

pub struct Ball {
    pos_x: usize,
    pos_y: usize,
    dir: i8,
    color: ColorCode,
}

impl Ball {
    pub fn advance(&mut self, screen: &Screen) {
        // moving to the right
        if self.dir > 0 {
            self.pos_x += 1;
            // bounce off right wall
            if self.pos_x > screen.width - 2 {
                self.dir = -1;
                self.pos_x -= 2;
            }
        } else {
            self.pos_x -= 1;
            // bounce off left wall
            if self.pos_x < 1 {
                self.dir = 1;
                self.pos_x += 2;
            }
        }
    }
}

pub struct PaddleGame {
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
            dir: 1,
            color,
        };

        let text_color = ColorCode::new(Color::White, Color::Black);

        PaddleGame {
            screen,
            ball,
            text_color,
        }
    }

    pub fn clear_screen(&self) {
        let mut writer = WRITER.lock();

        for y in 0..self.screen.height {
            for x in 0..self.screen.width {
                writer.write_byte_at(b' ', x, y, self.text_color);
            }
        }
    }

    pub fn draw_screen_border(&self) {
        let mut writer = WRITER.lock();

        for y in 0..self.screen.height {
            // left edge
            writer.write_byte_at(b'|', 0, y, self.text_color);
            // right edge
            writer.write_byte_at(b'|', self.screen.width - 1, y, self.text_color);
        }

        for x in 0..self.screen.width {
            // top edge
            writer.write_byte_at(b'-', x, 0, self.text_color);
            // bottom edge
            writer.write_byte_at(b'-', x, self.screen.height - 1, self.text_color);
        }
    }

    pub fn draw_ball(&mut self) {
        let mut writer = WRITER.lock();

        writer.write_byte_at(b'o', self.ball.pos_x, self.ball.pos_y, self.ball.color);
    }

    pub fn redraw(&mut self) {
        self.clear_screen();

        self.draw_screen_border();
        self.draw_ball();

        self.ball.advance(&self.screen);
    }
}
