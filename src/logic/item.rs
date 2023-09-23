use bevy::prelude::{Commands, Res, Resource};
use bevy::utils::HashMap;
use rand::{RngCore, thread_rng};

use crate::entities::Ship;
use crate::logic::upgrades::Upgrades;
use crate::screens;
use crate::screens::Credits;
use crate::util::items;

#[derive(Resource)]
pub struct ShipStatus {
    inventory: HashMap<Items, usize>,
    health: usize,
    max_health: usize,
}

impl ShipStatus {
    pub fn add(&mut self, item: &Items) {
        if *item == Items::Repair {
            if self.health < self.max_health { self.health += 1; }
        }

        if let Some(mut n) = self.inventory.get_mut(item) {
            *n += 1;
        } else {
            self.inventory.insert(*item, 1);
        }
    }

    pub fn remove(&mut self, item: &Items) -> bool {
        if let Some(mut n) = self.inventory.get_mut(item) {
            if *n > 0 {
                *n -= 1;
                return true;
            }
        }
        return false;
    }

    pub fn get(&self, item: &Items) -> usize {
        *self.inventory.get(item).unwrap_or(&0)
    }

    pub fn damage(&mut self, damage: usize) {
        if self.health < damage { self.health = 0; }
        else { self.health -= damage; }
    }

    pub fn health(&self) -> (usize, usize) {
        (self.health, self.max_health)
    }

    pub fn set_health(&mut self, new_health: usize) {
        self.health = new_health;
    }

    pub fn is_max_health(&self) -> bool { self.health >= self.max_health }
}

pub fn reset_inventory(
    mut commands: Commands,
    selected_ship: Res<screens::SelectedShip>,
) {
    let ship = Ship::from(selected_ship.0.model());
    commands.insert_resource(ShipStatus {
        inventory: items::STARTING_ITEMS.clone(),
        health: ship.max_health,
        max_health: ship.max_health,
    });
    commands.insert_resource(Credits(0));
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