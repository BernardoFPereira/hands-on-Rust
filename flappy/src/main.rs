use bracket_lib::prelude::*;

struct State {}
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Hello bracket!");
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Tutorial")
        .build()?;

    let gs = State {};

    main_loop(context, gs)
}
