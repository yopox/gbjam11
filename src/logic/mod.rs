use bevy::app::App;
use bevy::prelude::*;

pub use money::Loot;
pub use wave::ShipBundle;
pub use wave::WaveCleared;

use crate::logic::damage::DamagePlugin;
use crate::logic::hit::HitProcessingPlugin;
use crate::logic::money::MoneyLaundryPlugin;
use crate::logic::wave::WavePlugin;

pub mod upgrades;
pub mod hit;
pub mod damage;
pub mod route;
mod wave;
mod movement;
mod money;

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, upgrades::bounce_shots)
            .add_systems(Update, movement::apply_movement)
            .add_plugins((HitProcessingPlugin, DamagePlugin, WavePlugin, MoneyLaundryPlugin))
        ;
    }
}