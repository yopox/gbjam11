use bevy::app::{App, Plugin};
use bevy::prelude::{Commands, Entity, Event, EventReader};
use bevy::prelude::*;

use crate::entities::Ship;
use crate::GameState;

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<DamageEvent>()
            .add_systems(PostUpdate, die_gracefully.run_if(in_state(GameState::Space)))
        ;
    }
}

#[derive(Event)]
pub struct DamageEvent {
    pub ship: Entity,
    pub fatal: bool,
}

pub fn die_gracefully(
    mut commands: Commands,
    mut events: EventReader<DamageEvent>,
    mut ships: Query<&Ship>
) {
    for &DamageEvent { ship, fatal } in events.iter() {
        if !fatal { continue; }
        let entity = commands.get_entity(ship);
        if entity.is_some() && ships.get(ship).unwrap().health <= 0 {
            entity.unwrap().despawn_recursive();
        }
    }
}