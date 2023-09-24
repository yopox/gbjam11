use bevy::app::App;
use bevy::math::vec2;
use bevy::prelude::*;
use rand::{Rng, thread_rng};

use crate::entities::Weapons;
use crate::graphics::sizes::Hitbox;
use crate::logic::damage::DamageEvent;
use crate::logic::route::Route;
use crate::util::{Angle, base_stats};
use crate::util::space::{BLINK_DURATION, BLINK_DURATION_ENEMY, BLINK_INTERVAL};

pub struct ShipPlugin;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Ships {
    Player(u8),
    Invader(u8),
    Elite(u8),
}

impl Ships {
    pub fn is_elite(&self) -> bool {
        match self {
            Ships::Elite(_) => true,
            _ => false,
        }
    }

    pub fn random_enemy(level: usize) -> Self {
        let mut rng = thread_rng();
        let possible = [
            vec![0, 1, 2, 3],
            vec![0, 1, 2, 3, 4, 4, 5, 5, 6, 6],
            vec![0, 1, 2, 3, 4, 4, 5, 5, 6, 6, 7, 7, 7, 8, 8, 8],
        ];
        let act = level / Route::act_len();
        let possible = possible[act].to_owned();
        return Ships::Invader(possible[rng.gen_range(0..possible.len())]);
    }

    pub fn hitbox(&self) -> Hitbox {
        match self {
            Ships::Player(_) => Hitbox(vec2(6., 4.)),
            Ships::Invader(_) => Hitbox(vec2(12., 6.)),
            Ships::Elite(_) => Hitbox(vec2(8., 4.)),
        }
    }

    pub fn weapons(&self) -> Vec<(Weapons, Vec2, Angle)> {
        match self {
            Ships::Player(0) => vec![
                (Weapons::Standard, vec2(-4., 6.), Angle(90.)),
                (Weapons::Standard, vec2(4., 6.), Angle(90.)),
            ],
            Ships::Player(1) => vec![
                (Weapons::Wave, vec2(-4., 6.), Angle(90.)),
                (Weapons::Wave, vec2(4., 6.), Angle(90.)),
            ],
            Ships::Player(2) => vec![
                (Weapons::Ball, vec2(-4., 6.), Angle(115.)),
                (Weapons::Ball, vec2(0., 6.), Angle(90.)),
                (Weapons::Ball, vec2(4., 6.), Angle(65.)),
            ],
            Ships::Player(_) => vec![
                (Weapons::Energy, vec2(0., 6.), Angle(90.)),
            ],
            Ships::Elite(n) => Ships::Player(*n)
                .weapons()
                .iter()
                .map(|&(w, pos, Angle(a))| (w, vec2(pos.x, -pos.y), Angle(180. + a)))
                .collect()
            ,
            Ships::Invader(0) | Ships::Invader(2) => vec![
                (Weapons::Standard, vec2(0., -3.), Angle(270.)),
            ],
            Ships::Invader(1) => vec![
                (Weapons::Standard, vec2(-2., -3.), Angle(270.)),
                (Weapons::Standard, vec2(2., -3.), Angle(270.)),
            ],
            Ships::Invader(3) => vec![
                (Weapons::Standard, vec2(-5., -3.), Angle(225.)),
                (Weapons::Standard, vec2(5., -3.), Angle(315.)),
            ],
            Ships::Invader(4) | Ships::Invader(7) => vec![
                (Weapons::Dual, vec2(0., -3.), Angle(270.)),
            ],
            Ships::Invader(5) => vec![
                (Weapons::Dual, vec2(0., -3.), Angle(270.)),
                (Weapons::Standard, vec2(-5., -3.), Angle(225.)),
                (Weapons::Standard, vec2(5., -3.), Angle(315.)),
            ],
            Ships::Invader(6) => vec![
                (Weapons::Standard, vec2(0., -3.), Angle(270.)),
                (Weapons::Standard, vec2(-5., -3.), Angle(225.)),
                (Weapons::Standard, vec2(5., -3.), Angle(315.)),
            ],
            Ships::Invader(8) => vec![
                (Weapons::Standard, vec2(-5., -3.), Angle(270.)),
                (Weapons::Standard, vec2(5., -3.), Angle(270.)),
            ],
            _ => vec![],
        }
    }
}

#[derive(Component)]
pub struct Ship {
    pub(crate) model: Ships,
    pub friendly: bool,
    pub speed: f32,
    pub damage_factor: f32,
    pub shot_speed: f32,
    pub shot_frequency: f32,
    pub health: f32,
    pub max_health: f32,
}

