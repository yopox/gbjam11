use bevy::app::App;
use bevy::prelude::*;
use rand::{Rng, thread_rng};

use crate::entities::{MainShip, Ship, Ships, ShipWeapons};
use crate::GameState;
use crate::graphics::FakeTransform;
use crate::graphics::sizes::Hitbox;
use crate::logic::Loot;
use crate::logic::movement::{Movement, Moves};
use crate::logic::route::CurrentRoute;
use crate::logic::wave::WaveEvent::WaitSeconds;
use crate::screens::Textures;
use crate::util::{HALF_WIDTH, space, z_pos};

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

#[derive(Copy, Clone)]
enum WavePart {
    SimpleEnemy,
    ConsecutiveWithPause(u8, f32, f32),
}

impl WavePart {
    fn events(&self, level: usize) -> Vec<WaveEvent> {
        let mut events = vec![];
        match self {
            WavePart::SimpleEnemy => {
                events.push(WaveEvent::Spawn(
                    Ships::random_enemy(level),
                    Moves::random_crossing(),
                ));
            }
            WavePart::ConsecutiveWithPause(n, x, pause) => {
                let base_move = Moves::random_crossing();
                for _ in 0..*n {
                    events.push(WaveEvent::Spawn(
                        Ships::random_enemy(level),
                        Moves::WithPause(*x, *pause, 0., Box::new(base_move.clone())),
                    ));
                    events.push(WaitSeconds(*pause + 2.));
                }
            }
        }
        events
    }

    fn random(level: usize) -> Self {
        let mut rng = thread_rng();
        let possible_parts = match level {
            0..=8 => [
                WavePart::SimpleEnemy,
                WavePart::ConsecutiveWithPause(2, HALF_WIDTH, 4.),
            ],
            9..=17 => [
                WavePart::ConsecutiveWithPause(3, HALF_WIDTH, 4.),
                WavePart::ConsecutiveWithPause(4, HALF_WIDTH, 3.5),

            ],
            _ => [
                WavePart::ConsecutiveWithPause(4, HALF_WIDTH, 3.5),
                WavePart::ConsecutiveWithPause(5, HALF_WIDTH, 3.5),

            ],
        };
        return possible_parts[rng.gen_range(0..possible_parts.len())]
    }
}

#[derive(Resource)]
struct CurrentWave(Vec<WaveEvent>);

impl CurrentWave {
    pub fn new(level: usize) -> Self {
        let mut wave = vec![];

        for _ in 0..space::patterns_nb(level) {
            wave.append(&mut WavePart::random(level).events(level));
            // Always end wave with [WaveEvent::WaitForClear]
            wave.push(WaveEvent::WaitForClear);
        }

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