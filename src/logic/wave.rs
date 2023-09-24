use std::collections::BTreeMap;
use std::vec;

use bevy::app::App;
use bevy::math::vec2;
use bevy::prelude::*;
use rand::{Rng, thread_rng};
use rand::rngs::ThreadRng;

use crate::entities::{MainShip, Ship, Ships, ShipWeapons};
use crate::GameState;
use crate::graphics::FakeTransform;
use crate::graphics::sizes::Hitbox;
use crate::logic::Loot;
use crate::logic::movement::{Movement, Moves};
use crate::logic::route::CurrentRoute;
use crate::screens::Textures;
use crate::util::{HALF_HEIGHT, HALF_WIDTH, HEIGHT, in_states, space, z_pos};

pub struct WavePlugin;

impl Plugin for WavePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<WaveCleared>()
            .add_systems(Update, update.run_if(in_states(vec![GameState::Space, GameState::Elite, GameState::Boss])))
            .add_systems(OnEnter(GameState::Space), enter)
            .add_systems(OnEnter(GameState::Elite), enter)
            .add_systems(OnEnter(GameState::Boss), enter)
        ;
    }
}

#[derive(Bundle)]
pub struct ShipBundle {
    sprite: SpriteSheetBundle,
    pos: FakeTransform,
    weapons: ShipWeapons,
    hitbox: Hitbox,
    loot: Loot,
    pub ship: Ship,
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
            loot: Loot { credits: space::credits(model) },
            ship,
        }
    }
}

#[derive(Clone)]
enum WaveEvent {
    Spawn(Ships, Moves),
    WaitMilliseconds(usize),
    WaitForClear,
}

#[derive(Clone)]
enum SpecialEvent {
    Spawn(Ships, Moves),
    /// Spawn enemies continuously (delay / y)
    InfiniteWave(usize, f32),
    /// Spawn enemies continuously with a pause on x_0 (delay / y / pause / x_0)
    InfiniteWaveWithPause(usize, f32, usize, f32),
}

enum WavePart {
    SimpleEnemy,
    ConsecutiveWithPause(u8, f32, usize),
    Parallel(usize, Vec<WavePart>),
    InfiniteWave(usize, f32),
}

impl Default for WavePart {
    fn default() -> Self { WavePart::SimpleEnemy }
}

fn random_y(rng: &mut ThreadRng) -> f32 { HEIGHT as f32 / 5. * 2. + rng.gen_range(0.0..1.0) * HALF_HEIGHT }

impl WavePart {
    fn debug_string(&self) -> String {
        match self {
            WavePart::SimpleEnemy =>
                "Single Enemy".to_string(),
            WavePart::ConsecutiveWithPause(n, x, pause) => format!(
                "{} Enemies with {}ms pause on x={}",
                n, pause, x
            ),
            WavePart::Parallel(pause, parts) => format!(
                "Parallel with {}ms delay: {}",
                pause,
                parts.iter()
                    .map(|p| p.debug_string())
                    .reduce(|p1, p2| format!("{} / {}", p1, p2))
                    .unwrap_or("None".to_string())
            ),
            WavePart::InfiniteWave(pause, y) => format!(
                "Infinite wave with {}ms delay between enemies on y={}",
                pause, y
            ),
        }
    }

    fn events(&self, level: usize, base_y: f32) -> Vec<WaveEvent> {
        let mut events = vec![];
        match self {
            WavePart::SimpleEnemy => {
                events.push(WaveEvent::Spawn(
                    Ships::random_enemy(level),
                    Moves::random_crossing(base_y),
                ));
            }
            WavePart::ConsecutiveWithPause(n, x, pause) => {
                let base_move = Moves::random_crossing(base_y);
                for _ in 0..*n {
                    events.push(WaveEvent::Spawn(
                        Ships::random_enemy(level),
                        Moves::WithPause(*x, *pause as f32 / 1000., 0., Box::new(base_move.clone())),
                    ));
                    events.push(WaveEvent::WaitMilliseconds(*pause + 2000));
                }
            }
            WavePart::Parallel(pause, parts) => {
                let mut rng = thread_rng();
                let mut y_pos: Vec<f32> = vec![-100.];
                let mut parallel = vec![];
                for (i, part) in parts.iter().enumerate() {
                    let mut y = -100.;
                    while y_pos.iter().any(|existing| (y - *existing).abs() < 18.) {
                        y = random_y(&mut rng);
                    }
                    y_pos.push(y);
                    events.push(WaveEvent::WaitMilliseconds(i * *pause));
                    events.append(&mut part.events(level, y));
                    parallel.push(events.clone());
                    events.clear();
                }
                events = merge_waves(parallel.as_slice());
            }
            WavePart::InfiniteWave(delay, y) => {
                events.push(WaveEvent::Spawn(
                    Ships::random_enemy(level),
                    Moves::random_crossing(*y),
                ));
                events.push(WaveEvent::WaitMilliseconds(*delay));
            }
        }
        events
    }

