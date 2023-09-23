use bevy::prelude::{Component, Query, Transform};
use rand::{Rng, thread_rng};

use crate::entities::Shot;
use crate::graphics::FakeTransform;
use crate::logic::ShipStatus;
use crate::util::{HEIGHT, upgrades, WIDTH};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Upgrades {
    Speed,
    Damage,
    ShotSpeed,
    ShotFrequency,
    Hull,

    /// TODO: Update [logic::item::ShipStatus::shot_upgrades]
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
            Upgrades::Damage => "Power +",
            Upgrades::Hull => "Hull +",
            Upgrades::BouncingShots => "Bouncing Shots",
        }
    }

    pub fn description(&self, status: &ShipStatus) -> (String, String, String) {
        match self {
            Upgrades::Speed => { (
                "Increase ship speed".to_string(),
                format!("by {}%.", upgrades::SPEED * 100.),
                format!("Current: x{:.2}", status.speed_multiplier()),
            ) }
            Upgrades::ShotSpeed => { (
                "Increase shot speed".to_string(),
                format!("by {}%.", upgrades::SHOT_SPEED * 100.),
                format!("Current: x{:.2}", status.shot_speed_multiplier()),
            ) }
            Upgrades::ShotFrequency => { (
                "Increase shot frequency".to_string(),
                format!("by {}%.", upgrades::SHOT_FREQUENCY * 100.),
                format!("Current: x{:.2}", status.shot_frequency_multiplier()),
            ) }
            Upgrades::Hull => { (
                "Increase hull resistance".to_string(),
                format!("by {}.", upgrades::HEALTH),
                format!("Current: {:.0}", status.health().1),
            ) }
            Upgrades::Damage => { (
                "Increase shot damage".to_string(),
                format!("by {}%.", upgrades::DAMAGE * 100.),
                format!("Current: x{:.2}", status.damage_multiplier()),
            ) }
            Upgrades::BouncingShots => {(
                "Make shots bounce".to_string(),
                "against the edges".to_string(),
                "of the screen.".to_string(),
            )}
        }
    }
    pub fn is_stat_upgrade(&self) -> bool {
        match self {
            Upgrades::Speed
            | Upgrades::ShotSpeed
            | Upgrades::ShotFrequency
            | Upgrades::Damage
            | Upgrades::Hull => true,
            _ => false,
        }
    }

    pub fn random_stat_upgrade() -> Self {
        let mut rng = thread_rng();
        let options = [Upgrades::Speed, Upgrades::ShotSpeed, Upgrades::ShotFrequency, Upgrades::Damage, Upgrades::Hull];
        options[rng.gen_range(0..options.len())]
    }

    pub fn random_non_stat_upgrade() -> Self {
        let mut rng = thread_rng();
        let options = [Upgrades::BouncingShots];
        options[rng.gen_range(0..options.len())]
    }

    pub fn new_non_stat_upgrade(status: &ShipStatus) -> Self {
        let mut upgrade = Self::random_non_stat_upgrade();
        for i in 0..=30 {
            if status.has_upgrade(upgrade) { upgrade = Self::random_non_stat_upgrade(); }
            else if i == 30 { upgrade = Self::random_stat_upgrade(); }
            else { break }
        }
        upgrade
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