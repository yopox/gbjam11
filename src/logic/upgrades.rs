use bevy::prelude::Component;

pub const BOUNCING: usize = 1 << 1;
pub const PIERCING: usize = 1 << 2;
pub const LEECH: usize = 1 << 3;
pub const HOMING: usize = 1 << 4;
pub const STUN: usize = 1 << 5;
pub const SLOWING: usize = 1 << 6;
pub const SPLIT: usize = 1 << 7;
pub const CURLY: usize = 1 << 8;

#[derive(Component, Copy, Clone, Default)]
pub struct ShotUpgrades(pub usize);