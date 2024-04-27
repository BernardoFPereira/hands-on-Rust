use crate::prelude::*;

pub struct Player {
    position: Point,
}
impl Player {
    pub fn new(position: Point) -> Self {
        Self { position }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        ctx.set(
            self.position.x,
            self.position.y,
            CORNFLOWERBLUE,
            BLACK,
            to_cp437('@'),
        );
    }

    pub fn update(&mut self, ctx: &mut BTerm, map: &Map) {
        if let Some(key) = ctx.key {
            let delta = match key {
                VirtualKeyCode::Numpad4 => Point::new(-1, 0),
                VirtualKeyCode::Numpad6 => Point::new(1, 0),
                VirtualKeyCode::Numpad8 => Point::new(0, -1),
                VirtualKeyCode::Numpad2 => Point::new(0, 1),
                VirtualKeyCode::Numpad7 => Point::new(-1, -1),
                VirtualKeyCode::Numpad9 => Point::new(1, -1),
                VirtualKeyCode::Numpad1 => Point::new(-1, 1),
                VirtualKeyCode::Numpad3 => Point::new(1, 1),
                _ => Point::zero(),
            };

            let new_position = self.position + delta;
            if map.can_enter_tile(new_position) {
                self.position = new_position;
            }
        }
    }
}
