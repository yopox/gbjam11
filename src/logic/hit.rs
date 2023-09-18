use bevy::app::{App, Plugin};
use bevy::prelude::{Commands, Entity, Event, EventReader};
use bevy::prelude::*;

use crate::GameState;

pub struct HitProcessingPlugin;

impl Plugin for HitProcessingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<HitEvent>()
            .add_systems(PostUpdate, clear_shots.run_if(in_state(GameState::Space)))
        ;
    }
}

#[derive(Event)]
pub struct HitEvent {
    pub shot: Entity,
    pub ship: Entity,
}

fn clear_shots(
    mut commands: Commands,
    mut events: EventReader<HitEvent>,
) {
    for event in events.iter() {
        let entity = commands.get_entity(event.shot);
        if entity.is_some() {
            entity.unwrap().despawn_recursive();
        }
    }
}