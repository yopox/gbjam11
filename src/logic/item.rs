use bevy::prelude::{Commands, Resource};
use bevy::utils::HashMap;
use rand::{RngCore, thread_rng};

use crate::logic::upgrades::Upgrades;
use crate::screens::Credits;
use crate::util::items;

#[derive(Resource)]
pub struct Inventory(pub HashMap<Items, usize>);

pub fn reset_inventory(
    mut commands: Commands,
) {
    commands.insert_resource(Inventory(items::STARTING_ITEMS.clone()));
    commands.insert_resource(Credits(120));
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Items {
    Missile,
    Shield,
    Repair,
    Upgrade(Upgrades)
}

impl Items {
    pub fn name(&self) -> &str {
        match self {
            Items::Missile => "Missile",
            Items::Shield => "Shield",
            Items::Repair => "Repair",
            Items::Upgrade(u) => u.name(),
        }
    }

    pub fn random_collectible() -> Self {
        let mut rng = thread_rng();
        if rng.next_u32() % 2 == 0 { Items::Missile } else { Items::Shield }
    }

    pub fn random_upgrade() -> Self {
        let mut rng = thread_rng();
        if rng.next_u32() % 3 == 0 {
            Items::Upgrade(Upgrades::random_non_stat_upgrade())
        } else {
            Items::Upgrade(Upgrades::random_stat_upgrade())
        }
    }
}