use bevy::app::App;
use bevy::prelude::*;
use crate::GameState;

pub struct ShipPlugin;

#[derive(Component)]
pub struct Ship {
    pub friendly: bool,
}

impl Ship {
    pub fn new(friendly: bool) -> Self { Ship { friendly } }
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