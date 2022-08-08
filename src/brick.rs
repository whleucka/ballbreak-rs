use crate::meta::Pos;
use ggez::graphics;

pub struct Brick {
    pub circle: graphics::Mesh,
    pub radius: f32,
    pub pos: Pos,
}
