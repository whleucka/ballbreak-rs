use crate::meta::{Pos, Vel};
use ggez::graphics;

pub struct Player {
    pub rectangle: graphics::Mesh,
    pub speed: f32,
    pub pos: Pos,
    pub vel: Vel
}

impl Player {
    pub fn render(&mut self) {
        let x = self.pos.x + self.speed * self.vel.dx;
        self.pos.x = x;
    }
}
