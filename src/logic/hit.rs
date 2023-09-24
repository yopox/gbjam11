use bevy::app::{App, Plugin};
use bevy::prelude::{Commands, Entity, Event, EventReader};
use bevy::prelude::*;

use crate::entities::Shot;
use crate::GameState;
use crate::logic::upgrades::{PIERCING, ShotUpgrades};
use crate::util::in_states;

pub struct HitProcessingPlugin;

impl Plugin for HitProcessingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<HitEvent>()
            .add_systems(PostUpdate, clear_shots.run_if(in_states(vec![GameState::Space, GameState::Elite, GameState::Boss])))
        ;
    }
}

#[derive(Event)]
pub struct HitEvent {
    pub shot: Entity,
    pub ship: Entity,
}

pub fn clear_shots(
    mut commands: Commands,
    mut shot: Query<&ShotUpgrades, With<Shot>>,
    mut events: EventReader<HitEvent>,
) {
    for event in events.iter() {
        let Ok(upgrades) = shot.get(event.shot) else { continue; };
        if upgrades.0 & PIERCING == 0 {
            commands.entity(event.shot).despawn_recursive();
        }
    }
}