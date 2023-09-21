use bevy::app::App;
use bevy::prelude::*;

pub use ship::MainShip;
pub use ship::Ship;
pub use ship::Ships;
pub use shot::MuteShots;
pub use shot::Shot;
pub use shot::Shots;
pub use weapon::ShipWeapons;
pub use weapon::Weapons;

use crate::entities::ship::ShipPlugin;
use crate::entities::shot::ShotsPlugin;

mod ship;
mod weapon;
mod shot;

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((ShipPlugin, ShotsPlugin))
        ;
    }
}