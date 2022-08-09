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
mod brick;
mod bricks;
mod config;
mod meta;
mod player;

// Imports
use crate::ball::Ball;
use crate::brick::Brick;
use crate::bricks::Bricks;
use crate::config::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::meta::{Pos, Vel};
use crate::player::Player;

struct MainState {
    ball: Ball,
    player: Player,
    bricks: Bricks,
    level: i32,
    score: i32,
    lives: i32,
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
            // unwraps Result<T, E> and Option<T> values.
        )?;
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

        // Player is a rectangle
        let rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                0.,  // x
                0.,  // y
                60., // w
                8.,  // h
            ),
            Color::GREEN,
        )?;
        // Construct a player
        let player = Player {
            rectangle,
            speed: 10.,
            width: 60.,
            height: 8.,
            pos: Pos {
                x: (SCREEN_WIDTH / 2.) - 45.,
                y: SCREEN_HEIGHT - 50.,
            },
            vel: Vel { dx: 0., dy: 0. },
        };

        // Bricks are collection of balls
        let mut balls = Vec::new();
        let start_x: f32 = ball.radius;
        let start_y: f32 = ball.radius + 15.;
        let rows: f32 = (SCREEN_WIDTH - start_x * 2.).floor();
        let cols: f32 = 10.; // TODO fix this (use min and calculate rows from level)
        for h in 0..cols as i32 {
            for i in 0..rows as i32 {
                // WTF? int & float calculations
                let x: f32 = start_x + (ball.radius as i32 * 4 * i) as f32;
                let y: f32 = start_y + (ball.radius as i32 * 4 * h) as f32;
                let circle = graphics::Mesh::new_circle(
                    ctx,
                    graphics::DrawMode::fill(),
                    vec2(0., 0.),
                    5., // r
                    2., // tolerance
                    Color::WHITE,
                )?;
                let brick = Brick {
                    circle,
                    radius: 5.,
                    pos: Pos { x, y },
                };
                balls.push(brick);
            }
        }

        let bricks = Bricks { bricks: balls };

        // Initial state
        let level: i32 = 1;
        let score: i32 = 0;
        let lives: i32 = 5;

        Ok(MainState {
            ball,
            player,
            bricks,
            level,
            lives,
            score,
        })
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.ball.render();
        self.ball.check_wall_collision();
        let idx = self.ball.check_brick_collision(&self.bricks);
        if idx.is_some() {
            self.bricks.bricks.remove(idx.unwrap());
        }
        self.player.render();
        self.ball.check_player_collision(&self.player);

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

        // Draw the bricks on the screen
        for brick in self.bricks.bricks.iter() {
            canvas.draw(&brick.circle, Vec2::new(brick.pos.x, brick.pos.y));
        }

        canvas.finish(ctx)?;

        Ok(())
    }

    fn key_up_event(&mut self, _ctx: &mut Context, _input: KeyInput) -> GameResult {
        match _input.keycode {
            Some(KeyCode::Escape) => event::request_quit(_ctx),
            Some(_) => self.player.stop(),
            None => {}
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
            Some(KeyCode::A) | Some(KeyCode::J) => self.player.left(),
            Some(KeyCode::L) | Some(KeyCode::D) => self.player.right(),
            Some(_) => {}
            None => {}
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
