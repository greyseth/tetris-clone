use std::{f32::INFINITY, thread::spawn};

use data::block_data::{I_BLOCK, J_BLOCK, L_BLOCK, O_BLOCK, S_BLOCK, T_BLOCK, Z_BLOCK};
use macroquad::{color::{self, Color, BLACK, BLUE, GRAY, GREEN, PURPLE, RED, WHITE, YELLOW}, input::{is_key_down, is_key_pressed, KeyCode}, prelude::coroutines::wait_seconds, shapes::{draw_rectangle, draw_rectangle_lines}, text::draw_text, time::get_frame_time, window::{clear_background, next_frame, request_new_screen_size, screen_height, screen_width}};

mod models {pub mod vector2; pub mod tile; pub mod block; }
mod data {pub mod block_data;}
use models::{block::Block, tile::{self, Tile}, vector2::Vector2};
use rand::Rng;

const TILE_SIZE: f32 = 25.0;
const MOVE_INTERVAL: f64 = 1.0;
const MOVE_BOOST_INTERVAL: f64 = 0.1;

#[macroquad::main("Tetris Clone")]
async fn main() {
    let mut block: Block = Block::empty();    

    let mut placed_tiles: Vec<Tile> = Vec::new();

    let mut move_counter = 0.0;
    let mut boosting = false;

    loop {
        clear_background(BLACK);

        if is_key_down(KeyCode::Down) {boosting = true;}
        else {boosting = false;}

        if block.null == true {spawn_block(&mut block);}

        let area_width = TILE_SIZE * 10 as f32;
        let area_height = TILE_SIZE * 20 as f32;

        for y in 0..20 {
            for x in 0..10 {
                draw_rectangle_lines((screen_width() / 2.0 - area_width / 2.0)+TILE_SIZE*x as f32, 
                (screen_height() / 2.0 - area_height / 2.0)+TILE_SIZE*y as f32, 
                TILE_SIZE, TILE_SIZE, 2.0, GRAY);
            }
        }

        // Moves down over time
        move_counter += get_frame_time();
        if move_counter >= if boosting {MOVE_BOOST_INTERVAL} else {MOVE_INTERVAL} as f32 {
            move_block(&mut block, &mut placed_tiles);
            move_counter = 0.0;
        }

        // Checks for horizontal movement
        move_block_horizontal(&mut block, placed_tiles.clone());

        // Checks for instantaneous movement (hehe)
        if is_key_pressed(KeyCode::Space) {move_block_instant(&mut block, &mut placed_tiles);}

        if block.null != true {
            for tile in &block.tiles {create_tile(tile.pos.x, tile.pos.y, tile.color);}
        }
        
        for tile in &placed_tiles {
            create_tile(tile.pos.x, tile.pos.y, WHITE);
        }

        next_frame().await
    }
}

fn create_tile(pos_x: i32, pos_y: i32, col: Color) {
    let area_width = TILE_SIZE * 10.0;
    let area_height = TILE_SIZE * 20.0;
    
    for y in 0..20 {
        for x in 0..10 {
            if x == pos_x && y == pos_y {
                draw_rectangle((screen_width() / 2.0 - area_width / 2.0)+TILE_SIZE*x as f32, 
            (screen_height() / 2.0 - area_height / 2.0)+TILE_SIZE*y as f32, 
            TILE_SIZE, TILE_SIZE, col);
            draw_rectangle_lines((screen_width() / 2.0 - area_width / 2.0)+TILE_SIZE*x as f32, 
            (screen_height() / 2.0 - area_height / 2.0)+TILE_SIZE*y as f32, 
            TILE_SIZE, TILE_SIZE, 2.0,BLACK);
            }                    
        }
    }
}

fn move_block(block: &mut Block, placed_tiles: &mut Vec<Tile>) {
    let mut has_placed = false;

    for tile in &block.tiles {
        let placed_tile_check = placed_tiles.clone();

        // Check if tile is above a placed tile
        for placed_tile  in placed_tile_check {
            if tile.pos.x == placed_tile.pos.x && tile.pos.y == placed_tile.pos.y - 1 {
                has_placed = true;
                break;
            }
        }

        // Check if any tile has reached the bottom
        if tile.pos.y >= 19 {
            has_placed = true;
            break;
        }
    }

    // Move all tiles if not at bottom
    if !has_placed {
        for tile in &mut block.tiles {
            tile.pos.y += 1;
        }
    } else {
        // Adds to placed tiles
        placed_tiles.extend_from_slice(&block.tiles);
        spawn_block(block);
    }
}

