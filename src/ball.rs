use crate::bricks::Bricks;
use crate::config::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::meta::{Pos, Vel};
use crate::player::Player;
use ggez::graphics;
use ang::*;

pub struct Ball {
    pub circle: graphics::Mesh,
    pub speed: f32,
    pub radius: f32,
    pub pos: Pos,
    pub vel: Vel,
}

impl Ball {
    pub fn render(&mut self) {
        let x = self.pos.x + self.speed * self.vel.dx;
        let y = self.pos.y + self.speed * self.vel.dy;
        self.pos.x = x;
        self.pos.y = y;
    }
    pub fn check_wall_collision(&mut self) {
        if self.pos.x >= SCREEN_WIDTH {
            self.vel.dx = -1.;
        } else if self.pos.x <= 0. {
            self.vel.dx = 1.;
        }
        if self.pos.y >= SCREEN_HEIGHT {
            self.vel.dy = -1.;
        } else if self.pos.y <= 0. {
            self.vel.dy = 1.;
        }
    }
    pub fn check_player_collision(&mut self, _player: &Player) {
        // Check if player hits ball
        if self.pos.y >= _player.pos.y && self.pos.y <= _player.pos.y + _player.height {
            if self.pos.x >= _player.pos.x && self.pos.x <= _player.pos.x + _player.width {
                self.change_direction(0., -1.);
            }
        }
    }
    pub fn calc_distance(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
       ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
    }
    pub fn calc_angle(&mut self, y2: f32, y1: f32, x2: f32, x1: f32) -> Angle<f32> {
        atan2(y2 - y1, x2 - x1) * 180. / std::f32::consts::PI
    }
    pub fn check_brick_collision(&mut self, _bricks: &Bricks) -> Result<usize,&str> {
        for (i, brick) in _bricks.bricks.iter().enumerate() {
            let brick_x: f32 = brick.pos.x + brick.radius / 2.;
            let brick_y: f32 = brick.pos.y + brick.radius / 2.;
            let ball_x: f32 = self.pos.x;
            let ball_y: f32 = self.pos.y;
            let distance: f32 = self.calc_distance(brick_x, brick_y, ball_x, ball_y);
            let angle = self.calc_angle(ball_y, brick_y, ball_x, brick_x);

            if distance <= self.radius * 2. {
                let ball_dx: f32 = (angle * std::f32::consts::PI / 180.).cos();
                let ball_dy: f32 = (angle * std::f32::consts::PI / 180.).sin();
                self.change_direction(ball_dx, ball_dy);
                return Ok(i);
            }
        }
        Err("No index here")
    }
    pub fn change_direction(&mut self, dx: f32, dy: f32) {
        if dx != 0. {
            self.vel.dx = dx;
        }
        if dy != 0. {
            self.vel.dy = dy;
        }
    }
    pub fn is_south(&mut self) -> bool {
        self.vel.dy == 1.
    }
    pub fn is_east(&mut self) -> bool {
        self.vel.dx == 1.
    }
}
