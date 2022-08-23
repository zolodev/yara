/*****************************************************************************
 * Filename      : map.rs
 * Created       : Mon Aug 22 2022
 * Author        : Zolo
 * Github        : https://github.com/zolodev
 * Description   : This is the main file for everything related to Map
*****************************************************************************/
#![warn(clippy::all, clippy::pedantic)]

use crate::prelude::*;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    #[allow(clippy::match_on_vec_items)]
    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let idx = striding_map_index(x, y);

                match self.tiles[idx] {
                    TileType::Floor => {
                        let glyph = to_cp437('.');
                        ctx.set(x, y, YELLOW, BLACK, glyph);
                    }
                    TileType::Wall => {
                        let glyph = to_cp437('#');
                        ctx.set(x, y, GREEN, BLACK, glyph);
                    }
                }
            }
        }
    }
}

/// Striding is a way to transform map locations (x, y) into vector indices.
#[allow(clippy::cast_sign_loss)]
pub fn striding_map_index(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

#[cfg(test)]
mod tests {
    use super::striding_map_index;

    #[test]
    fn it_can_index_the_map() {
        assert_eq!(4030, striding_map_index(30, 50))
    }
}
