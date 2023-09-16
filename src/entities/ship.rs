use bevy::app::App;
use bevy::prelude::*;
use crate::GameState;
use crate::graphics::sizes::ShipSize;

pub struct ShipPlugin;

#[derive(Component)]
pub struct Ship {
    pub friendly: bool,
    pub size: ShipSize,
    pub speed: f32,
}

impl Ship {
    pub fn new(friendly: bool, size: ShipSize, speed: f32) -> Self { Ship { friendly, size, speed } }
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