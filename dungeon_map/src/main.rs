/*****************************************************************************
 * Filename      : main.rs
 * Created       : Mon Aug 22 2022
 * Author        : Zolo
 * Github        : https://github.com/zolodev
 * Description   : This is the start file for YARA
*****************************************************************************/
#![warn(clippy::all, clippy::pedantic)]

mod map;
mod player;

mod prelude {

    // re-exports the bracket_lib
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
    pub use crate::player::*;
}

use prelude::*;

struct State {
    map: Map,
}

impl State {
    fn new() -> Self {
        Self { map: Map::new() }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.map.render(ctx);
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .build()?;

    main_loop(context, State::new())
}
