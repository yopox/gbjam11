use bevy::app::{App, PostUpdate};
use bevy::prelude::{Component, EventReader, in_state, IntoSystemConfigs, Plugin, Query, ResMut};

use crate::entities::Ship;
use crate::GameState;
use crate::logic::{damage, ShipStatus};
use crate::logic::damage::DamageEvent;

pub struct LootPlugin;

#[derive(Component)]
pub struct Loot {
    pub(crate) credits: i16
}
impl Plugin for LootPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, credit_money
            .after(damage::damage_ship)
            .before(damage::die_gracefully)
            .run_if(in_state(GameState::Space))
        );
    }
}

fn credit_money(
    mut events: EventReader<DamageEvent>,
    ships: Query<(&Ship, &Loot)>,
    mut status: ResMut<ShipStatus>
) {
    for DamageEvent { ship, fatal} in events.iter() {
        if !fatal {
            continue
        }

        if let Ok((_ship, loot)) = ships.get(*ship) {
            status.add_credits(loot.credits);
        }
    }
}