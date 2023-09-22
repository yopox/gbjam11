use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::entities::Ship;
use crate::entities::shot::Shots;
use crate::graphics::Palette;
use crate::util::Angle;

pub enum Weapons {
    Standard,
    Wave,
    Ball,
    Energy,
}

impl Weapons {
    pub fn shot_type(&self) -> Shots {
        match self {
            Weapons::Standard => Shots::Bullet,
            Weapons::Wave => Shots::Wave,
            Weapons::Ball => Shots::Ball,
            Weapons::Energy => Shots::Energy,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Weapon {
    pub shot: Shots,
    pub attack: f32,
    pub speed: Vec2,
    pub offset: Vec2,
    delay: f32,
}

impl Weapon {
    pub fn fires(&self, timer: f32, delta: f32) -> bool { ((timer - delta) % self.delay) > timer % self.delay }

    pub fn sprite(&self, friendly: bool) -> TextureAtlasSprite {
        TextureAtlasSprite {
            index: self.shot.sprite_atlas_index(),
            anchor: Anchor::Center,
            color: if friendly { Palette::Greyscale.colors()[2] } else { Palette::Greyscale.colors()[1] },
            ..default()
        }
    }
}

impl Weapon {
    fn new(model: Shots, ship: &Ship, offset: Vec2, angle: Angle) -> Self {
        Weapon {
            shot: model,
            attack: model.attack() * ship.damage_factor,
            speed: angle.rotate(ship.shot_speed),
            offset,
            delay: model.delay() / ship.shot_frequency,
        }
    }
}

#[derive(Component)]
pub struct ShipWeapons {
    pub weapons: Vec<Weapon>,
    pub timer: f32,
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
            timer: 0.,
        }
    }
}