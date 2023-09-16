mod util;

mod entities;
mod graphics;
mod logic;
mod screens;

use bevy::prelude::*;
use crate::entities::EntitiesPlugin;
use crate::graphics::GraphicsPlugin;
use crate::screens::ScreensPlugin;
use crate::util::{HEIGHT, SCALE, WIDTH};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
    Title,
    Space,
}

fn main() {
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
                    title: "gbjam11".to_string(),
                    canvas: Some("#bevy".to_owned()),
                    ..default()
                }),
                ..default()
            })
        )
        .add_plugins((EntitiesPlugin, GraphicsPlugin, ScreensPlugin))
        .add_state::<GameState>()
        .add_systems(Startup, init)
        .run();
}

fn init(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform {
            scale: Vec3::new(1. / SCALE, 1. / SCALE, 1.),
            translation: Vec3::new(WIDTH as f32 / 2., HEIGHT as f32 / 2., 100.),
            ..default()
        },
        ..default()
    });
}
