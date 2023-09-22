use bevy::app::{App, PostUpdate};
use bevy::prelude::{Component, EventReader, Plugin, Query, ResMut};

use crate::entities::Ship;
use crate::logic::damage::DamageEvent;
use crate::screens::Credits;

pub struct MoneyLaundryPlugin;

#[derive(Component)]
pub struct Loot {
    pub(crate) credits: u16
}
impl Plugin for MoneyLaundryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, credit_money);
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
            credits.as_mut().0 += loot.credits;
        }
    }
}