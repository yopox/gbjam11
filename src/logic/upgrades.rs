use bevy::prelude::{Component, Query, Transform};
use rand::{Rng, thread_rng};

use crate::entities::Shot;
use crate::graphics::FakeTransform;
use crate::util::{HEIGHT, WIDTH};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Upgrades {
    Speed,
    ShotSpeed,
    ShotFrequency,
    ShotDamage,

    BouncingShots,
    // PiercingShots,
    // LeechShots,
    // HomingShots,
    // StunShots,
    // SlowingShots,
    // SplitShots,
    // CurlyShots,
}

impl Upgrades {
    pub fn name(&self) -> &str {
        match self {
            Upgrades::Speed => "Speed Module +",
            Upgrades::ShotSpeed => "Shot Speed +",
            Upgrades::ShotFrequency => "Shot Frequency +",
            Upgrades::ShotDamage => "Power +",
            Upgrades::BouncingShots => "Bouncing Shots",
        }
    }
    pub fn is_stat_upgrade(&self) -> bool {
        match self {
            Upgrades::Speed
            | Upgrades::ShotSpeed
            | Upgrades::ShotFrequency
            | Upgrades::ShotDamage => true,
            _ => false,
        }
    }

    pub fn random_stat_upgrade() -> Self {
        let mut rng = thread_rng();
        let options = [Upgrades::Speed, Upgrades::ShotSpeed, Upgrades::ShotFrequency, Upgrades::ShotDamage];
        options[rng.gen_range(0..options.len())]
    }

    pub fn random_non_stat_upgrade() -> Self {
        let mut rng = thread_rng();
        let options = [Upgrades::BouncingShots];
        options[rng.gen_range(0..options.len())]
    }
}

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