impl Ship {
    fn new(model: Ships, friendly: bool) -> Self {
        Self {
            model, friendly,
            // Base stats
            health: base_stats::HEALTH,
            max_health: base_stats::HEALTH,
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
    fn with_health(mut self, health: f32) -> Self { self.health = health; self.max_health = health; self }

    pub fn from(model: Ships) -> Self {
        match model {
            Ships::Player(0) => Ship::new(model, true)
                .with_health(base_stats::HEALTH * 1.5)
            ,
            Ships::Elite(0) => Ship::new(model, false)
                .with_health(base_stats::HEALTH * 1.75)
            ,
            Ships::Player(1) => Ship::new(model, true)
                .with_health(base_stats::HEALTH * 2.5)
            ,
            Ships::Elite(1) => Ship::new(model, false)
                .with_health(base_stats::HEALTH * 2.5)
            ,
            Ships::Player(2) => Ship::new(model, true)
                .with_health(base_stats::HEALTH * 1.5)
                .with_speed(base_stats::SPEED * 1.5)
            ,
            Ships::Elite(2) => Ship::new(model, false)
                .with_health(base_stats::HEALTH * 1.75)
            ,
            Ships::Player(_) => Ship::new(model, true)
                .with_health(base_stats::HEALTH * 0.75)
            ,
            Ships::Elite(_) => Ship::new(model, false)
                .with_health(base_stats::HEALTH * 1.25)
            ,
            Ships::Invader(0) => Ship::new(model, false)
                .with_health(base_stats::HEALTH / 2.)
                .with_speed(base_stats::SPEED / 2.)
            ,
            Ships::Invader(1) => Ship::new(model, false)
                .with_health(base_stats::HEALTH / 1.5)
                .with_speed(base_stats::SPEED / 3.)
            ,
            Ships::Invader(2) => Ship::new(model, false)
                .with_health(base_stats::HEALTH / 2.)
                .with_speed(base_stats::SPEED)
                .with_shot_frequency(base_stats::SHOT_FREQUENCY * 1.5)
            ,
            Ships::Invader(3) => Ship::new(model, false)
                .with_health(base_stats::HEALTH / 1.5)
                .with_speed(base_stats::SPEED / 2.)
            ,
            Ships::Invader(4) => Ship::new(model, false)
                .with_health(base_stats::HEALTH)
                .with_speed(base_stats::SPEED / 1.5)
            ,
            Ships::Invader(5) => Ship::new(model, false)
                .with_health(base_stats::HEALTH / 2.0)
                .with_speed(base_stats::SPEED * 1.25)
                .with_damage_factor(base_stats::DAMAGE_FACTOR * 2.0)
            ,
            Ships::Invader(6) => Ship::new(model, false)
                .with_health(base_stats::HEALTH * 2.)
                .with_speed(base_stats::SPEED / 2.)
                .with_shot_frequency(base_stats::SHOT_FREQUENCY / 1.5)
                .with_damage_factor(base_stats::DAMAGE_FACTOR * 1.5)
            ,
            Ships::Invader(7) => Ship::new(model, false)
                .with_health(base_stats::HEALTH * 1.5)
                .with_speed(base_stats::SPEED * 1.25)
                .with_damage_factor(base_stats::DAMAGE_FACTOR * 1.5)
            ,
            Ships::Invader(8) => Ship::new(model, false)
                .with_health(base_stats::HEALTH * 1.5)
                .with_speed(base_stats::SPEED / 1.5)
                .with_shot_frequency(base_stats::SHOT_FREQUENCY * 2.0)
                .with_damage_factor(base_stats::DAMAGE_FACTOR * 1.5)
            ,
            Ships::Invader(_) => Ship::new(model, false),
        }
    }

    pub fn sprite_index(&self) -> usize {
        match self.model {
            Ships::Player(n) => n as usize,
            Ships::Invader(n) => 4 + n as usize,
            Ships::Elite(n) => 13 + n as usize,
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
pub struct MainShip;

#[derive(Component)]
pub struct Blink(pub usize);

fn add_blinking(
    mut commands: Commands,
    mut hits: EventReader<DamageEvent>,
    ships: Query<&Ship>,
) {
    for &DamageEvent { ship: ship_entity, fatal } in hits.iter() {
        let Ok(ship) = ships.get(ship_entity) else { continue };
        let friendly = ship.friendly;
        let Some(mut ship_commands) = commands.get_entity(ship_entity) else { continue; };
        ship_commands.insert(Blink(if friendly { BLINK_DURATION } else { BLINK_DURATION_ENEMY }));
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