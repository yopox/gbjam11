use std::ops::Add;

use bevy::hierarchy::DespawnRecursiveExt;
use bevy::math::{Vec2, vec2, Vec3Swizzles};
use bevy::prelude::{Commands, Component, Entity, Query, Res, Time, Transform, Without};

use crate::entities::{MainShip, Ship};
use crate::graphics::FakeTransform;
use crate::util::{Angle, HALF_HEIGHT, HALF_WIDTH, HEIGHT};

#[derive(Clone)]
pub enum Moves {
    Linear(Vec2, Angle),
    Wavy(Vec2, Angle, f32, f32),
    Triangular(Vec2, Angle, f32, f32),
    /// x: f32, pause_duration: f32, t_x: f32, original move
    WithPause(f32, f32, f32, Box<Moves>),
    /// x: f32, final_pos: Vec2, original move
    StationaryAt(f32, Vec2, Box<Moves>),
}

impl Moves {
    pub fn starting_pos(&self) -> &Vec2 {
        match &self {
            Moves::Linear(pos, _) => pos,
            Moves::Wavy(pos, _, _, _) => pos,
            Moves::Triangular(pos, _, _, _) => pos,
            Moves::WithPause(_, _, _, moves)
            | Moves::StationaryAt(_, _, moves) =>
                moves.starting_pos(),
        }
    }

    pub fn pos(&mut self, time: f32, delta: f32, speed: f32) -> Vec2 {
        match self {
            Moves::Linear(starting, angle) => {
                compute_position(starting, time * speed, 0., angle)
            }
            Moves::Wavy(starting, angle, frequency, amplitude) => {
                compute_position(
                    starting,
                    time * speed,
                    // Note: frequency is not matching any specific time
                    (time * *frequency).cos() * *amplitude,
                    angle
                )
            }
            Moves::Triangular(starting, angle, frequency, amplitude) => {
                compute_position(
                    starting,
                    time * speed,
                    // /\/\/\/\/\/\/\/\/\/\/\...POOF
                    (((time * *frequency) % 2. - 1.).abs() * 2. - 1.) * *amplitude,
                    angle
                )
            }
            Moves::WithPause(x, pause, ref mut t_x, moves) => {
                if *t_x != 0. && time - *t_x < *pause { moves.pos(*t_x, delta, speed) }
                else {
                    let pos = moves.pos(time - if *t_x != 0. { *pause } else { 0. }, delta, speed);
                    if (pos.x - *x).abs() < 0.5 && *t_x == 0. { *t_x = time; }
                    pos
                }
            }
            Moves::StationaryAt(x, ref mut final_pos, moves) => {
                if !final_pos.is_nan() { *final_pos }
                else {
                    let pos = moves.pos(time, delta, speed);
                    if (pos.x - *x).abs() < 0.5 { *final_pos = pos; }
                    pos
                }
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
    mut query: Query<(&mut Movement, &Ship, &mut FakeTransform)>,
) {
    for (mut movement, ship, mut pos) in query.iter_mut() {
        let t_0 = movement.t_0;
        let new_pos = movement.moves.pos(time.elapsed_seconds() - t_0, time.delta_seconds(), ship.speed);
        pos.translation.x = new_pos.x;
        pos.translation.y = new_pos.y;
    }
}

pub fn despawn_far_ships(
    mut commands: Commands,
    ships: Query<(Entity, &Transform), Without<MainShip>>,
) {
    let center = vec2(HALF_WIDTH, HALF_HEIGHT);
    for (e, pos) in ships.iter() {
        if pos.translation.xy().distance(center) > HEIGHT as f32 {
            commands.entity(e).despawn_recursive();
        }
    }
}
