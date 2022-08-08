// Create a "Good game easily" lol ggez
use crate::config::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::meta::{Pos, Vel};
use ggez::graphics;

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
