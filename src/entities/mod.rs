mod ship;

use bevy::app::App;
use bevy::prelude::*;
use crate::entities::ship::ShipPlugin;
pub use ship::Ship;

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(ShipPlugin)
        ;
    }
}