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

// Includes
mod ball;
mod player;
mod config;
mod meta;

// Imports
use crate::ball::Ball;
use crate::player::Player;
use crate::config::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::meta::{Pos, Vel};

struct MainState {
    ball: Ball,
    player: Player
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        // Ball is a circle
        let circle = graphics::Mesh::new_circle(
            // Context
            ctx,
            // Mode
            graphics::DrawMode::fill(),
            // Point
            vec2(0., 0.),
            // Radius
            5.,
            // Tolerance
            2.,
            // Colour
            Color::WHITE,
        )?;
        let player = Player {
            speed: 10.,
            pos: Pos {
                x: SCREEN_WIDTH / 2.,
                y: SCREEN_HEIGHT - 20.,
            }
        };
        // Construct a ball
        let ball = Ball {
            circle,
            speed: 5.,
            pos: Pos {
                x: SCREEN_WIDTH / 2.,
                y: SCREEN_HEIGHT - 20.,
            },
            vel: Vel { dx: 1., dy: -1. },
        };
        // Player is a rectangle (wip)
        // Construct a player (wip)
        Ok(MainState { ball, player })
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Move the ball on the screen
        self.ball.render();
        self.ball.check_wall_collision();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::CanvasLoadOp::Clear([0.1, 0.2, 0.3, 1.0].into()),
        );

        // Draw the ball on the screen
        canvas.draw(
            &self.ball.circle,
            Vec2::new(self.ball.pos.x, self.ball.pos.y),
        );

        canvas.finish(ctx)?;

        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("ballbreak", "William Hleucka")
        .window_setup(ggez::conf::WindowSetup::default().title("Ballbreak"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT));
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
