mod map;

use bracket_lib::prelude::*;

struct State {}
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(24, "HEY, LISTEN!");
    }
}

fn main() -> BError {
    let gs = State {};
    let ctx = BTermBuilder::simple80x50()
        .with_title("Crusty Rogue")
        .build()?;

    main_loop(ctx, gs)
}
