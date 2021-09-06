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

        let mut moving_toward_us = false;

        let mut ballpos = ball.pos.clone();

        // just crossed over the midpoint of the screen
        if ballpos.x == mid || ballpos.x == mid_less1 {
            // moving toward us?
            if self.side == PaddleSide::Left && ballpos.velocity.x < 0 {
                moving_toward_us = true;

                // simulate advancing ball to our x position
                while self.pos_x < ballpos.x {
                    ballpos.advance(screen);
                }
            }

            // moving toward us?
            if self.side == PaddleSide::Right && ballpos.velocity.x > 0 {
                moving_toward_us = true;

                // simulate advancing ball to our x position
                while ballpos.x < self.pos_x {
                    ballpos.advance(screen);
                }
            }

            if moving_toward_us == true {
                // it's going to impact above us
                if ballpos.y < self.pos_y {
                    self.dir = PaddleMoveDirection::Up;
                    self.displace_units = self.pos_y - ballpos.y;

                // it's going to impact below us
                } else if ballpos.y > (self.pos_y + self.length) {
                    self.dir = PaddleMoveDirection::Down;
                    self.displace_units = ballpos.y - (self.pos_y + self.length);

                // it's going to impact us
                } else {
                    self.dir = PaddleMoveDirection::None;
                    self.displace_units = 0;
                }

                serial_println!(
                    "ballpos_y: {}, self.pos_y: [{} - {}] -> units: {}, dir: {:?}",
                    ballpos.y,
                    self.pos_y,
                    self.pos_y + self.length,
                    self.displace_units,
                    self.dir
                );
            }
        }
    }

    fn advance(&mut self, screen: &Screen) {
        if self.displace_units > 0 {
            self.displace_units -= 1;

            // move up without hitting the edge
            if self.dir == PaddleMoveDirection::Up && self.pos_y > 1 {
                self.pos_y -= 1;

            // move down without hitting the edge
            } else if self.dir == PaddleMoveDirection::Down
                && self.pos_y + self.length < (screen.height as i8 - 1)
            {
                self.pos_y += 1;
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Velocity {
    x: i8,
    y: i8,
    // speed
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BallPosition {
    x: i8,
    y: i8,
    velocity: Velocity,
}

impl BallPosition {
    pub fn default(screen: &Screen) -> Self {
        BallPosition {
            x: screen.width as i8 / 4,
            y: screen.height as i8 / 2,
            velocity: Velocity { x: 2, y: -1 },
        }
    }

    pub fn advance(&mut self, screen: &Screen) -> Option<GameState> {
        self.x += self.velocity.x;
        self.y += self.velocity.y;

        // bounce off left wall
        if self.x < 1 {
            return Some(GameState::GameOver);
            // serial_println!("hit left wall at pos_y: {}", self.y);
            // self.velocity.x *= -1;
            // self.x += 2;
        }

        // bounce off right wall
        if self.x > screen.width as i8 - 2 {
            return Some(GameState::GameOver);
            // serial_println!("hit right wall at pos_y: {}", self.y);
            // self.velocity.x *= -1;
            // self.x -= 2;
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

        None
    }
}

struct Ball {
    pos: BallPosition,
    color: ColorCode,
}

impl Ball {
    pub fn advance(
        &mut self,
        screen: &Screen,
        left_paddle: &Paddle,
        right_paddle: &Paddle,
    ) -> Option<GameState> {
        if let Some(state) = self.pos.advance(screen) {
            return Some(state);
        }

        // bounce off left paddle
        if left_paddle.intersects(&self.pos) {
            self.pos.velocity.x *= -1;
            self.pos.x += 2;
        }

        // bounce off right paddle
        if right_paddle.intersects(&self.pos) {
            self.pos.velocity.x *= -1;
            self.pos.x -= 2;
        }

        None
    }
}

enum GameState {
    Waiting,
    Playing,
    GameOver,
}

pub struct PaddleGame {
    ticks: u64,
    state: GameState,
    screen: Screen,
    ball: Ball,
    left_paddle: Paddle,
    right_paddle: Paddle,
    text_color: ColorCode,
}

impl PaddleGame {
    pub fn new(screen: Screen) -> Self {
        let ball_color = ColorCode::new(Color::Green, Color::Black);
        let left_paddle_color = ColorCode::new(Color::Yellow, Color::Black);
        let right_paddle_color = ColorCode::new(Color::LightBlue, Color::Black);

        let ball = Ball {
            pos: BallPosition::default(&screen),
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
            color: left_paddle_color,
            dir: PaddleMoveDirection::None,
            displace_units: 0,
        };
        let right_paddle = Paddle {
            side: PaddleSide::Right,
            length,
            pos_x: screen.width as i8 - 3,
            pos_y,
            color: right_paddle_color,
            dir: PaddleMoveDirection::None,
            displace_units: 0,
        };

        let text_color = ColorCode::new(Color::White, Color::Black);

        PaddleGame {
            ticks: 0,
            state: GameState::Waiting,
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

        // for y in 0..self.screen.height {
        //     // left edge
        //     buffer.chars[y][0] = char(b'|', self.text_color);
        //     // right edge
        //     buffer.chars[y][self.screen.width - 1] = char(b'|', self.text_color);
        // }

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

    pub fn keypress(&mut self, ch: char) {
        match self.state {
            GameState::Waiting => {
                self.state = GameState::Playing;
            }
            GameState::Playing => {
                if ch == '8' {
                    self.right_paddle.dir = PaddleMoveDirection::Up;
                    self.right_paddle.displace_units = 1;
                } else if ch == '2' {
                    self.right_paddle.dir = PaddleMoveDirection::Down;
                    self.right_paddle.displace_units = 1;
                } else if ch == 'w' {
                    self.left_paddle.dir = PaddleMoveDirection::Up;
                    self.left_paddle.displace_units = 1;
                } else if ch == 's' {
                    self.left_paddle.dir = PaddleMoveDirection::Down;
                    self.left_paddle.displace_units = 1;
                }
            }
            GameState::GameOver => {
                self.state = GameState::Playing;
                self.ball.pos = BallPosition::default(&self.screen);
            }
        }
    }

    pub fn redraw_waiting(&mut self) {
        self.clear_screen();
        self.paint_buffer();

        let mut writer = WRITER.lock();
        writer.write_string_at("press any key to start game...", 0, 0, self.text_color);
    }

    pub fn redraw_playing(&mut self) {
        self.ticks += 1;

        if let Some(state) = self
            .ball
            .advance(&self.screen, &self.left_paddle, &self.right_paddle)
        {
            self.state = state;
            return;
        }

        self.left_paddle.program_move(&self.screen, &self.ball);
        self.left_paddle.advance(&self.screen);

        self.right_paddle.program_move(&self.screen, &self.ball);
        self.right_paddle.advance(&self.screen);

        self.clear_screen();

        self.draw_screen_border();
        self.draw_ball();
        for paddle in [&self.left_paddle, &self.right_paddle] {
            self.draw_paddle(paddle);
        }

        self.paint_buffer();
    }

    pub fn redraw_game_over(&mut self) {
        self.clear_screen();
        self.paint_buffer();

        let mut writer = WRITER.lock();
        writer.write_string_at(
            "game over. press any key to restart game...",
            0,
            0,
            self.text_color,
        );
    }

    pub fn redraw(&mut self) {
        match self.state {
            GameState::Waiting => {
                self.redraw_waiting();
            }
            GameState::Playing => {
                self.redraw_playing();
            }
            GameState::GameOver => {
                self.redraw_game_over();
            }
        }
    }
}
