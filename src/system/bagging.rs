use macroquad::{color::{Color, BLUE, GRAY, GREEN, PURPLE, RED, YELLOW}, rand::ChooseRandom};
use rand::Rng;

use crate::{data::block_data::{I_BLOCK, J_BLOCK, L_BLOCK, O_BLOCK, S_BLOCK, T_BLOCK, Z_BLOCK}, models::block::Block};

pub struct Bag {
    pub blocks: Vec<Block>,
    pub cur_index: i32,
}

impl Bag {
    pub fn create_bag() -> Self {
        let mut index_arr: Vec<i32> = (1..=7).collect();
    
        let mut blocks: Vec<Block> = Vec::new();
        for _r in 0..7 {
            let color_index = rand::thread_rng().gen_range(1..=6);
            let r_index = rand::thread_rng().gen_range(0..index_arr.len());
            
            let mut block_type: [[bool; 4]; 4] = [
                [false, false, false, false], 
                [false, false, false, false], 
                [false, false, false, false], 
                [false, false, false, false]];

            let block_index = index_arr[r_index];
            if block_index == 1 {block_type = I_BLOCK;}
            else if block_index == 2 {block_type = J_BLOCK;}
            else if block_index == 3 {block_type = L_BLOCK;}
            else if block_index == 4 {block_type = T_BLOCK;}
            else if block_index == 5 {block_type = O_BLOCK;}
            else if block_index == 6 {block_type = S_BLOCK;}
            else if block_index == 7 {block_type = Z_BLOCK}
            
            let mut color = Color::new(0.0, 0.0, 0.0, 0.0);
            if color_index == 1 {color = YELLOW;}
            else if color_index == 2 {color = BLUE;}
            else if color_index == 3 {color = RED;}
            else if color_index == 4 {color = GREEN;}
            else if color_index == 5 {color = PURPLE;}
            else if color_index == 6 {color = GRAY;}
            
            blocks.push(Block::new(color, block_type));

            index_arr.remove(r_index);
        }

        Bag {blocks, cur_index: 0}
    }

    pub fn bag_next(&mut self) -> Block {
        self.cur_index += 1;
        return self.blocks[(self.cur_index - 1) as usize].clone();
    }

    pub fn bag_next_preview(&mut self) -> Block {
        return self.blocks[(self.cur_index + 1) as usize].clone();
    }
}