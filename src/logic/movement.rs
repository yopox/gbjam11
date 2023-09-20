use bevy::math::{Vec2, vec2};
use bevy::prelude::{Component, Query, Res, Time};

use crate::entities::{Angle, Ship};
use crate::graphics::FakeTransform;
use crate::util::to_rad;

#[derive(Copy, Clone)]
pub enum Moves {
    Linear(Vec2, Angle),
}

impl Moves {
    pub fn starting_pos(&self) -> &Vec2 {
        match &self {
            Moves::Linear(pos, _) => pos,
        }
    }

    pub fn pos(&self, time: f32, speed: f32) -> Vec2 {
        match self {
            Moves::Linear(starting, angle) => {
                vec2(
                    starting.x + time * speed * to_rad(angle.0).cos(),
                    starting.y + time * speed * to_rad(angle.0).sin(),
                )
            },
        }
    }
}

#[derive(Component)]
pub struct Movement {
    pub(crate) moves: Moves,
    pub(crate) t_0: f32,
}

pub fn apply_movement(
    time: Res<Time>,
    mut query: Query<(&Movement, &Ship, &mut FakeTransform)>
) {
    for (movement, ship, mut pos) in query.iter_mut() {
        let new_pos = movement.moves.pos(time.elapsed_seconds() - movement.t_0, ship.speed);
        pos.translation.x = new_pos.x;
        pos.translation.y = new_pos.y;
    }
}
