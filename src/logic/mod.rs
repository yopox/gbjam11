use bevy::app::App;
use bevy::prelude::*;

pub use wave::ShipBundle;

use crate::logic::damage::DeathNote;
use crate::logic::hit::HitProcessingPlugin;

pub mod upgrades;
pub mod hit;
pub mod damage;
mod wave;

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, upgrades::bounce_shots)
            .add_plugins(HitProcessingPlugin)
            .add_plugins(DeathNote)
        ;
    }
}