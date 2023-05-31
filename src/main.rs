use std::process;

mod prelude {
    pub use bracket_lib::prelude::*;
}

use prelude::*;

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Hello, Bracket Terminal");
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

    if let Err(e) = main_loop(context.unwrap(), State {}) {
        eprintln!("Error in main loop: {:#?}", e);
        process::exit(1);
    }
}
