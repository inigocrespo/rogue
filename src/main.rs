use std::process;

mod map;
mod map_builder;
mod player;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::player::*;
}

use prelude::*;

struct State {
    map: Map,
    player: Player,
}

impl State {
    fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        Self {
            map: map_builder.map,
            player: Player::new(map_builder.player_start),
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
