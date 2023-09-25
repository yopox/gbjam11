use std::f32::consts::PI;

use bevy::math::{Vec2, vec2};
use bevy::prelude::{Res, State, States};

use crate::entities::Shots;
use crate::logic::{Items, ShipStatus};

pub const WIDTH: usize = 160;
pub const HALF_WIDTH: f32 = WIDTH as f32 / 2.;
pub const HEIGHT: usize = 144;
pub const HALF_HEIGHT: f32 = HEIGHT as f32 / 2.;

pub const SCALE: f32 = 4.;

/// Min distance between the player and the screen border
pub const BORDER: f32 = 2.;

pub mod space {
    use crate::entities::Ships;
    use crate::util::{base_stats, HALF_HEIGHT, WIDTH};

    pub const BLINK_INTERVAL: f32 = 10. / 60.;
    pub const BLINK_DURATION: f32 = BLINK_INTERVAL * 8.;
    pub const BLINK_DURATION_ELITE: f32 = BLINK_INTERVAL * 6.;
    pub const BLINK_DURATION_ENEMY: f32 = BLINK_INTERVAL * 4.;

    pub const TIME_RATIO_DEAD: f32 = 0.6;

    pub const SHIELD_DURATION: f32 = 6.;
    pub const MISSILE_RANGE: usize = WIDTH / 3;
    pub const MISSILE_SPEED: f32 = base_stats::SPEED / 3.;

    pub const NEXT_LEVEL_SPEED_Y: f32 = -18.;
    pub const NEXT_LEVEL_CHOICE_Y: f32 = HALF_HEIGHT;
    pub const RUSH_SPEED_Y: f32 = base_stats::SPEED * 12.;

    pub fn time_ratio(level: usize) -> f32 { 1. + level as f32 / 26. * 0.3 }

    pub fn patterns_nb(level: usize) -> usize {
        match level {
            0..=8 => 2,
            9..=17 => 3,
            _ => 4,
        }
    }

    pub fn credits(model: Ships) -> i16 {
        match model {
            Ships::Player(_) => 0,
            Ships::Invader(n) if n <= 3 => 2,
            Ships::Invader(n) if n <= 6 => 5,
            Ships::Invader(_) => 10,
            Ships::Elite(_) => 25,
            Ships::Boss(_) => 50,
        }
    }
}

pub mod star_field {
    use bevy::math::vec2;
    use bevy::prelude::Vec2;

    pub const INITIAL_SPEED: Vec2 = vec2(0., -30.);
    pub const MAX_INTENSITY_FACTOR: f32 = 6.;
    pub const HANGAR_SPEED: Vec2 = vec2(0., INITIAL_SPEED.y / 3.);
    pub const RUSH_SPEED: Vec2 = vec2(0., INITIAL_SPEED.y * 10.);
    pub const STARS_COUNT: usize = 50;
}

pub mod hud {
    pub const HEALTH_BAR_SIZE: usize = 24;
}

pub mod z_pos {
    pub const STAR_FIELD: f32 = 10.;
    pub const SHIPS: f32 = 30.;
    pub const SHOTS: f32 = 31.;
    pub const PAUSE: f32 = 39.;
    pub const GUI: f32 = 40.;
    pub const HANGAR: f32 = 50.;
    pub const HANGAR_TEXT: f32 = 51.;
    pub const SHOP: f32 = 50.;
    pub const SHOP_TEXT: f32 = 51.;
}

pub mod base_stats {
    pub const HEALTH: f32 = 8.;
    pub const SPEED: f32 = 25.;
    pub const DAMAGE_FACTOR: f32 = 1.0;
    pub const SHOT_SPEED: f32 = 100.;
    pub const SHOT_FREQUENCY: f32 = 1.0;
}

pub mod upgrades {
    pub const SPEED: f32 = 0.2;
    pub const DAMAGE: f32 = 0.25;
    pub const SHOT_SPEED: f32 = 0.2;
    pub const SHOT_FREQUENCY: f32 = 0.15;
    pub const HEALTH: f32 = 4.;

    pub const MAX_BOUNCES: u8 = 3;
    pub const LEECH_COUNT: usize = 8;
    pub const STUN_CHANCE: f32 = 0.05;
    pub const STUN_DURATION: f32 = 5.0;
    pub const BERSERK: f32 = 0.25;
}

impl Shots {
    pub fn attack(&self) -> f32 {
        match self {
            Shots::Bullet => 1.0,
            Shots::Wave => 0.9,
            Shots::Energy => 6.0,
            Shots::DualBeam => 1.25,
            Shots::Missile => 10.0,
        }
    }

    pub fn delay(&self) -> f32 {
        match self {
            Shots::Bullet => 1.0,
            Shots::Wave => 1.25,
            Shots::Missile => 1.0,
            Shots::Energy => 1.75,
            Shots::DualBeam => 0.9,
        }
    }
}

pub mod items {
    use bevy::utils::HashMap;
    use lazy_static::lazy_static;

    use crate::logic::Items;
    use crate::logic::route::GameMode;

    pub const STARTING_CREDITS: i16 = 0;

    lazy_static! {
        pub static ref STARTING_ITEMS: HashMap<GameMode, Vec<(Items, usize)>> = HashMap::from([
            (GameMode::Standard, vec![(Items::Missile, 2), (Items::Shield, 2),]),
            (GameMode::Act2, vec![(Items::Missile, 4), (Items::Shield, 4),]),
            (GameMode::Act3, vec![(Items::Missile, 4), (Items::Shield, 4),]),
            (GameMode::LastBoss, vec![(Items::Missile, 1), (Items::Shield, 2),]),
            (GameMode::BossRush, vec![(Items::Missile, 3), (Items::Shield, 3),]),
        ]);
    }
}

pub mod shop {
    use crate::logic::Items;

    pub fn item_price(item: &Items, sale: bool) -> i16 {
        let p = match item {
            Items::Missile | Items::Shield => 12,
            Items::Repair => 6,
            Items::Upgrade(u) if u.is_stat_upgrade() => 50,
            Items::Upgrade(_) => 100,
        };
        if sale { p / 2 } else { p }
    }
}

/// Angle in degrees
#[derive(Copy, Clone)]
pub struct Angle(pub f32);
impl Angle {
    pub fn to_rad(&self) -> f32 { self.0 * PI / 180. }
    pub fn rotate_vec(&self, vector: Vec2) -> Vec2 {
        let rad = self.to_rad();
        vector.rotate(vec2(rad.cos(), rad.sin()))
    }

    /// Returns rotation of vec2(value, 0) by the angle
    pub fn rotate(&self, value: f32) -> Vec2 {
        let rad = self.to_rad();
        vec2(value * rad.cos(), value * rad.sin())
    }
}

pub fn in_states<S: States>(states: Vec<S>) -> impl FnMut(Res<State<S>>) -> bool + Clone {
    move |current_state: Res<State<S>>| states.contains(current_state.get())
}

pub fn format_credits(credits: i16) -> String { format!("Credits: {:03}", credits) }

pub fn format_items(status: &ShipStatus) -> String { format!("M{} S{}", status.get(&Items::Missile), status.get(&Items::Shield)) }