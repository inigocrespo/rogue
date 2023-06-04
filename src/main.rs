use std::process;

mod map;
mod player;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
    pub use crate::player::*;
}

use prelude::*;

struct State {
    map: Map,
    player: Player,
}

impl State {
    fn new() -> Self {
        Self {
            map: Map::new(),
            player: Player::new(Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2)),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.player.update(ctx, &self.map);
        self.map.render(ctx);
        self.player.render(ctx);
    }
}

fn main() {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build();

    if let Err(e) = context {
        eprintln!("Error building context: {:#?}", e);
        process::exit(1);
    }

    if let Err(e) = main_loop(context.unwrap(), State::new()) {
        eprintln!("Error in main loop: {:#?}", e);
        process::exit(1);
    }
}
