use bevy::prelude::{Commands, Res, Resource};
use bevy::utils::HashMap;
use rand::{RngCore, thread_rng};

use crate::entities::Ship;
use crate::logic::route::CurrentRoute;
use crate::logic::upgrades::{BOUNCING, PIERCING, STUN, Upgrades};
use crate::screens;
use crate::util::{items, upgrades};

#[derive(Resource)]
pub struct ShipStatus {
    inventory: HashMap<Items, usize>,
    upgrades: Vec<Upgrades>,
    health: f32,
    max_health: f32,
    credits: i16,
}

impl ShipStatus {
    pub fn add(&mut self, item: &Items) {
        if *item == Items::Repair {
            if self.health < self.max_health { self.health += 1.; }
            if self.health > self.max_health { self.health = self.max_health; }
            return;
        }

        if let Items::Upgrade(u) = *item {
            self.upgrades.push(u);
            if u == Upgrades::Hull {
                self.max_health += 4.;
                self.health += 4.;
            }
            return;
        }

        if let Some(n) = self.inventory.get_mut(item) {
            *n += 1;
        } else {
            self.inventory.insert(*item, 1);
        }
    }

    pub fn remove(&mut self, item: &Items) -> bool {
        if let Some(n) = self.inventory.get_mut(item) {
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

    pub fn health(&self) -> (f32, f32) {
        (self.health, self.max_health)
    }

    pub fn set_health(&mut self, new_health: f32) {
        self.health = new_health;
    }

    pub fn is_max_health(&self) -> bool { self.health >= self.max_health }

    pub fn speed_multiplier(&self) -> f32 {
        1. + self.upgrades.iter().map(
            |u| if *u == Upgrades::Speed { upgrades::SPEED } else { 0. }
        ).sum::<f32>()
    }

    pub fn damage_multiplier(&self) -> f32 {
        1. + self.upgrades.iter().map(
            |u| if *u == Upgrades::Damage { upgrades::DAMAGE } else { 0. }
        ).sum::<f32>()
    }

    pub fn shot_speed_multiplier(&self) -> f32 {
        1. + self.upgrades.iter().map(
            |u| if *u == Upgrades::ShotSpeed { upgrades::SHOT_SPEED } else { 0. }
        ).sum::<f32>()
    }

    pub fn shot_frequency_multiplier(&self) -> f32 {
        1. + self.upgrades.iter().map(
            |u| if *u == Upgrades::ShotFrequency { upgrades::SHOT_FREQUENCY } else { 0. }
        ).sum::<f32>()
    }

    pub fn shot_upgrades(&self) -> usize {
        let mut modifier = 0;
        if self.upgrades.contains(&Upgrades::BouncingShots) { modifier |= BOUNCING; }
        if self.upgrades.contains(&Upgrades::PiercingShots) { modifier |= PIERCING; }
        if self.upgrades.contains(&Upgrades::StunShots) { modifier |= STUN; }
        modifier
    }

    pub fn has_upgrade(&self, upgrade: Upgrades) -> bool { self.upgrades.contains(&upgrade) }

    pub fn is_berserk(&self) -> bool {
        self.has_upgrade(Upgrades::Berserk) && self.health / self.max_health <= upgrades::BERSERK
    }

    pub fn non_stat_upgrades(&self) -> Vec<&Upgrades> {
        self.upgrades.iter().filter(|u| !u.is_stat_upgrade()).collect()
    }

    pub fn get_credits(&self) -> i16 { self.credits }
    pub fn add_credits(&mut self, gain: i16) { self.credits += gain; }
    pub fn buy(&mut self, cost: i16) {
        self.credits -= cost;
    }
}

pub fn reset_inventory(
    mut commands: Commands,
    route: Res<CurrentRoute>,
    selected_ship: Res<screens::SelectedShip>,
) {
    let ship = Ship::from(selected_ship.0.model());
    commands.insert_resource(ShipStatus {
        inventory: items::STARTING_ITEMS[&route.mode].clone().into_iter().collect(),
        upgrades: vec![],
        health: ship.max_health,
        max_health: ship.max_health,
        credits: items::STARTING_CREDITS,
    });
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
}