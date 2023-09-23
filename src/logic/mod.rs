use bevy::app::App;
use bevy::prelude::*;

pub use item::Items;
pub use loot::Loot;
pub use wave::ShipBundle;
pub use wave::WaveCleared;

use crate::GameState;
use crate::logic::damage::DamagePlugin;
use crate::logic::hit::HitProcessingPlugin;
use crate::logic::loot::LootPlugin;
use crate::logic::wave::WavePlugin;

pub mod upgrades;
pub mod hit;
pub mod damage;
pub mod route;
mod wave;
mod movement;
mod loot;

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, upgrades::bounce_shots)
            .add_systems(Update, movement::apply_movement)
            .add_systems(PostUpdate, movement::despawn_far_ships)
            .add_plugins((HitProcessingPlugin, DamagePlugin, WavePlugin, LootPlugin))
        ;
    }
}