use ggez::{event, ContextBuilder, GameResult};
use ggez::graphics::{self, DrawMode, Color, Text, TextFragment};
use ggez::nalgebra as na;

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;
const PADDLE_WIDTH: f32 = 20.0;
const PADDLE_HEIGHT: f32 = 100.0;
const BALL_RADIUS: f32 = 10.0;
const PADDLE_SPEED: f32 = 5.0;
const BALL_SPEED: f32 = 2.0;

struct Paddle {
    pos: na::Point2<f32>,
    score: i32,
}

impl Paddle {
    fn new(x: f32, y: f32) -> Self {
        Self {
            pos: na::Point2::new(x, y),
            score: 0,
        }
    }

    fn update(&mut self, _ctx: &mut ggez::Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult {
        let rect = graphics::Rect::new(self.pos.x, self.pos.y, PADDLE_WIDTH, PADDLE_HEIGHT);
        let color = Color::WHITE;
        let mesh = graphics::Mesh::new_rectangle(ctx, DrawMode::fill(), rect, color)?;
        graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
        Ok(())
    }
}

struct PongGame {
    player1: Paddle,
    player2: Paddle,
    ball_pos: na::Point2<f32>,
    ball_velocity: na::Vector2<f32>,
}

impl PongGame {
    fn new() -> Self {
        Self {
            player1: Paddle::new(0.0, WINDOW_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0),
            player2: Paddle::new(WINDOW_WIDTH - PADDLE_WIDTH, WINDOW_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0),
            ball_pos: na::Point2::new(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0),
            ball_velocity: na::Vector2::new(BALL_SPEED, BALL_SPEED),
        }
    }

    fn update(&mut self, _ctx: &mut ggez::Context) -> GameResult {
        // Update ball position
        self.ball_pos += self.ball_velocity;

        // Check for collisions with top and bottom walls
        if self.ball_pos.y <= BALL_RADIUS || self.ball_pos.y >= WINDOW_HEIGHT - BALL_RADIUS {
            self.ball_velocity.y *= -1.0;
        }

        // Check for collisions with paddles
        if self.ball_pos.x <= PADDLE_WIDTH && (self.ball_pos.y >= self.player1.pos.y && self.ball_pos.y <= self.player1.pos.y + PADDLE_HEIGHT) {
            self.ball_velocity.x *= -1.0;
        }

        if self.ball_pos.x >= WINDOW_WIDTH - PADDLE_WIDTH && (self.ball_pos.y >= self.player2.pos.y && self.ball_pos.y <= self.player2.pos.y + PADDLE_HEIGHT) {
            self.ball_velocity.x *= -1.0;
        }

        // Check for scoring
        if self.ball_pos.x <= 0.0 {
            self.player2.score += 1;
            self.reset_ball();
        } else if self.ball_pos.x >= WINDOW_WIDTH {
            self.player1.score += 1;
            self.reset_ball();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult {
        // Draw game elements
        graphics::clear(ctx, Color::BLACK);

        // Draw paddles
        self.player1.draw(ctx)?;
        self.player2.draw(ctx)?;

        // Draw ball
        let ball_mesh = graphics::Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            self.ball_pos,
            BALL_RADIUS,
            2.0,
            Color::WHITE,
        )?;
        graphics::draw(ctx, &ball_mesh, graphics::DrawParam::default())?;

        // Draw scores
        let score_text = format!("{} - {}", self.player1.score, self.player2.score);
        let text = Text::new(TextFragment::new(score_text).color(Color::WHITE));
        graphics::draw(ctx, &text, (na::Point2::new(10.0, 10.0),))?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn reset_ball(&mut self) {
        self.ball_pos = na::Point2::new(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0);
        // Reset ball velocity here if needed
    }
}

impl event::EventHandler<ggez::GameError> for PongGame {
    fn update(&mut self, ctx: &mut ggez::Context) -> GameResult {
        self.update(ctx)
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult {
        self.draw(ctx)
    }
}

fn main() -> GameResult {
    let (mut ctx, mut event_loop) = ContextBuilder::new("pong_game", "Your Name")
        .window_setup(ggez::conf::WindowSetup::default().title("Pong Game"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
        .build()?;
    let mut game = PongGame::new();
    event::run(&mut ctx, &mut event_loop, &mut game)
}
