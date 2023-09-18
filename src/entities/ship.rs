use bevy::app::App;
use bevy::math::vec2;
use bevy::prelude::*;

use crate::graphics::sizes::Hitbox;
use crate::logic::hit::HitEvent;
use crate::util::base_stats;
use crate::util::space::{BLINK_DURATION, BLINK_INTERVAL};

pub struct ShipPlugin;

pub enum Ships {
    Player,
    Enemy,
}

impl Ships {
    pub fn hitbox(&self) -> Hitbox {
        match self {
            Ships::Player => Hitbox(vec2(16., 10.)),
            Ships::Enemy => Hitbox(vec2(12., 8.)),
        }
    }
}

#[derive(Component)]
pub struct Ship {
    model: Ships,
    pub friendly: bool,
    pub speed: f32,
    pub damage_factor: f32,
    pub shot_speed: f32,
    pub shot_frequency: f32,
}

impl Ship {
    fn new(model: Ships, friendly: bool) -> Self {
        Self {
            model, friendly,
            // Base stats
            speed: base_stats::SPEED,
            damage_factor: base_stats::DAMAGE_FACTOR,
            shot_speed: base_stats::SHOT_SPEED,
            shot_frequency: base_stats::SHOT_FREQUENCY,
        }
    }

    fn with_speed(mut self, speed: f32) -> Self { self.speed = speed; self }
    fn with_damage_factor(mut self, damage_factor: f32) -> Self { self.damage_factor = damage_factor; self }
    fn with_shot_speed(mut self, shot_speed: f32) -> Self { self.shot_speed = shot_speed; self }
    fn with_shot_frequency(mut self, shot_frequency: f32) -> Self { self.shot_frequency = shot_frequency; self }

    pub fn from(model: Ships) -> Self {
        match model {
            Ships::Player => Ship::new(model, true)
                .with_speed(base_stats::SPEED / 2.)
                .with_shot_frequency(base_stats::SHOT_FREQUENCY * 2.),
            Ships::Enemy => Ship::new(model, false),
        }
    }

    pub fn sprite_index(&self) -> usize {
        match self.model {
            Ships::Player => 0,
            Ships::Enemy => 1,
        }
    }
}

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (add_blinking, blink))
        ;
    }
}

#[derive(Component)]
pub struct Blink(pub usize);

fn add_blinking(
    mut commands: Commands,
    mut hits: EventReader<HitEvent>,
) {
    for HitEvent { shot: _, ship } in hits.iter() {
        commands
            .entity(*ship)
            .insert(Blink(BLINK_DURATION));
    }
}

fn blink(
    mut commands: Commands,
    mut sprites: Query<(Entity, &mut Blink, &mut Visibility)>,
) {
    for (e, mut blink, mut vis) in sprites.iter_mut() {
        blink.0 -= 1;
        let new_vis = if (blink.0 / BLINK_INTERVAL) % 2 == 0 { Visibility::Inherited } else { Visibility::Hidden };
        vis.set_if_neq(new_vis);

        if blink.0 == 0 { commands.entity(e).remove::<Blink>(); }
    }
}