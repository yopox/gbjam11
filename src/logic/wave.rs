use bevy::app::App;
use bevy::math::vec2;
use bevy::prelude::*;

use crate::entities::{Angle, MainShip, Ship, Ships, ShipWeapons};
use crate::GameState;
use crate::graphics::FakeTransform;
use crate::graphics::sizes::Hitbox;
use crate::logic::movement::{Movement, Moves};
use crate::screens::Textures;
use crate::util::{WIDTH, z_pos};

pub struct WavePlugin;

#[derive(Component)]
struct WaveUI;

impl Plugin for WavePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update.run_if(in_state(GameState::Space)))
            .add_systems(OnEnter(GameState::Space), enter)
            .add_systems(OnExit(GameState::Space), exit)
        ;
    }
}

#[derive(Bundle)]
pub struct ShipBundle {
    sprite: SpriteSheetBundle,
    pos: FakeTransform,
    weapons: ShipWeapons,
    hitbox: Hitbox,
    ship: Ship,
}

impl ShipBundle {
    pub fn from(atlas: Handle<TextureAtlas>, model: Ships, pos: Vec2) -> ShipBundle {
        let ship = Ship::from(model);
        Self {
            sprite: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: ship.sprite_index(),
                    ..default()
                },
                texture_atlas: atlas,
                ..default()
            },
            pos: FakeTransform::from_xyz(pos.x, pos.y, z_pos::SHIPS),
            weapons: ShipWeapons::new(&ship, model.weapons()),
            hitbox: model.hitbox(),
            ship,
        }
    }
}

enum WaveEvent {
    Spawn(Ships, Moves),
    WaitSeconds(f32),
    WaitForClear,
}

#[derive(Resource)]
struct CurrentWave(Vec<WaveEvent>);

fn enter(
    mut commands: Commands,
) {
    commands.insert_resource(CurrentWave(vec![
        WaveEvent::Spawn(Ships::Enemy, Moves::Linear(vec2(-16., 90.), Angle(0.))),
        WaveEvent::WaitSeconds(5.),
        WaveEvent::Spawn(Ships::Enemy, Moves::Linear(vec2(WIDTH as f32 + 16., 110.), Angle(180.))),
    ]));
}

fn update(
    mut commands: Commands,
    time: Res<Time>,
    textures: Res<Textures>,
    mut wave: ResMut<CurrentWave>,
    ships: Query<&Ship, Without<MainShip>>,
) {
    let mut next = false;

    match wave.0.get_mut(0) {
        None => { /* TODO: Next screen */ }
        Some(WaveEvent::Spawn(model, moves)) => {
            commands
                .spawn(ShipBundle::from(
                    textures.ship.clone(),
                    model.clone(),
                    *moves.starting_pos(),
                ))
                .insert(Movement {
                    moves: moves.clone(),
                    t_0: time.elapsed_seconds(),
                })
            ;
            next = true;
        }
        Some(WaveEvent::WaitSeconds(ref mut s)) => {
            if *s > 0. { *s -= time.delta_seconds(); }
            else { next = true; }
        }
        Some(WaveEvent::WaitForClear) => {
            if ships.is_empty() { next = true; }
        }
    }

    if next { wave.0.remove(0); }
}

fn exit(
    mut commands: Commands,
    to_clean: Query<Entity, With<WaveUI>>,
) {
    for id in to_clean.iter() {
        commands
            .entity(id)
            .despawn_recursive();
    }
}