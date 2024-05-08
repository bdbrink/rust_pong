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

struct PongGame {
    player1_score: i32,
    player2_score: i32,
    player1_pos: f32,
    player2_pos: f32,
    ball_pos: na::Point2<f32>,
    ball_velocity: na::Vector2<f32>,
}

impl PongGame {
    fn new() -> Self {
        Self {
            player1_score: 0,
            player2_score: 0,
            player1_pos: WINDOW_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0,
            player2_pos: WINDOW_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0,
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
        if self.ball_pos.x <= PADDLE_WIDTH && (self.ball_pos.y >= self.player1_pos && self.ball_pos.y <= self.player1_pos + PADDLE_HEIGHT) {
            self.ball_velocity.x *= -1.0;
        }

        if self.ball_pos.x >= WINDOW_WIDTH - PADDLE_WIDTH && (self.ball_pos.y >= self.player2_pos && self.ball_pos.y <= self.player2_pos + PADDLE_HEIGHT) {
            self.ball_velocity.x *= -1.0;
        }

        // Check for scoring
        if self.ball_pos.x <= 0.0 {
            self.player2_score += 1;
            self.reset_ball();
        } else if self.ball_pos.x >= WINDOW_WIDTH {
            self.player1_score += 1;
            self.reset_ball();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult {
        // Draw game elements
        graphics::clear(ctx, Color::BLACK);

        // Draw paddles
        let player1_rect = graphics::Rect::new(0.0, self.player1_pos, PADDLE_WIDTH, PADDLE_HEIGHT);
        let player2_rect = graphics::Rect::new(WINDOW_WIDTH - PADDLE_WIDTH, self.player2_pos, PADDLE_WIDTH, PADDLE_HEIGHT);
        let paddle_color = Color::WHITE;
        let paddle_mesh = graphics::Mesh::new_rectangle(ctx, DrawMode::fill(), player1_rect, paddle_color)?;
        graphics::draw(ctx, &paddle_mesh, graphics::DrawParam::default())?;
        let paddle_mesh = graphics::Mesh::new_rectangle(ctx, DrawMode::fill(), player2_rect, paddle_color)?;
        graphics::draw(ctx, &paddle_mesh, graphics::DrawParam::default())?;

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
        let score_text = format!("{} - {}", self.player1_score, self.player2_score);
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
    fn update(&
