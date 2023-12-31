use bevy::math::vec2;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use rand::{Rng, RngCore, thread_rng};

use crate::entities::Ship;
use crate::entities::weapon::{ShipWeapons, Weapon};
use crate::GameState;
use crate::graphics::FakeTransform;
use crate::graphics::sizes::Hitbox;
use crate::logic::damage::damage_ship;
use crate::logic::hit;
use crate::logic::hit::HitEvent;
use crate::logic::upgrades::{PIERCING, ShotUpgrades, STUN};
use crate::music::{PlaySFXEvent, SFX};
use crate::screens::Textures;
use crate::util::{HEIGHT, in_states, upgrades, WIDTH, z_pos};

pub struct ShotsPlugin;

impl Plugin for ShotsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (shoot, update_shots, collide_shots)
                .run_if(in_states(vec![GameState::Space, GameState::Elite, GameState::Boss, GameState::Hangar])),
            )
            .add_systems(PostUpdate, damage_ship
                .before(hit::clear_shots)
                .run_if(in_states(vec![GameState::Space, GameState::Elite, GameState::Boss])),
            )
        ;
    }
}

#[derive(Component)]
pub struct Shot {
    pub weapon: Weapon,
    pub friendly: bool,
    pub bounce_count: u8,
    pub collisions: Vec<Entity>,
}

impl Shot {
    pub fn new(weapon: Weapon, friendly: bool) -> Self { Self {
        weapon, friendly, bounce_count: 0, collisions: vec![],
    } }
}

#[derive(Copy, Clone)]
pub enum Shots {
    Bullet,
    Wave,
    Missile,
    Energy,
    DualBeam,
}

impl Shots {
    pub(crate) fn sprite_atlas_index(&self) -> usize {
        match self {
            Shots::Bullet => 0,
            Shots::Wave => 1,
            Shots::Missile => 2,
            Shots::Energy => 3,
            Shots::DualBeam => 4
        }
    }

    pub fn hitbox(&self) -> Hitbox {
        match self {
            Shots::Bullet => Hitbox(vec2(2., 2.)),
            Shots::Wave => Hitbox(vec2(6., 2.)),
            Shots::Missile => Hitbox(vec2(4., 4.)),
            Shots::Energy => Hitbox(vec2(2., 4.)),
            Shots::DualBeam => Hitbox(vec2(4., 4.)),
        }
    }
}

#[derive(Component)]
pub struct MuteShots;

#[derive(Component)]
pub struct MuteShotsFor(pub f32);

fn shoot(
    mut commands: Commands,
    time: Res<Time>,
    mut ships: Query<(&Ship, &FakeTransform, &mut ShipWeapons, Option<&ShotUpgrades>), (Without<MuteShots>, Without<MuteShotsFor>)>,
    textures: Res<Textures>,
    mut sfx: EventWriter<PlaySFXEvent>,
) {
    for (ship, ship_pos, mut weapons, upgrades) in ships.iter_mut() {
        weapons.timer += time.delta_seconds();
        let mut fired = false;
        if !((-8.)..(WIDTH as f32 + 8.)).contains(&ship_pos.translation.x) { continue; }
        for weapon in &weapons.weapons {
            if weapon.fires(weapons.timer, time.delta_seconds()) {
                fired = true;
                commands
                    .spawn(SpriteSheetBundle {
                        sprite: weapon.sprite(ship.friendly),
                        texture_atlas: textures.shots.clone(),
                        ..default()
                    })
                    .insert(Shot::new(weapon.clone(), ship.friendly))
                    .insert(weapon.shot.hitbox())
                    .insert(ShotUpgrades(match upgrades {
                        Some(u) => u.0,
                        None => 0,
                    }))
                    .insert(FakeTransform::from_xyz(
                        ship_pos.translation.x + weapon.offset.x,
                        ship_pos.translation.y + weapon.offset.y,
                        z_pos::SHOTS,
                    ))
                ;
            }
        }
        if fired {
            sfx.send(PlaySFXEvent(if ship.friendly { SFX::ShipFire } else { SFX::EnemyFire }));
        }
    }
}

fn update_shots(
    mut commands: Commands,
    time: Res<Time>,
    mut shots: Query<(Entity, &Shot, &mut FakeTransform)>,
) {
    for (e, shot, mut pos) in shots.iter_mut() {
        // Move shot
        pos.translation.x += shot.weapon.speed.x * time.delta_seconds();
        pos.translation.y += shot.weapon.speed.y * time.delta_seconds();

        // Destroy shot
        if pos.translation.x > 2. * WIDTH as f32
            || pos.translation.x < -1. * WIDTH as f32
            || pos.translation.y > 2. * HEIGHT as f32
            || pos.translation.y < -1. * HEIGHT as f32 {
            commands.entity(e).despawn_recursive();
        }
    }
}

fn collide_shots(
    mut commands: Commands,
    mut shots: Query<(&mut Shot, &Hitbox, &ShotUpgrades, &FakeTransform, Entity)>,
    ships: Query<(&Ship, &Hitbox, &FakeTransform, Entity)>,
    mut event_writer: EventWriter<HitEvent>,
    mut sfx: EventWriter<PlaySFXEvent>,
) {
    let mut rng = thread_rng();

    for (mut shot, shot_hitbox, upgrades, shot_pos, shot_entity) in shots.iter_mut() {
        for (ship, ship_hitbox, ship_pos, ship_entity) in &ships {
            if shot.friendly == ship.friendly { continue; }
            if upgrades.0 & PIERCING != 0 && shot.collisions.contains(&ship_entity) { continue }
            let collision = collide(
                shot_pos.translation,
                shot_hitbox.0,
                ship_pos.translation,
                ship_hitbox.0,
            );
            if collision.is_some() {
                if upgrades.0 & STUN != 0 && rng.gen_range(0.0..1.0) < upgrades::STUN_CHANCE {
                    if !ship.model.is_elite() || rng.next_u32() % 2 == 0 {
                        if let Some(mut e) = commands.get_entity(ship_entity) {
                            e.insert(MuteShotsFor(upgrades::STUN_DURATION));
                            sfx.send(PlaySFXEvent(SFX::Error));
                        }
                    }
                }
                shot.collisions.push(ship_entity);
                event_writer.send(HitEvent { shot: shot_entity, ship: ship_entity });
            }
        }
    }
}