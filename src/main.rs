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
    input::keyboard::{KeyCode, KeyInput},
    Context, GameResult,
};
// Linear algebra lib
use glam::*;

// Includes
mod ball;
mod config;
mod meta;
mod player;

// Imports
use crate::ball::Ball;
use crate::config::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::meta::{Pos, Vel};
use crate::player::Player;

struct MainState {
    ball: Ball,
    player: Player,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        // Ball is a circle
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            vec2(0., 0.),
            5., // r
            2., // tolerance
            Color::WHITE,
            // Note the )?; here <- this:
            // It is a postfix operator that unwraps Result<T, E> and Option<T> values. If applied to Result<T, E> , it unwraps the result and gives you the inner value, propagating the error to the calling function.
        )?;
        // Otherwise, I guess we would want to 'unwrap' the value

        // Construct a ball
        let ball = Ball {
            circle,
            speed: 5.,
            radius: 5.,
            pos: Pos {
                x: SCREEN_WIDTH / 2.,
                y: SCREEN_HEIGHT - 20.,
            },
            vel: Vel { dx: 1., dy: -1. },
        };
        // Player is a rectangle (wip)
        let rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                0.,  // x
                0.,  // y
                60., // w
                8., // h
            ),
            Color::GREEN,
        )?;
        // Construct a player (wip)
        let player = Player {
            rectangle,
            speed: 10.,
            width: 60.,
            height: 8.,
            pos: Pos {
                x: (SCREEN_WIDTH / 2.) - 45.,
                y: SCREEN_HEIGHT - 50.,
            },
            vel: Vel { dx: 0., dy: 0. }
        };
        Ok(MainState { ball, player })
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Move the ball on the screen
        self.ball.render();
        self.ball.check_wall_collision();
        self.player.render();

        let mut ball_x: f32;
        let mut ball_y: f32;
        if self.ball.is_east() {
            ball_x = self.ball.pos.x + self.ball.radius + self.ball.speed;
        } else {
            ball_x = self.ball.pos.x - self.ball.radius + self.ball.speed;
        }
        if self.ball.is_south() {
            ball_y = self.ball.pos.y + self.ball.radius + self.ball.speed;
        } else {
            ball_y = self.ball.pos.y - self.ball.radius + self.ball.speed;
        }

        // Check if player hits ball
        if self.ball.is_south() {
            if self.ball.pos.y >= self.player.pos.y &&
                self.ball.pos.y <= self.player.pos.y + self.player.height {
                    if self.ball.pos.x >= self.player.pos.x && self.ball.pos.x <= self.player.pos.x + self.player.width {
                        self.ball.change_direction(0., -1.);
                    }
                }
        }
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
        // Draw the player on the screen
        canvas.draw(
            &self.player.rectangle,
            Vec2::new(self.player.pos.x, self.player.pos.y),
        );

        canvas.finish(ctx)?;

        Ok(())
    }

    fn key_up_event(&mut self, _ctx: &mut Context, _input: KeyInput) -> GameResult {
        match _input.keycode {
            Some(KeyCode::Escape) => event::request_quit(_ctx),
            Some(_) => self.player.vel.dx = 0.,
            None => {},
        }
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        _input: KeyInput,
        _repeat: bool,
    ) -> GameResult {
        match _input.keycode {
            Some(KeyCode::A) | Some(KeyCode::J) => self.player.vel.dx = -1.,
            Some(KeyCode::L) | Some(KeyCode::D) => self.player.vel.dx = 1.,
            Some(_) => {},
            None => {},
        }
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
