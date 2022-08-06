#![allow(clippy::unnecessary_wraps)]
/**
 * Ballbreak (Rust)
 * @author William Hleucka <william.hleucka@gmail.com>
 * @date 2022-08-06
 */

// Create a "Good game easily" lol ggez
use ggez::{
    event,
    graphics::{self, Color},
    Context, GameResult,
};
// Linear algebra lib
use glam::*;

// Position (x,y coordinates)
struct Pos {
    x: f32,
    y: f32,
}

// Velocity (x,y delta)
struct Vel {
    dx: f32,
    dy: f32,
}

struct Ball {
   circle: graphics::Mesh,
   speed: f32,
   pos: Pos,
   vel: Vel,
}

struct MainState {
    ball: Ball
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let ball_x = 800. / 2.;
        let ball_y = 600. - 40.;
        let circle = graphics::Mesh::new_circle(
            // Context
            ctx,
            // Mode
            graphics::DrawMode::fill(),
            // Point
            vec2(0.,0.),
            // Radius
            5.,
            // Tolerance
            2.,
            // Colour
            Color::WHITE,
        )?;
        let ball = Ball {
            circle,
            speed: 5.,
            pos: Pos {
                x: ball_x,
                y: ball_y,
            },
            vel: Vel {
                dx: 1.,
                dy: -1.,
            }
        };
        Ok(MainState { ball })
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Move the ball on the screen
        let x = self.ball.pos.x + self.ball.speed * self.ball.vel.dx;
        let y = self.ball.pos.y + self.ball.speed * self.ball.vel.dy;
        self.ball.pos.x = x;
        self.ball.pos.y = y;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::CanvasLoadOp::Clear([0.1, 0.2, 0.3, 1.0].into()),
        );

        // Draw the ball on the screen
        canvas.draw(&self.ball.circle, Vec2::new(self.ball.pos.x, self.ball.pos.y));

        canvas.finish(ctx)?;

        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("ballbreak", "William Hleucka");
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
