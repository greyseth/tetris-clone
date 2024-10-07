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

    let mut paused = false;

    loop {
        if paused {
            if is_key_pressed(KeyCode::Escape) {paused = false;}
            return next_frame().await
        }
        
        clear_background(BLACK);

        if is_key_pressed(KeyCode::Escape) {paused = false;}

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

        // Checks for horizontal and rotational movement
        move_block_horizontal(&mut block, placed_tiles.clone());
        rotate_block(&mut block);

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
        check_lines(placed_tiles);
        block.null = true;
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
    check_lines(placed_tiles);
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

fn rotate_block(block: &mut Block) {
    let prev_tiles = block.tiles.clone();

    // Calculates center point of block
    let mut x_sum = 0.0;
    let mut y_sum = 0.0;
    for tile in block.tiles.clone() {
        x_sum += tile.pos.x as f64;
        y_sum += tile.pos.y as f64;
    }

    let center: Vector2 = Vector2::new((x_sum / block.tiles.len() as f64) as i32, (y_sum / block.tiles.len() as f64) as i32);
    
    // Gets rotation input
    let mut rotate_angle = 0;
    if is_key_pressed(KeyCode::A) {rotate_angle = -90;}
    else if is_key_pressed(KeyCode::D) {rotate_angle = 90;}

    // Rotates each tile according to center point
    if rotate_angle != 0 {
        for tile in &mut block.tiles {
            if !(tile.pos.x == center.x && tile.pos.y == center.y) {
                tile.pos = rotate_point(center.x, center.y, rotate_angle, tile.pos.clone());
            }

            // Shitty hack for now ig
            tile.pos.x += 1;
        }
    }
}

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

fn rotate_point(cx: i32, cy: i32, angle: i32, p: Vector2) -> Vector2 {
    let mut rotated_vector = Vector2::new(0, 0);
    let mut local_vector = Vector2::new(p.x - cx, p.y - cy);

    local_vector = Vector2::new(-local_vector.y, local_vector.x);
    rotated_vector = Vector2::new(local_vector.x + cx, local_vector.y + cy);

    rotated_vector
}

fn check_lines(placed_tiles: &mut Vec<Tile>) {
    // Checks each tile for a completed line
    // This needs some cleaning...
    for row in 0..20 {
        let mut index_to_delete: Vec<usize> = Vec::new();
        let mut row_tiles = 0;
        for (i, tile) in placed_tiles.iter().enumerate() {
            if tile.pos.y == row {row_tiles += 1; index_to_delete.push(i);}
        }

        if row_tiles >= 10 {
            for col in 0..10 {
                placed_tiles.remove(placed_tiles.iter().position(|tile| tile.pos.y == row).unwrap());
            }

            for tile in placed_tiles.iter_mut() {
                if tile.pos.y < row {tile.pos.y += 1;}
            }
        }
    }
}