    fn random(level: usize) -> Self {
        let mut rng = thread_rng();
        let mut possible_parts = match level {
            0..=8 => vec![
                WavePart::SimpleEnemy,
                WavePart::ConsecutiveWithPause(2, HALF_WIDTH, 4000),
                WavePart::Parallel(8000, vec![
                    WavePart::SimpleEnemy,
                    WavePart::ConsecutiveWithPause(2, HALF_WIDTH, 4000),
                ]),
                WavePart::Parallel(4000, vec![
                    WavePart::SimpleEnemy,
                    WavePart::SimpleEnemy,
                ]),
                WavePart::Parallel(5000, vec![
                    WavePart::SimpleEnemy,
                    WavePart::SimpleEnemy,
                    WavePart::SimpleEnemy,
                ]),
            ],
            9..=17 => vec![
                WavePart::ConsecutiveWithPause(3, HALF_WIDTH, 4000),
                WavePart::ConsecutiveWithPause(4, HALF_WIDTH, 3500),
                WavePart::Parallel(8000, vec![
                    WavePart::ConsecutiveWithPause(2, HALF_WIDTH, 4000),
                    WavePart::ConsecutiveWithPause(2, HALF_WIDTH, 4000),
                ]),
            ],
            _ => vec![
                WavePart::ConsecutiveWithPause(4, HALF_WIDTH, 3500),
                WavePart::ConsecutiveWithPause(5, HALF_WIDTH, 3500),
            ],
        };
        possible_parts.remove(rng.gen_range(0..possible_parts.len()))
    }
}

#[derive(Resource)]
struct CurrentWave(Vec<WaveEvent>, Vec<SpecialEvent>);

impl CurrentWave {
    pub fn new(state: &GameState, level: usize) -> Self {
        info!("{:?} – Generating events for level {}:", state, level);

        let (wave, special) = match state {
            GameState::Elite => (vec![], Self::gen_elite_wave(level)),
            GameState::Boss => (vec![], Self::gen_boss_wave(level)),
            _ => (Self::gen_space_wave(level), vec![]),
        };

        CurrentWave(wave, special)
    }

    fn gen_space_wave(level: usize) -> Vec<WaveEvent> {
        let mut rng = thread_rng();
        let mut wave = vec![];

        for _ in 0..space::patterns_nb(level) {
            let wave_part = WavePart::random(level);
            info!("- {}", wave_part.debug_string());
            wave.append(&mut wave_part.events(level, random_y(&mut rng)));
            // Always end wave with [WaveEvent::WaitForClear]
            wave.push(WaveEvent::WaitForClear);
        }
        wave
    }

    fn gen_elite_wave(level: usize) -> Vec<SpecialEvent> {
        let mut rng = thread_rng();
        let mut wave = vec![];

        wave.push(SpecialEvent::Spawn(
            Ships::Elite(1),
            Moves::Ellipsis(vec2(HALF_WIDTH, HALF_HEIGHT + 32.), 1.2, 32., 16.))
        );

        wave
    }

    fn gen_boss_wave(level: usize) -> Vec<SpecialEvent> {
        let mut rng = thread_rng();
        let mut wave = vec![];

        wave
    }
}

#[derive(Event)]
pub struct WaveCleared;

fn enter(
    mut commands: Commands,
    route: Res<CurrentRoute>,
    state: Res<State<GameState>>,
) {
    commands.insert_resource(CurrentWave::new(state.get(), route.level));
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
            ;
            next = true;
        }
        Some(WaveEvent::WaitMilliseconds(ref mut s)) => {
            if *s > 0 {
                let dt = (time.delta_seconds() * 1000.) as usize;
                *s = if *s < dt { 0 } else { *s - dt };
            }
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

    next = false;
    match wave.1.get_mut(0) {
        Some(SpecialEvent::Spawn(model, moves)) => {
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
        Some(SpecialEvent::InfiniteWave(_, _)) => {}
        Some(SpecialEvent::InfiniteWaveWithPause(_, _, _, _)) => {}
        None => {}
    }

    if next { wave.1.remove(0); }
}

fn merge_waves(waves: &[Vec<WaveEvent>]) -> Vec<WaveEvent> {
    let mut events = BTreeMap::<usize, Vec<WaveEvent>>::new();
    for wave in waves {
        let mut pause = 0;
        for event in wave.iter() {
            match event {
                WaveEvent::WaitMilliseconds(s) => { pause += *s; }
                WaveEvent::WaitForClear => {}
                _ => {
                    if let Some(e) = events.get_mut(&pause) {
                        e.push(event.clone());
                    } else {
                        events.insert(pause, vec![event.clone()]);
                    }
                }
            }
        }
    }

    let mut merged = vec![];
    let mut last_pause = 0;
    for (p, e) in events.iter() {
        let wait = *p - last_pause;
        last_pause = *p;
        merged.push(WaveEvent::WaitMilliseconds(wait));
        e.iter().for_each(|event| merged.push(event.clone()));
    }
    merged
}