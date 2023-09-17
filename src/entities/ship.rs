use bevy::app::App;
use bevy::prelude::*;
use crate::GameState;

pub struct ShipPlugin;

pub enum Ships {
    Player,
}

#[derive(Component)]
pub struct Ship {
    pub friendly: bool,
    pub speed: f32,
    pub damage_factor: f32,
    pub shot_speed: f32,
}

impl Ship {
    pub fn from(model: Ships) -> Self {
        match model {
            Ships::Player => Ship { friendly: true, speed: 0.25, damage_factor: 1.0, shot_speed: 1.0 }
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