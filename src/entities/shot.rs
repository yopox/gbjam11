use bevy::prelude::*;

use crate::entities::Ship;
use crate::entities::weapon::{ShipWeapons, Weapon};
use crate::GameState;
use crate::graphics::FakeTransform;
use crate::screens::Textures;
use crate::util::{HEIGHT, WIDTH, z_pos};

pub struct ShotsPlugin;

impl Plugin for ShotsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (shoot, update_shots)
                .run_if(in_state(GameState::Space))
            )
        ;
    }
}

#[derive(Component)]
pub struct Shot(pub Weapon);

#[derive(Copy, Clone)]
pub enum Shots {
    Bullet,
    Wave,
    Ball,
    Energy,
    DualBeam,
}

impl Shots {
    pub(crate) fn attack(&self) -> f32 {
        match self {
            Shots::Bullet => 1.0,
            Shots::Wave => 0.5,
            Shots::Ball => 1.5,
            Shots::Energy => 1.25,
            Shots::DualBeam => 1.75,
        }
    }

    pub(crate) fn delay(&self) -> usize {
        match self {
            _ => 32
        }
    }

    pub(crate) fn sprite_atlas_index(&self) -> usize {
        match self {
            Shots::Bullet => 0,
            Shots::Wave => 1,
            Shots::Ball => 2,
            Shots::Energy => 3,
            Shots::DualBeam => 4
        }
    }
}

fn shoot(
    mut commands: Commands,
    mut ships: Query<(&Ship, &FakeTransform, &mut ShipWeapons)>,
    textures: Res<Textures>,
) {
    for (ship, ship_pos, mut weapons) in ships.iter_mut() {
        weapons.timer += 1;
        for weapon in &weapons.weapons {
            if weapon.fires(weapons.timer) {
                commands
                    .spawn(SpriteSheetBundle {
                        sprite: weapon.sprite(ship.friendly),
                        texture_atlas: textures.shots.clone(),
                        ..default()
                    })
                    .insert(Shot(weapon.clone()))
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
    mut shots: Query<(Entity, &Shot, &mut FakeTransform)>,
) {
    for (e, shot, mut pos) in shots.iter_mut() {
        // Move shot
        pos.translation.x += shot.0.speed.x;
        pos.translation.y += shot.0.speed.y;

        // Destroy shot
        if pos.translation.x > 2. * WIDTH as f32
            || pos.translation.x < -1. * WIDTH as f32
            || pos.translation.y > 2. * HEIGHT as f32
            || pos.translation.y < -1. * HEIGHT as f32 {
            commands.entity(e).despawn_recursive();
        }
    }
}