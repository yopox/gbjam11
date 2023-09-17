use bevy::math::vec2;
use bevy::prelude::Vec2;
use crate::entities::Ship;

pub struct Shot {
    /// Attack value (base value * ship multiplier)
    attack: f32,
    /// Speed (ship shot speed)
    speed: Vec2,
}

impl Shot {
    pub fn new(ship: &Ship, model: Shots, direction: Vec2) -> Self {
        Shot {
            attack: model.attack() * ship.damage_factor,
            speed: vec2(direction.x * ship.shot_speed, direction.y * ship.shot_speed),
        }
    }
}

pub enum Shots {
    Square,
}

impl Shots {
    fn attack(&self) -> f32 {
        match self {
            Shots::Square => 1.0,
        }
    }
}