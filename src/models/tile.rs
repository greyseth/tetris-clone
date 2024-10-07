use macroquad::color::Color;

use super::vector2::Vector2;

#[derive(Clone)]
pub struct Tile {
    pub color: Color,
    pub pos: Vector2
}
impl Tile {
    pub fn new(x: i32, y: i32 ) -> Self {
        Tile {color: macroquad::color::WHITE, pos: Vector2::new(x, y)}
    }
    pub fn new_col(x: i32, y:i32, color: Color) -> Self {
        Tile {color, pos: Vector2::new(x, y)}
    }
}