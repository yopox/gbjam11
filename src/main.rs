use bevy::prelude::*;

use crate::entities::EntitiesPlugin;
use crate::graphics::{GBShaderSettings, GraphicsPlugin};
use crate::graphics::Palette;
use crate::logic::LogicPlugin;
use crate::music::{AudioPlugin, BGM};
use crate::screens::ScreensPlugin;
use crate::util::{HALF_HEIGHT, HALF_WIDTH, HEIGHT, SCALE, WIDTH};

mod util;

mod entities;
mod graphics;
mod logic;
mod screens;
mod music;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
    Title,
    Hangar,
    Space, Elite, Boss,
    Shop,
    Upgrade,
    Repair,
    SimpleText,
    GameOver,
    /// Dummy state to fix Space -> Space transition
    Dummy,
}

impl GameState {
    pub fn bgm(&self) -> Option<BGM> {
        match self {
            GameState::Title => Some(BGM::Title),
            GameState::Hangar
            | GameState::Upgrade => Some(BGM::Hangar),
            GameState::Space => Some(BGM::Space),
            GameState::Elite => Some(BGM::Elite),
            GameState::Boss => Some(BGM::Boss),
            GameState::Shop
            | GameState::GameOver => Some(BGM::Shop),
            GameState::Repair => Some(BGM::Repair),
            _ => None,
        }
    }
}

fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    App::new()
        .insert_resource(Msaa::Off)
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (
                        WIDTH as f32 * SCALE,
                        HEIGHT as f32 * SCALE,
                    ).into(),
                    title: "space station No. 34".to_string(),
                    canvas: Some("#bevy".to_owned()),
                    ..default()
                }),
                ..default()
            })
        )
        .add_plugins((EntitiesPlugin, GraphicsPlugin, LogicPlugin, ScreensPlugin, AudioPlugin))
        .add_state::<GameState>()
        .add_systems(Startup, init)
        .run();
}

fn init(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle {
            transform: Transform {
                scale: Vec3::new(1. / SCALE, 1. / SCALE, 1.),
                translation: Vec3::new(HALF_WIDTH, HALF_HEIGHT, 100.),
                ..default()
            },
            ..default()
        })
        .insert(GBShaderSettings::from_palette(Palette::Yopox))
    ;
}