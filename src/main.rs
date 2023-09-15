mod util;
mod loading;
mod title;
mod palette;

use bevy::prelude::*;
use crate::loading::LoadingPlugin;
use crate::palette::Palette;
use crate::title::TitlePlugin;
use crate::util::{HEIGHT, SCALE, WIDTH};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
    Title,
    Main,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::from(Palette::BACKGROUND)))
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
        .add_state::<GameState>()
        .add_systems(Startup, init)
        .add_plugins((LoadingPlugin, TitlePlugin))
        .run();
}

fn init(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform {
            scale: Vec3::new(1. / SCALE, 1. / SCALE, 1.),
            translation: Vec3::new(WIDTH as f32 / 2., HEIGHT as f32 / 2., 100.),
            ..Default::default()
        },
        ..Default::default()
    });
}
