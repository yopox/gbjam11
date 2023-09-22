use std::f32::consts::PI;

use bevy::math::{Vec2, vec2};
use bevy::prelude::{Res, State, States};

use crate::entities::Shots;

pub const WIDTH: usize = 160;
pub const HEIGHT: usize = 144;

pub const SCALE: f32 = 4.;

/// Min distance between the player and the screen border
pub const BORDER: f32 = 2.;

pub mod space {
    use crate::util::{base_stats, HEIGHT};

    pub const BLINK_INTERVAL: usize = 20;
    pub const BLINK_DURATION: usize = BLINK_INTERVAL * 6;

    pub const NEXT_LEVEL_SPEED_Y: f32 = -12.;
    pub const NEXT_LEVEL_CHOICE_Y: f32 = HEIGHT as f32 / 2.;
    pub const RUSH_SPEED_Y: f32 = base_stats::SPEED * 10.;
}

pub mod star_field {
    use bevy::math::vec2;
    use bevy::prelude::Vec2;

    pub const INITIAL_SPEED: Vec2 = vec2(0., -30.);
    pub const HANGAR_SPEED: Vec2 = vec2(0., INITIAL_SPEED.y / 3.);
    pub const RUSH_SPEED: Vec2 = vec2(0., INITIAL_SPEED.y * 12.);
    pub const STARS_COUNT: usize = 50;
}

pub mod hud {
    pub const HEALTH_BAR_SIZE: usize = 32;
}

pub mod z_pos {
    pub const STAR_FIELD: f32 = 10.;
    pub const GUI: f32 = 20.;
    pub const SHIPS: f32 = 30.;
    pub const SHOTS: f32 = 31.;
    pub const HANGAR: f32 = 50.;
    pub const HANGAR_TEXT: f32 = 51.;
}

pub mod base_stats {
    pub const HEALTH: usize = 9;
    pub const SPEED: f32 = 25.;
    pub const DAMAGE_FACTOR: f32 = 1.0;
    pub const SHOT_SPEED: f32 = 100.;
    pub const SHOT_FREQUENCY: f32 = 1.0;
}

impl Shots {
    pub fn attack(&self) -> f32 {
        match self {
            Shots::Bullet => 1.0,
            Shots::Wave => 0.5,
            Shots::Ball => 1.5,
            Shots::Energy => 1.25,
            Shots::DualBeam => 1.75,
        }
    }

    pub fn delay(&self) -> usize {
        match self {
            _ => 120
        }
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