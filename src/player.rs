use crate::meta::Pos;
use ggez::graphics;

pub struct Player {
    pub rectangle: graphics::Mesh,
    pub speed: f32,
    pub pos: Pos,
}
