use bevy::math::{Vec2, vec2};
use bevy::prelude::{Component, Query, Res, Time};

use crate::entities::Ship;
use crate::graphics::FakeTransform;
use crate::util::Angle;

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
                compute_position(starting, time * speed, 0., angle)
            }
                )
            },
        }
    }
}

/// Compute positions for moves that follow a line (start, angle) with variation on normal position
/// (eg. movement on x -> variation on y based on time)
#[inline]
fn compute_position(start: &Vec2, linear_diff: f32, normal_diff: f32, angle: &Angle) -> Vec2 {
    return start.add(angle.rotate_vec(vec2(linear_diff, normal_diff)))
}

#[derive(Component)]
pub struct Movement {
    pub(crate) moves: Moves,
    pub(crate) t_0: f32,
}

pub fn apply_movement(
    time: Res<Time>,
    mut query: Query<(&Movement, &Ship, &mut FakeTransform)>,
) {
    for (movement, ship, mut pos) in query.iter_mut() {
        let new_pos = movement.moves.pos(time.elapsed_seconds() - movement.t_0, ship.speed);
        pos.translation.x = new_pos.x;
        pos.translation.y = new_pos.y;
    }
}
