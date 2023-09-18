use crate::entities::Shots;

pub const WIDTH: usize = 160;
pub const HEIGHT: usize = 144;

pub const SCALE: f32 = 4.;

/// Min distance between the player and the screen border
pub const BORDER: f32 = 2.;

pub mod star_field {
    pub const INITIAL_SPEED: f32 = -0.5;
    pub const STARS_COUNT: usize = 50;
}

pub mod z_pos {
    pub const STAR_FIELD: f32 = 10.;
    pub const GUI: f32 = 20.;
    pub const SHIPS: f32 = 30.;
    pub const SHOTS: f32 = 31.;
}

pub mod base_stats {
    pub const SPEED: f32 = 0.5;
    pub const DAMAGE_FACTOR: f32 = 1.0;
    pub const SHOT_SPEED: f32 = 1.0;
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