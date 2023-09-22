use std::ops::Add;

use bevy::hierarchy::DespawnRecursiveExt;
use bevy::math::{Vec2, vec2, Vec3Swizzles};
use bevy::prelude::{Commands, Component, Entity, Query, Res, Time, Transform, Without};

use crate::entities::{MainShip, Ship};
use crate::graphics::FakeTransform;
use crate::util::{Angle, HEIGHT, WIDTH};

#[derive(Copy, Clone)]
pub enum Moves {
    Linear(Vec2, Angle),
    Wavy(Vec2, Angle, f32, f32),
    Triangular(Vec2, Angle, f32, f32),
}

impl Moves {
    pub fn starting_pos(&self) -> &Vec2 {
        match &self {
            Moves::Linear(pos, _) => pos,
            Moves::Wavy(pos, _, _, _) => pos,
            Moves::Triangular(pos, _, _, _) => pos
        }
    }

    pub fn pos(&self, time: f32, speed: f32) -> Vec2 {
        match self {
            Moves::Linear(starting, angle) => {
                compute_position(starting, time * speed, 0., angle)
            }
            Moves::Wavy(starting, angle, frequency, amplitude) => {
                compute_position(
                    starting,
                    time * speed,
                    // Note: frequency is not matching any specific time
                    (time * frequency).cos() * amplitude,
                    angle
                )
            }
            Moves::Triangular(starting, angle, frequency, amplitude) => {
                compute_position(
                    starting,
                    time * speed,
                    // /\/\/\/\/\/\/\/\/\/\/\...POOF
                    (((time * frequency) % 2. - 1.).abs() * 2. - 1.) * amplitude,
                    angle
                )
            }
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

pub fn despawn_far_ships(
    mut commands: Commands,
    ships: Query<(Entity, &Transform), Without<MainShip>>,
) {
    let center = vec2(WIDTH as f32 / 2., HEIGHT as f32 / 2.);
    for (e, pos) in ships.iter() {
        if pos.translation.xy().distance(center) > HEIGHT as f32 {
            commands.entity(e).despawn_recursive();
        }
    }
}
