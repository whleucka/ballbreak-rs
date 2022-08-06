// Create a "Good game easily" lol ggez
use ggez::graphics;
use crate::meta::{Pos,Vel};

pub struct Ball {
   pub circle: graphics::Mesh,
   pub speed: f32,
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
    pub fn check_all_collision(&mut self) {
        if self.pos.x >= 800. {
            self.vel.dx = -1.;
        } else if self.pos.x <= 0. {
            self.vel.dx = 1.;
        }
        if self.pos.y >= 600. {
            self.vel.dy = -1.;
        } else if self.pos.y <= 0. {
            self.vel.dy = 1.;
        }
    }
}