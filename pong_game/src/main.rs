use ggez::{event, ContextBuilder, GameResult};
use ggez::graphics::{self, DrawMode, Color};
use ggez::nalgebra as na;

struct PongGame {
}

impl PongGame {
    fn new() -> Self {
        Self {
        }
    }

    fn update(&mut self, _ctx: &mut ggez::Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult {
        graphics::clear(ctx, Color::BLACK);
        graphics::present(ctx)?;
        Ok(())
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
        .build()?;
    let mut game = PongGame::new();
    event::run(&mut ctx, &mut event_loop, &mut game)
}
