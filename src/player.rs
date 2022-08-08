use crate::{meta::{Pos, Vel}, config::SCREEN_WIDTH};
use ggez::graphics;

pub struct Player {
    pub rectangle: graphics::Mesh,
    pub speed: f32,
    pub width: f32,
    pub height: f32,
    pub pos: Pos,
    pub vel: Vel
}

impl Player {
    pub fn render(&mut self) {
        if self.vel.dx != 0. {
            let x = self.pos.x + self.speed * self.vel.dx;
            self.pos.x = x;

            if self.pos.x <= 0. {
                self.pos.x = 0.;
            } else if self.pos.x + self.width >= SCREEN_WIDTH {
                self.pos.x = SCREEN_WIDTH - self.width;
            }
        }
    }
    pub fn is_moving(&mut self) -> bool {
        self.vel.dx != 0.
    }
}
