use bevy::prelude::{Component, Query, Transform};

use crate::entities::Shot;
use crate::graphics::FakeTransform;
use crate::util::{HEIGHT, WIDTH};

pub const BOUNCING: usize = 1 << 1;
pub const PIERCING: usize = 1 << 2;
pub const LEECH: usize = 1 << 3;
pub const HOMING: usize = 1 << 4;
pub const STUN: usize = 1 << 5;
pub const SLOWING: usize = 1 << 6;
pub const SPLIT: usize = 1 << 7;
pub const CURLY: usize = 1 << 8;

#[derive(Component, Copy, Clone, Default)]
pub struct ShotUpgrades(pub usize);

pub fn bounce_shots(
    mut shots: Query<(&mut Shot, &FakeTransform, &mut Transform, &ShotUpgrades)>,
) {
    for (mut shot, pos, mut transform, upgrades) in shots.iter_mut() {
        if upgrades.0 & BOUNCING == 0 { continue; }

        if pos.translation.x >= WIDTH as f32 { shot.weapon.speed.x *= -1.; transform.scale.x *= -1.; }
        if pos.translation.x <= 0. { shot.weapon.speed.x *= -1.; transform.scale.x *= -1.; }
        if pos.translation.y >= HEIGHT as f32 { shot.weapon.speed.y *= -1.; transform.scale.y *= -1.; }
        if pos.translation.y <= 0. { shot.weapon.speed.y *= -1.; transform.scale.y *= -1.; }
    }
}