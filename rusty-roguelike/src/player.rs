use crate::prelude::*;

pub struct Player {
    pub hp: i32,
    pub position: Point,
}

impl Player {
    pub fn new(point: Point) -> Self {
        return Self {
            hp: 100,
            position: point,
        };
    }

    pub fn update(&mut self, ctx: &mut BTerm, map: &Map) {
        if let Some(key) = ctx.key {
            let delta = match key {
                VirtualKeyCode::Up => Point::new(0, -1),
                VirtualKeyCode::Right => Point::new(1, 0),
                VirtualKeyCode::Down => Point::new(0, 1),
                VirtualKeyCode::Left => Point::new(-1, 0),
                _ => Point::zero(),
            };

            let new_pos = self.position + delta;
            if map.can_enter_tile(new_pos) {
                self.position = new_pos;
            }
        }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        return ctx.set(
            self.position.x,
            self.position.y,
            WHITE,
            BLACK,
            to_cp437('@')
        );
    }
}