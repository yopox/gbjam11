use bevy::math::vec2;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

use crate::entities::Ship;
use crate::entities::weapon::{ShipWeapons, Weapon};
use crate::GameState;
use crate::graphics::FakeTransform;
use crate::graphics::sizes::Hitbox;
use crate::logic::damage::damage_ship;
use crate::logic::hit;
use crate::logic::hit::HitEvent;
use crate::logic::upgrades::ShotUpgrades;
use crate::screens::Textures;
use crate::util::{HEIGHT, in_states, WIDTH, z_pos};

pub struct ShotsPlugin;

impl Plugin for ShotsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (shoot, update_shots, collide_shots)
                .run_if(in_states(vec![GameState::Space, GameState::Hangar])),
            )
            .add_systems(PostUpdate, damage_ship
                .before(hit::clear_shots)
                .run_if(in_state(GameState::Space)),
            )
        ;
    }
}

#[derive(Component)]
pub struct Shot {
    pub weapon: Weapon,
    pub friendly: bool,
}

#[derive(Copy, Clone)]
pub enum Shots {
    Bullet,
    Wave,
    Ball,
    Energy,
    DualBeam,
}

impl Shots {
    pub(crate) fn sprite_atlas_index(&self) -> usize {
        match self {
            Shots::Bullet => 0,
            Shots::Wave => 1,
            Shots::Ball => 2,
            Shots::Energy => 3,
            Shots::DualBeam => 4
        }
    }

    pub fn hitbox(&self) -> Hitbox {
        match self {
            Shots::Bullet => Hitbox(vec2(2., 2.)),
            Shots::Wave => Hitbox(vec2(6., 2.)),
            Shots::Ball => Hitbox(vec2(4., 4.)),
            Shots::Energy => Hitbox(vec2(2., 4.)),
            Shots::DualBeam => Hitbox(vec2(4., 4.)),
        }
    }
}

#[derive(Component)]
pub struct MuteShots;

fn shoot(
    mut commands: Commands,
    time: Res<Time>,
    mut ships: Query<(&Ship, &FakeTransform, &mut ShipWeapons, Option<&ShotUpgrades>), Without<MuteShots>>,
    textures: Res<Textures>,
) {
    for (ship, ship_pos, mut weapons, upgrades) in ships.iter_mut() {
        weapons.timer += time.delta_seconds();
        for weapon in &weapons.weapons {
            if weapon.fires(weapons.timer, time.delta_seconds()) {
                commands
                    .spawn(SpriteSheetBundle {
                        sprite: weapon.sprite(ship.friendly),
                        texture_atlas: textures.shots.clone(),
                        ..default()
                    })
                    .insert(Shot { weapon: weapon.clone(), friendly: ship.friendly })
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
    shots: Query<(&Shot, &Hitbox, &ShotUpgrades, &FakeTransform, Entity)>,
    ships: Query<(&Ship, &Hitbox, &FakeTransform, Entity)>,
    mut event_writer: EventWriter<HitEvent>,
) {
    for (shot, shot_hitbox, _, shot_pos, shot_entity) in &shots {
        for (ship, ship_hitbox, ship_pos, ship_entity) in &ships {
            if shot.friendly == ship.friendly { continue; }
            let collision = collide(
                shot_pos.translation,
                shot_hitbox.0,
                ship_pos.translation,
                ship_hitbox.0,
            );
            if collision.is_some() {
                event_writer.send(HitEvent { shot: shot_entity, ship: ship_entity });
            }
        }
    }
}