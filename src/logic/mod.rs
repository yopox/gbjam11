use bevy::app::App;
use bevy::prelude::*;

pub use item::Items;
pub use item::ShipStatus;
pub use loot::Loot;
pub use wave::EliteKilled;
pub use wave::ShipBundle;
pub use wave::WaveCleared;

use crate::GameState;
use crate::logic::damage::DamagePlugin;
use crate::logic::hit::HitProcessingPlugin;
use crate::logic::loot::LootPlugin;
use crate::logic::wave::WavePlugin;
use crate::util::in_states;

pub mod upgrades;
pub mod hit;
pub mod damage;
pub mod route;
mod wave;
mod movement;
mod loot;
mod item;
mod elite;

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnExit(GameState::Hangar), item::reset_inventory)
            .add_systems(Update, (upgrades::bounce_shots, upgrades::leech)
                .run_if(in_states(vec![GameState::Space, GameState::Elite, GameState::Boss]))
            )
            .add_systems(Update, movement::apply_movement)
            .add_systems(PostUpdate, movement::despawn_far_ships)
            .add_plugins((HitProcessingPlugin, DamagePlugin, WavePlugin, LootPlugin))
        ;
    }
}