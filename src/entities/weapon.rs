use std::f32::consts::PI;

use bevy::math::vec2;
use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::entities::Ship;
use crate::entities::shot::Shots;
use crate::graphics::Palette;

pub enum Weapons {
    Standard,
    Wave,
}

impl Weapons {
    pub fn shot_type(&self) -> Shots {
        match self {
            Weapons::Standard => Shots::Bullet,
            Weapons::Wave => Shots::Wave,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Weapon {
    pub shot: Shots,
    pub attack: f32,
    pub speed: Vec2,
    pub offset: Vec2,
    delay: usize,
}

impl Weapon {
    pub fn fires(&self, timer: usize) -> bool { timer % self.delay == 0 }

    pub fn sprite(&self, friendly: bool) -> TextureAtlasSprite {
        TextureAtlasSprite {
            index: self.shot.sprite_atlas_index(),
            anchor: Anchor::Center,
            color: if friendly { Palette::Greyscale.colors()[2] } else { Palette::Greyscale.colors()[1] },
            ..default()
        }
    }
}

/// Angle in degrees
#[derive(Copy, Clone)]
pub struct Angle(pub f32);

impl Weapon {
    fn new(model: Shots, ship: &Ship, offset: Vec2, angle: Angle) -> Self {
        Weapon {
            shot: model,
            attack: model.attack() * ship.damage_factor,
            speed: vec2(
                (angle.0 * PI / 180.).cos() * ship.shot_speed,
                (angle.0 * PI / 180.).sin() * ship.shot_speed
            ),
            offset,
            delay: (model.delay() as f32 / ship.shot_frequency).ceil() as usize
        }
    }
}

#[derive(Component)]
pub struct ShipWeapons {
    pub weapons: Vec<Weapon>,
    pub timer: usize,
}

impl ShipWeapons {
    pub fn new(ship: &Ship, weapons: Vec<(Weapons, Vec2, Angle)>) -> Self {
        ShipWeapons {
            weapons: weapons.iter().map(|(w, offset, angle)| Weapon::new(
                w.shot_type(),
                ship,
                offset.clone(),
                *angle,
            )).collect(),
            timer: 0,
        }
    }
}