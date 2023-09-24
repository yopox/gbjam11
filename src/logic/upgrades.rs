use bevy::prelude::{Component, Query, Transform};
use rand::{Rng, RngCore, thread_rng};

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
    PiercingShots,
    // LeechShots,
    // HomingShots,
    // StunShots,
    // SlowingShots,
    // SplitShots,
    // CurlyShots,

    SideShots,
    BetterShields,
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
            Upgrades::PiercingShots => "Piercing Shots",
            Upgrades::SideShots => "Side Shots",
            Upgrades::BetterShields => "Better Shields",
        }
    }

    pub fn description(&self, status: &ShipStatus) -> (String, String, String) {
        match self {
            Upgrades::Speed => { (
                "Improves ship speed".to_string(),
                format!("by {:.0}%.", upgrades::SPEED * 100.),
                format!("Current: x{:.2}", status.speed_multiplier()),
            ) }
            Upgrades::ShotSpeed => { (
                "Improves shot speed".to_string(),
                format!("by {:.0}%.", upgrades::SHOT_SPEED * 100.),
                format!("Current: x{:.2}", status.shot_speed_multiplier()),
            ) }
            Upgrades::ShotFrequency => { (
                "Makes your ship shoot".to_string(),
                format!("{:.0}% faster.", upgrades::SHOT_FREQUENCY * 100.),
                format!("Current: x{:.2}", status.shot_frequency_multiplier()),
            ) }
            Upgrades::Hull => { (
                "Improves hull".to_string(),
                format!("resistance by {:.0}.", upgrades::HEALTH),
                format!("Current: {:.0}", status.health().1),
            ) }
            Upgrades::Damage => { (
                "Improves shot damage".to_string(),
                format!("by {:.0}%.", upgrades::DAMAGE * 100.),
                format!("Current: x{:.2}", status.damage_multiplier()),
            ) }
            Upgrades::BouncingShots => {(
                "Make shots bounce".to_string(),
                "against the edges".to_string(),
                "of the screen.".to_string(),
            )}
            Upgrades::PiercingShots => {(
                "Make shots go".to_string(),
                "through multiple".to_string(),
                "enemies.".to_string(),
            )}
            Upgrades::BetterShields => {(
                "Make shields last".to_string(),
                "twice as long.".to_string(),
                "".to_string(),
            )}
            Upgrades::SideShots => {(
                "Equip your ship".to_string(),
                "with diagonal shots.".to_string(),
                "".to_string(),
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

    fn random_non_stat_upgrade() -> Self {
        let mut rng = thread_rng();
        let options = [Upgrades::BouncingShots, Upgrades::PiercingShots, Upgrades::SideShots, Upgrades::BetterShields];
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

    pub fn new_upgrade(status: &ShipStatus) -> Self {
        let mut rng = thread_rng();
        if rng.next_u32() % 3 == 0 {
            Upgrades::new_non_stat_upgrade(status)
        } else {
            Upgrades::random_stat_upgrade()
        }
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
        if shot.bounce_count + 1 > upgrades::MAX_BOUNCES { continue; }

        let mut bounce = false;

        if pos.translation.x >= WIDTH as f32 { shot.weapon.speed.x *= -1.; transform.scale.x *= -1.; bounce = true; }
        if pos.translation.x <= 0. { shot.weapon.speed.x *= -1.; transform.scale.x *= -1.; bounce = true; }
        if pos.translation.y >= HEIGHT as f32 { shot.weapon.speed.y *= -1.; transform.scale.y *= -1.; bounce = true; }
        if pos.translation.y <= 0. { shot.weapon.speed.y *= -1.; transform.scale.y *= -1.; bounce = true; }

        if bounce {
            shot.bounce_count += 1;
            shot.collisions.clear();
        }
    }
}