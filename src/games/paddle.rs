use crate::vga_buffer::{
    Color, ColorCode, DoubleBuffer, ScreenChar, BUFFER_HEIGHT, BUFFER_WIDTH, WRITER,
};
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PaddleSide {
    Left,
    Right,
}

struct Paddle {
    side: PaddleSide,
    length: i8,
    pos_x: i8, // the column (paddle is one char wide)
    pos_y: i8, // top of paddle
    color: ColorCode,
}

impl Paddle {
    fn intersects(&self, ball: &Ball) -> bool {
        // not on or behind left paddle
        if self.side == PaddleSide::Left && self.pos_x < ball.pos_x {
            return false;
        }

        // not on or behind right paddle
        if self.side == PaddleSide::Right && ball.pos_x < self.pos_x {
            return false;
        }

        // overlaps the paddle in the y coordinate
        if ball.pos_y >= self.pos_y && ball.pos_y <= self.pos_y + self.length {
            return true;
        }

        return false;
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
    pub fn advance(&mut self, screen: &Screen, left_paddle: &Paddle, right_paddle: &Paddle) {
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

        // bounce off left paddle
        if left_paddle.intersects(self) {
            self.velocity.x *= -1;
            self.pos_x += 2;
        }

        // bounce off right paddle
        if right_paddle.intersects(self) {
            self.velocity.x *= -1;
            self.pos_x -= 2;
        }
    }
}

pub struct PaddleGame {
    ticks: u64,
    screen: Screen,
    ball: Ball,
    left_paddle: Paddle,
    right_paddle: Paddle,
    text_color: ColorCode,
}

impl PaddleGame {
    pub fn new(screen: Screen) -> Self {
        let ball_color = ColorCode::new(Color::Green, Color::Black);
        let paddle_color = ColorCode::new(Color::Yellow, Color::Black);

        let ball = Ball {
            pos_x: 40,
            pos_y: 12,
            velocity: Velocity { x: 2, y: -1 },
            color: ball_color,
        };

        let length = 8;
        let mid_y = screen.height as i8 / 2;
        let pos_y = mid_y - (length / 2);

        let left_paddle = Paddle {
            side: PaddleSide::Left,
            length,
            pos_x: 3,
            pos_y,
            color: paddle_color,
        };
        let right_paddle = Paddle {
            side: PaddleSide::Right,
            length,
            pos_x: screen.width as i8 - 4,
            pos_y,
            color: paddle_color,
        };

        let text_color = ColorCode::new(Color::White, Color::Black);

        PaddleGame {
            ticks: 0,
            screen,
            ball,
            text_color,
            left_paddle,
            right_paddle,
        }
    }

    fn clear_screen(&self) {
        let mut buffer = BUFFER.lock();

        for y in 0..self.screen.height {
            for x in 0..self.screen.width {
                buffer.chars[y][x] = char(b' ', self.text_color);
            }
        }
    }

    fn draw_screen_border(&self) {
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

    fn draw_ball(&mut self) {
        let mut buffer = BUFFER.lock();

        let x = self.ball.pos_x as usize;
        let y = self.ball.pos_y as usize;
        buffer.chars[y][x] = char(b'o', self.ball.color);
    }

    fn draw_paddle(&self, paddle: &Paddle) {
        let mut buffer = BUFFER.lock();

        let x = paddle.pos_x as usize;
        let lower = paddle.pos_y as usize;
        let upper = lower + paddle.length as usize;

        for y in lower..upper {
            buffer.chars[y][x] = char(b'|', paddle.color);
        }
    }

    fn paint_buffer(&self) {
        let mut writer = WRITER.lock();
        let buffer = BUFFER.lock();

        writer.write_double_buffer(&buffer);
    }

    pub fn redraw(&mut self) {
        self.ticks += 1;

        self.clear_screen();

        self.draw_screen_border();
        self.draw_ball();
        for paddle in [&self.left_paddle, &self.right_paddle] {
            self.draw_paddle(paddle);
        }

        self.paint_buffer();

        self.ball
            .advance(&self.screen, &self.left_paddle, &self.right_paddle);
    }
}
