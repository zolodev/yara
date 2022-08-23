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

    /// Check the bounds to determine if x/y coordinates pair is within the bounds
    /// of the map. If we do not check the bounds, the player could move of the
    /// edge of the map.
    pub fn in_bounds(&self, point: Point) -> bool {
        let check_x = point.x >= 0 && point.x < SCREEN_WIDTH;
        let check_y = point.y >= 0 && point.y < SCREEN_HEIGHT;
        check_x && check_y
    }

    /// Check if the tile the player wants to enter is type floor
    pub fn can_enter_tile(&self, p: Point) -> bool {
        let in_bound = self.in_bounds(p);
        let is_floor = self.tiles[map_index(p.x, p.y)] == TileType::Floor;
        in_bound && is_floor
    }

    /// Try get the map index to make sure it is in bounds
    /// Used when generating maps
    pub fn try_map_index(&self, p: Point) -> Option<usize> {
        if !self.in_bounds(p) {
            None
        } else {
            Some(map_index(p.x, p.y))
        }
    }

    #[allow(clippy::match_on_vec_items)]
    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let idx = map_index(x, y);

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
pub fn map_index(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

#[cfg(test)]
mod tests {
    use bracket_lib::prelude::Point;

    use crate::prelude::{SCREEN_HEIGHT, SCREEN_WIDTH};

    use super::map_index;

    #[test]
    fn it_can_index_the_map() {
        assert_eq!(4030, map_index(30, 50))
    }

    #[test]
    fn try_get_index_the_map() {
        let m = crate::Map::new();
        let p1 = Point::new(10, 10);
        let idx = m.try_map_index(p1);

        assert_eq!(Some(810), idx)
    }

    #[test]
    fn can_fit_in_bounds() {
        let m = crate::Map::new();

        let p1 = Point::new(10, 10);
        let p2 = Point::new(0, 0);
        let p3 = Point::new(SCREEN_WIDTH - 1, SCREEN_HEIGHT - 1);

        assert_eq!(true, m.in_bounds(p1));
        assert_eq!(true, m.in_bounds(p2));
        assert_eq!(true, m.in_bounds(p3));
    }

    #[test]
    fn can_not_fit_in_bounds() {
        let m = crate::Map::new();

        let p1 = Point::new(-10, 10);
        let p2 = Point::new(10, -10);
        let p3 = Point::new(-10, -10);
        let p4 = Point::new(100, 100);
        let p5 = Point::new(SCREEN_WIDTH, SCREEN_HEIGHT);

        assert_eq!(false, m.in_bounds(p1));
        assert_eq!(false, m.in_bounds(p2));
        assert_eq!(false, m.in_bounds(p3));
        assert_eq!(false, m.in_bounds(p4));
        assert_eq!(false, m.in_bounds(p5));
    }

    #[test]
    fn player_can_enter_tile() {
        let m = crate::Map::new();

        for x in 0..SCREEN_WIDTH - 1 {
            for y in 0..SCREEN_HEIGHT - 1 {
                let p1 = Point::new(x, y);
                assert_eq!(true, m.can_enter_tile(p1));
            }
        }
    }
}
