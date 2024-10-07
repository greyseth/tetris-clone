#[derive(Clone)]
pub struct Vector2 {
    pub x: i32, pub y: i32
}
impl Vector2 {
    pub fn new(x: i32, y: i32) -> Self {
        Vector2 {x, y}
    }
}