fn move_block_instant(block: &mut Block, placed_tiles: &mut Vec<Tile>) {
    let mut move_distances: Vec<i32> = Vec::new();
    
    // Checks each tile for where they stop
    for tile in block.tiles.clone() {
        let mut touched_placed_tile = false;
        for placed_tile in placed_tiles.clone() {
            if tile.pos.x == placed_tile.pos.x && tile.pos.y < placed_tile.pos.y {
                move_distances.push(placed_tile.pos.y - tile.pos.y - 1);
                touched_placed_tile = true;
            }
        }

        if !touched_placed_tile {move_distances.push(19 - tile.pos.y);}
    }

    let mut smallest_move_distance= INFINITY as i32;
    for move_distance in move_distances {
        if move_distance < smallest_move_distance {smallest_move_distance = move_distance;}
    }

    for tile in &mut block.tiles {
        tile.pos.y += smallest_move_distance;        
    }

    placed_tiles.extend_from_slice(&block.tiles);
    block.null = true;
}

fn move_block_horizontal(block: &mut Block, placed_tiles: Vec<Tile>) {
    let mut hor_move = 0;
    if is_key_pressed(KeyCode::Right) {hor_move = 1;}
    else if is_key_pressed(KeyCode::Left) {hor_move = -1;}

    if hor_move == 0 {return;}
    if block.null {return;}

    let mut can_move = true;
    if hor_move == 1 {
        for tile in &mut block.tiles {
            // Checks if there's a placed tile to the right
            for placed_tile in &placed_tiles {
                if tile.pos.y == placed_tile.pos.y && tile.pos.x == placed_tile.pos.x - 1 {can_move = false;}
            }
            
            // Checks if on the right edge of the screen
            if tile.pos.x >= 9 {can_move = false;}
        }
    }else if hor_move == -1 {        
        for tile in &mut block.tiles {
            // Checks if there's a placed tile to the left
            for placed_tile in &placed_tiles {
                if tile.pos.y == placed_tile.pos.y && tile.pos.x == placed_tile.pos.x + 1 {can_move = false;}
            }

            // Checks if on the left edge of the screen
            if tile.pos.x <= 0 {can_move = false;}
        }
    }

    if can_move {
        for tile in &mut block.tiles {
            tile.pos.x += hor_move;
        }
    }
}

// Unfinished function
// fn rotate_block(block: &mut Block) {
//     let prev_tiles = block.tiles.clone();

//     // Calculates center point of block
//     let mut x_sum = 0.0;
//     let mut y_sum = 0.0;


//     let center: Vector2 = Vector2::new(0.0, 0.0);
// }

fn spawn_block(block: &mut Block) {
    let color_index = rand::thread_rng().gen_range(1..=6);
    let block_index = rand::thread_rng().gen_range(1..=7);

    let mut color = Color::new(0.0, 0.0, 0.0, 0.0);
    let mut block_type: [[bool; 4]; 4] = [
        [false, false, false, false], 
        [false, false, false, false], 
        [false, false, false, false], 
        [false, false, false, false]];

    if color_index == 1 {color = YELLOW;}
    else if color_index == 2 {color = BLUE;}
    else if color_index == 3 {color = RED;}
    else if color_index == 4 {color = GREEN;}
    else if color_index == 5 {color = PURPLE;}
    else if color_index == 6 {color = GRAY;}

    if block_index == 1 {block_type = I_BLOCK;}
    else if block_index == 2 {block_type = J_BLOCK;}
    else if block_index == 3 {block_type = L_BLOCK;}
    else if block_index == 4 {block_type = T_BLOCK;}
    else if block_index == 5 {block_type = O_BLOCK;}
    else if block_index == 6 {block_type = S_BLOCK;}
    else if block_index == 7 {block_type = Z_BLOCK}
    
    *block = Block::new(color, block_type);
}