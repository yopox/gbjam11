use bevy::app::{App, Plugin};
use bevy::prelude::{Commands, Entity, Event, EventReader};
use bevy::prelude::*;
use crate::entities::Ship;

use crate::GameState;

pub struct DeathNote;

impl Plugin for DeathNote {
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
}

fn die_gracefully(
    mut commands: Commands,
    mut events: EventReader<DamageEvent>,
    mut ships: Query<&Ship>
) {
    for event in events.iter() {
        let entity = commands.get_entity(event.ship);
        if entity.is_some() && ships.get(event.ship).unwrap().health <= 0. {
            entity.unwrap().despawn_recursive();
        }
    }
}