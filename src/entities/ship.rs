use bevy::app::App;
use bevy::math::vec2;
use bevy::prelude::*;

use crate::GameState;
use crate::graphics::sizes::Hitbox;

pub struct ShipPlugin;

pub enum Ships {
    Player,
    Enemy,
}

impl Ships {
    pub fn hitbox(&self) -> Hitbox {
        match self {
            Ships::Player => Hitbox(vec2(16., 10.)),
            Ships::Enemy => Hitbox(vec2(12., 8.)),
        }
    }
}

#[derive(Component)]
pub struct Ship {
    model: Ships,
    pub friendly: bool,
    pub speed: f32,
    pub damage_factor: f32,
    pub shot_speed: f32,
    pub shot_frequency: f32,
}

impl Ship {
    fn new(model: Ships, friendly: bool) -> Self {
        Self {
            model, friendly,
            // Base stats
            speed: 0.5, damage_factor: 1.0, shot_speed: 1.0, shot_frequency: 1.0,
        }
    }

    fn with_speed(mut self, speed: f32) -> Self { self.speed = speed; self }
    fn with_damage_factor(mut self, damage_factor: f32) -> Self { self.damage_factor = damage_factor; self }
    fn with_shot_speed(mut self, shot_speed: f32) -> Self { self.shot_speed = shot_speed; self }
    fn with_shot_frequency(mut self, shot_frequency: f32) -> Self { self.shot_frequency = shot_frequency; self }

    pub fn from(model: Ships) -> Self {
        match model {
            Ships::Player => Ship::new(model, true)
                .with_speed(0.25)
                .with_shot_frequency(2.0),
            Ships::Enemy => Ship::new(model, false),
        }
    }

    pub fn sprite_index(&self) -> usize {
        match self.model {
            Ships::Player => 0,
            Ships::Enemy => 1,
        }
    }
}

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update.run_if(in_state(GameState::Space)))
        ;
    }
}

fn update(

) {

}