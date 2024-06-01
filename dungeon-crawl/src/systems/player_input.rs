use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Numpad8 => Point::new(0, -1),
            VirtualKeyCode::Numpad2 => Point::new(0, 1),
            VirtualKeyCode::Numpad6 => Point::new(1, 0),
            VirtualKeyCode::Numpad4 => Point::new(-1, 0),
            VirtualKeyCode::Numpad7 => Point::new(-1, -1),
            VirtualKeyCode::Numpad9 => Point::new(1, -1),
            VirtualKeyCode::Numpad1 => Point::new(-1, 1),
            VirtualKeyCode::Numpad3 => Point::new(1, 1),
            VirtualKeyCode::Numpad5 => {
                // *turn_state = TurnState::PlayerTurn;
                Point::new(0, 0)
            }
            _ => Point::new(0, 0),
        };
        if delta.x != 0 || delta.y != 0 {
            let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
            players.iter_mut(ecs).for_each(|(entity, pos)| {
                let destination = *pos + delta;

                commands.push((
                    (),
                    WantsToMove {
                        entity: *entity,
                        destination,
                    },
                ));
            });
        }
        *turn_state = TurnState::PlayerTurn;
    }
}
