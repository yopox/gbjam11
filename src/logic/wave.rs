use bevy::app::App;
use bevy::math::vec2;
use bevy::prelude::*;

use crate::entities::{MainShip, Ship, Ships, ShipWeapons};
use crate::GameState;
use crate::graphics::FakeTransform;
use crate::graphics::sizes::Hitbox;
use crate::logic::Loot;
use crate::logic::movement::{Movement, Moves};
use crate::logic::route::CurrentRoute;
use crate::screens::Textures;
use crate::util::{Angle, HALF_WIDTH, z_pos};

pub struct WavePlugin;

#[derive(Component)]
struct WaveUI;

impl Plugin for WavePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<WaveCleared>()
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

impl CurrentWave {
    pub fn new(difficulty: usize) -> Self {
        let mut wave = vec![];

        // TODO: generate wave
        // wave.push(WaveEvent::Spawn(Ships::Enemy, Moves::Wavy(vec2(-16., 90.), Angle(0.), 2., 20.)));
        // wave.push(WaveEvent::WaitSeconds(5.));
        // wave.push(WaveEvent::Spawn(Ships::Enemy, Moves::Triangular(vec2(WIDTH as f32 + 16., 110.), Angle(180.), 0.4,20.)));

        wave.push(WaveEvent::Spawn(Ships::Enemy, Moves::WithPause(
            HALF_WIDTH, 2., 0., Box::new(
                Moves::Wavy(vec2(-16., 90.), Angle(0.), 2., 20.)
            ))));

        // Always end wave with [WaveEvent::WaitForClear]
        wave.push(WaveEvent::WaitForClear);
        CurrentWave(wave)
    }
}

#[derive(Event)]
pub struct WaveCleared;

fn enter(
    mut commands: Commands,
    route: Res<CurrentRoute>,
) {
    info!("Start wave with difficulty {}", route.level);
    commands.insert_resource(CurrentWave::new(route.level));
}

fn update(
    mut commands: Commands,
    time: Res<Time>,
    textures: Res<Textures>,
    mut wave: ResMut<CurrentWave>,
    ships: Query<&Ship, Without<MainShip>>,
    mut cleared: EventWriter<WaveCleared>,
) {
    let mut next = false;

    match wave.0.get_mut(0) {
        None => {}
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
                // TODO customize credit count
                .insert(Loot { credits: 2 })
            ;
            next = true;
        }
        Some(WaveEvent::WaitSeconds(ref mut s)) => {
            if *s > 0. { *s -= time.delta_seconds(); }
            else { next = true; }
        }
        Some(WaveEvent::WaitForClear) => {
            if ships.is_empty() {
                next = true;
                if wave.0.len() == 1 { cleared.send(WaveCleared); }
            }
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