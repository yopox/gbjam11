use bevy::app::{App, Plugin};
use bevy::prelude::{Commands, Entity, Event, EventReader};
use bevy::prelude::*;

use crate::entities::{Blink, MainShip, Ship, Shot};
use crate::GameState;
use crate::logic::hit::HitEvent;
use crate::logic::ShipStatus;

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

pub fn damage_ship(
    mut hit_events: EventReader<HitEvent>,
    mut ships: Query<(&mut Ship, Option<&MainShip>, Option<&Blink>)>,
    mut shots: Query<&Shot>,
    mut damage_event: EventWriter<DamageEvent>,
    mut ship_status: ResMut<ShipStatus>,
) {
    for HitEvent { ship, shot } in hit_events.iter() {
        if ships.contains(*ship) && shots.contains(*shot) {
            let (mut data, is_main_ship, is_blinking) = ships.get_mut(*ship).unwrap();

            // Main ship invulnerable if blinking
            // TODO all friendly ships?
            if is_main_ship.and(is_blinking).is_none() {
                let damage = shots.get(*shot).unwrap().weapon.attack;
                if data.health > 0.001 {
                    if data.health < damage { data.health = 0.; }
                    else { data.health -= damage; }
                    if is_main_ship.is_some() { ship_status.set_health(data.health); }
                    damage_event.send(DamageEvent { ship: *ship, fatal: data.health < 0.001 })
                }
            }
        }
    }
}

pub fn die_gracefully(
    mut commands: Commands,
    mut events: EventReader<DamageEvent>,
    mut ships: Query<&Ship>
) {
    for &DamageEvent { ship, fatal } in events.iter() {
        if !fatal { continue; }
        let entity = commands.get_entity(ship);
        if entity.is_some() && ships.get(ship).unwrap().health < 0.001 {
            entity.unwrap().despawn_recursive();
        }
    }
}