use bevy::app::{App, PostUpdate};
use bevy::prelude::{Component, EventReader, in_state, IntoSystemConfigs, Plugin, Query, ResMut};

use crate::{entities, GameState};
use crate::entities::Ship;
use crate::logic::damage;
use crate::logic::damage::DamageEvent;
use crate::screens::Credits;

pub struct LootPlugin;

#[derive(Component)]
pub struct Loot {
    pub(crate) credits: u16
}
impl Plugin for LootPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, credit_money
            .after(entities::damage_ship)
            .before(damage::die_gracefully)
            .run_if(in_state(GameState::Space))
        );
    }
}

fn credit_money(
    mut events: EventReader<DamageEvent>,
    ships: Query<(&Ship, &Loot)>,
    mut credits: ResMut<Credits>
) {
    for DamageEvent { ship, fatal} in events.iter() {
        if !fatal {
            continue
        }

        if let Ok((_ship, loot)) = ships.get(*ship) {
            credits.0 += loot.credits;
        }
    }
}