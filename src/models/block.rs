use macroquad::color::Color;

use super::tile::Tile;

#[derive(Clone)]
pub struct Block {
    pub tiles: Vec<Tile>,
    pub placed: bool,
    pub null: bool
}

impl Block {
    pub fn new(color: Color, tiles: [[bool; 4]; 4]) -> Self {
        let mut result_vec: Vec<Tile> = Vec::new();

        for (y, row) in tiles.iter().enumerate() {
            for (x, &is_tile) in row.iter().enumerate() {
                if is_tile {
                    // If there's a tile at this position, create a Tile with the given color
                    result_vec.push(Tile::new_col(3 + x as i32, y as i32, color));
                }
            }
        }

        Block { tiles: result_vec, placed: false, null: false }
    }

    pub fn new_preview(color: Color, tiles: [[bool; 4]; 4]) -> Self {
        let mut result_vec: Vec<Tile> = Vec::new();

        for (y, row) in tiles.iter().enumerate() {
            for (x, &is_tile) in row.iter().enumerate() {
                if is_tile {
                    // If there's a tile at this position, create a Tile with the given color
                    result_vec.push(Tile::new_col(x as i32, y as i32, color));
                }
            }
        }

        Block { tiles: result_vec, placed: false, null: false }
    }

    pub fn empty() -> Self {
        Block {tiles: Vec::new(), placed: false, null: true}
    }
}