use bevy::app::App;
use bevy::prelude::*;

pub mod upgrades;

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, upgrades::bounce_shots)
        ;
    }
}