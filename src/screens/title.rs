use bevy::app::App;
use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::GameState;
use crate::graphics::{ScreenTransition, StarsSpeed, TextStyles};
use crate::screens::{Fonts, Textures};
use crate::util::{HALF_WIDTH, star_field, z_pos};

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
    mut transition: ResMut<ScreenTransition>,
) {
    if !transition.is_none() { return; }

    if keys.just_pressed(KeyCode::Space) {
        transition.set_if_neq(ScreenTransition::to(GameState::Hangar))
    }
}

fn enter(
    mut commands: Commands,
    textures: Res<Textures>,
    mut star_speed: ResMut<StarsSpeed>,
    fonts: Res<Fonts>,
) {
    star_speed.0 = star_field::INITIAL_SPEED;
    commands
        .spawn(Text2dBundle {
            text: Text::from_section("Press start", TextStyles::Basic.style(&fonts)),
            text_anchor: Anchor::Center,
            transform: Transform::from_xyz(HALF_WIDTH, 44., z_pos::GUI),
            ..default()
        })
        .insert(TitleUI)
    ;
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