use bevy::app::{App, Plugin};
use bevy::prelude::{Commands, Entity, Event, EventReader};
use bevy::prelude::*;

use crate::entities::{Blink, MainShip, MuteShots, Ship, Shot};
use crate::GameState;
use crate::graphics::ScreenTransition;
use crate::logic::{EliteKilled, ShipStatus, WaveCleared};
use crate::logic::hit::HitEvent;
use crate::music::{PlaySFXEvent, SFX};
use crate::util::{in_states, space};

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<DamageEvent>()
            .add_systems(Update, elite_cleared)
            .add_systems(PostUpdate, (die_gracefully, despawn_ships)
                .run_if(in_states(vec![GameState::Space, GameState::Elite, GameState::Boss]))
            )
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
    mut sfx: EventWriter<PlaySFXEvent>,
) {
    let mut hit = None;

    for HitEvent { ship, shot } in hit_events.iter() {
        if ships.contains(*ship) && shots.contains(*shot) {
            let (mut data, is_main_ship, is_blinking) = ships.get_mut(*ship).unwrap();

            // Main ship invulnerable if blinking
            // TODO all friendly ships?
            if is_main_ship.and(is_blinking).is_none() {
                let damage = shots.get(*shot).unwrap().weapon.attack;
                if data.health > 0.001 {
                    hit = Some(data.friendly);
                    if data.health < damage { data.health = 0.; }
                    else { data.health -= damage; }
                    if is_main_ship.is_some() { ship_status.set_health(data.health); }
                    damage_event.send(DamageEvent { ship: *ship, fatal: data.health < 0.001 })
                }
            }
        }
    }

    match hit {
        None => {}
        Some(true) => { sfx.send(PlaySFXEvent(SFX::ShipHit)); }
        Some(false) => { sfx.send(PlaySFXEvent(SFX::EnemyHit)); }
    }
}

#[derive(Component)]
pub struct Dead;

pub fn elite_cleared(
    mut commands: Commands,
    mut ships: Query<(Entity, &Ship), Without<Dead>>,
    mut elite_killed: EventReader<EliteKilled>,
    shots: Query<Entity, With<Shot>>,
) {
    if elite_killed.is_empty() { return; }
    elite_killed.clear();

    for (e, ship) in ships.iter() {
        if ship.friendly { continue; }
        commands
            .entity(e)
            .insert(Blink(space::BLINK_DURATION_ENEMY))
            .insert(Dead)
            .insert(MuteShots)
        ;
    }

    shots.for_each(|e| commands.entity(e).despawn_recursive());
}

pub fn die_gracefully(
    mut commands: Commands,
    mut events: EventReader<DamageEvent>,
    mut transition: ResMut<ScreenTransition>,
    mut elite_killed: EventWriter<EliteKilled>,
    ships: Query<(&Ship, Option<&MainShip>)>,
    mut sfx: EventWriter<PlaySFXEvent>,
) {
    for &DamageEvent { ship, fatal } in events.iter() {
        if !fatal { continue; }
        let entity = commands.get_entity(ship);
        if entity.is_some() {
            let (ship, main) = ships.get(ship).unwrap();
            if ship.health < 0.001 {
                entity.unwrap()
                    .insert(Dead)
                    .insert(MuteShots)
                ;
                if main.is_some() { sfx.send(PlaySFXEvent(SFX::Die)); transition.set_if_neq(ScreenTransition::to(GameState::GameOver)); }
                if ship.model.is_elite() { elite_killed.send(EliteKilled); }
            }
        }
    }
}

pub fn despawn_ships(
    mut commands: Commands,
    mut wave_cleared: EventWriter<WaveCleared>,
    dead: Query<(Entity, &Ship), (With<Dead>, Without<Blink>)>,
) {
    for (e, ship) in dead.iter() {
        commands.entity(e).despawn_recursive();
        if ship.model.is_elite() { wave_cleared.send(WaveCleared); }
    }
}