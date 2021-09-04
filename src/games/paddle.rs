use crate::{
    serial_println,
    vga_buffer::{Color, ColorCode, DoubleBuffer, ScreenChar, BUFFER_HEIGHT, BUFFER_WIDTH, WRITER},
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PaddleMoveDirection {
    None,
    Up,
    Down,
}

struct Paddle {
    side: PaddleSide,
    length: i8,
    pos_x: i8, // the column (paddle is one char wide)
    pos_y: i8, // top of paddle
    color: ColorCode,

    // movement
    dir: PaddleMoveDirection,
    displace_units: i8,
}

impl Paddle {
    fn intersects(&self, pos: &BallPosition) -> bool {
        // not on or behind left paddle
        if self.side == PaddleSide::Left && self.pos_x < pos.x {
            return false;
        }

        // not on or behind right paddle
        if self.side == PaddleSide::Right && pos.x < self.pos_x {
            return false;
        }

        // overlaps the paddle in the y coordinate
        if pos.y >= self.pos_y && pos.y <= self.pos_y + self.length {
            return true;
        }

        return false;
    }

    fn program_move(&mut self, screen: &Screen, ball: &Ball) {
        // do nothing - we're in the middle of a programmed move
        if self.displace_units > 0 {
            return;
        }

        let mid = screen.width as i8 / 2;
        let mid_less1 = mid - 1;

        let mut toward_us = false;

        // just crossed over the midpoint of the screen
        if ball.pos_x == mid || ball.pos_x == mid_less1 {
            let mut ball_pos_x = ball.pos_x;
            let mut ball_pos_y = ball.pos_y;

            // moving toward us?
            if self.side == PaddleSide::Left && ball.velocity.x < 0 {
                toward_us = true;

                // simulate advancing ball to our y position
                while ball_pos_x > self.pos_x {
                    ball_pos_x += ball.velocity.x;
                    ball_pos_y += ball.velocity.y;
                }
            }

            // moving toward us?
            if self.side == PaddleSide::Right && ball.velocity.x > 0 {
                toward_us = true;
                // simulate advancing ball to our y position

                while ball_pos_x < self.pos_x {
                    ball_pos_x += ball.velocity.x;
                    ball_pos_y += ball.velocity.y;
                }
            }

            if toward_us == true {
                // if above the screen simulate a bounce off the top edge
                if ball_pos_y < 0 {
                    ball_pos_y *= -1;
                }

                // if below the screen simulate a bounce off the bottom edge
                if ball_pos_y > screen.height as i8 - 1 {
                    ball_pos_y -= screen.height as i8;
                }

                // it's going to impact above us
                if ball_pos_y < self.pos_y {
                    self.dir = PaddleMoveDirection::Up;
                    self.displace_units = self.pos_y - ball_pos_y;
                }

                // it's going to impact below us
                if ball_pos_y > (self.pos_y + self.length) {
                    self.dir = PaddleMoveDirection::Down;
                    self.displace_units = ball_pos_y - (self.pos_y + self.length);
                }

                serial_println!(
                    "ball_pos_y: {}, self.pos_y: {} -> units: {}, dir: {:?}",
                    ball_pos_y,
                    self.pos_y,
                    self.displace_units,
                    self.dir
                );
            }
        }
    }

    fn advance(&mut self) {
        if self.displace_units > 0 {
            self.displace_units -= 1;
            if self.dir == PaddleMoveDirection::Up {
                self.pos_y -= 1;
            } else if self.dir == PaddleMoveDirection::Down {
                self.pos_y += 1;
            }
        }
    }
}

struct Velocity {
    x: i8,
    y: i8,
    // speed
}

struct BallPosition {
    x: i8,
    y: i8,
    velocity: Velocity,
}

impl BallPosition {
    pub fn advance(&mut self, screen: &Screen) {
        self.x += self.velocity.x;
        self.y += self.velocity.y;

        // bounce off left wall
        if self.x < 1 {
            serial_println!("hit left wall at pos_y: {}", self.y);
            self.velocity.x *= -1;
            self.x += 2;
        }

        // bounce off right wall
        if self.x > screen.width as i8 - 2 {
            serial_println!("hit right wall at pos_y: {}", self.y);
            self.velocity.x *= -1;
            self.x -= 2;
        }

        // bounce off top wall
        if self.y < 1 {
            self.velocity.y *= -1;
            self.y += 2;
        }

        // bounce off bottom wall
        if self.y > screen.height as i8 - 2 {
            self.velocity.y *= -1;
            self.y -= 2;
        }
    }
}

struct Ball {
    pos_x: i8,
    pos_y: i8,
    velocity: Velocity,
    pos: BallPosition,
    color: ColorCode,
}

impl Ball {
    pub fn advance(&mut self, screen: &Screen, left_paddle: &Paddle, right_paddle: &Paddle) {
        self.pos.advance(screen);
        // self.pos_x += self.velocity.x;
        // self.pos_y += self.velocity.y;

        // // bounce off left wall
        // if self.pos_x < 1 {
        //     serial_println!("hit left wall at pos_y: {}", self.pos_y);
        //     self.velocity.x *= -1;
        //     self.pos_x += 2;
        // }

        // // bounce off right wall
        // if self.pos_x > screen.width as i8 - 2 {
        //     serial_println!("hit right wall at pos_y: {}", self.pos_y);
        //     self.velocity.x *= -1;
        //     self.pos_x -= 2;
        // }

        // // bounce off top wall
        // if self.pos_y < 1 {
        //     self.velocity.y *= -1;
        //     self.pos_y += 2;
        // }

        // // bounce off bottom wall
        // if self.pos_y > screen.height as i8 - 2 {
        //     self.velocity.y *= -1;
        //     self.pos_y -= 2;
        // }

        // bounce off left paddle
        if left_paddle.intersects(&self.pos) {
            self.velocity.x *= -1;
            self.pos.x += 2;
        }

        // bounce off right paddle
        if right_paddle.intersects(&self.pos) {
            self.velocity.x *= -1;
            self.pos.x -= 2;
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
            pos: BallPosition {
                x: 40,
                y: 12,
                velocity: Velocity { x: 2, y: -1 },
            },
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
            dir: PaddleMoveDirection::None,
            displace_units: 0,
        };
        let right_paddle = Paddle {
            side: PaddleSide::Right,
            length,
            pos_x: screen.width as i8 - 4,
            pos_y,
            color: paddle_color,
            dir: PaddleMoveDirection::None,
            displace_units: 0,
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

        let x = self.ball.pos.x as usize;
        let y = self.ball.pos.y as usize;
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

        self.ball
            .advance(&self.screen, &self.left_paddle, &self.right_paddle);

        self.left_paddle.program_move(&self.screen, &self.ball);
        self.left_paddle.advance();

        self.right_paddle.program_move(&self.screen, &self.ball);
        self.right_paddle.advance();

        self.clear_screen();

        self.draw_screen_border();
        self.draw_ball();
        for paddle in [&self.left_paddle, &self.right_paddle] {
            self.draw_paddle(paddle);
        }

        self.paint_buffer();
    }
}
