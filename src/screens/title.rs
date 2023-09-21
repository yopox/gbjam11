use bevy::app::App;
use bevy::math::vec2;
use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::entities::Ships;
use crate::GameState;
use crate::graphics::{ScreenTransition, TextStyles,};
use crate::logic::ShipBundle;
use crate::screens::{Fonts, Textures};
use crate::util::{WIDTH, z_pos};

pub struct TitlePlugin;

#[derive(Component)]
struct TitleUI;

impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update.run_if(in_state(GameState::Title)))
            .add_systems(OnEnter(GameState::Title), enter)
            .add_systems(OnExit(GameState::Title), exit)
        ;
    }
}

fn update(
    keys: Res<Input<KeyCode>>,
    mut commands: Commands,
) {
    if keys.just_pressed(KeyCode::Space) {
        commands.insert_resource(ScreenTransition::to(GameState::Space))
    }
}

fn enter(
    mut commands: Commands,
    textures: Res<Textures>,
    fonts: Res<Fonts>,
) {
    commands
        .spawn(Text2dBundle {
            text: Text::from_section("Press start", TextStyles::Basic.style(&fonts)),
            text_anchor: Anchor::Center,
            transform: Transform::from_xyz(WIDTH as f32 / 2., 44., z_pos::GUI),
            ..default()
        })
        .insert(TitleUI)
    ;

    commands
        .spawn(ShipBundle::from(textures.ship.clone(), Ships::Player, vec2(16., 16.)))
        .insert(TitleUI);
}

fn exit(
    mut commands: Commands,
    to_clean: Query<Entity, With<TitleUI>>,
) {
    for id in to_clean.iter() {
        commands
            .entity(id)
            .despawn_recursive();
    }
}