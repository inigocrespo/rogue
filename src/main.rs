use std::process;

mod map;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
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
