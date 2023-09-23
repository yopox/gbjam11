use bevy::app::App;
use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::GameState;
use crate::graphics::{ScreenTransition, TextStyles};
use crate::logic::route::CurrentRoute;
use crate::screens::Fonts;
use crate::util::{HALF_HEIGHT, HALF_WIDTH, z_pos};

pub struct SimpleTextPlugin;

#[derive(Component)]
struct SimpleTextUI;

#[derive(Resource)]
pub struct SimpleText(pub String);

#[derive(Resource)]
struct Wait(f32);

impl Plugin for SimpleTextPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SimpleText(String::new()))
            .add_systems(Update, update.run_if(in_state(GameState::SimpleText)))
            .add_systems(OnEnter(GameState::SimpleText), enter)
            .add_systems(OnExit(GameState::SimpleText), exit)
        ;
    }
}

fn update(
    time: Res<Time>,
    mut wait: ResMut<Wait>,
    mut route: ResMut<CurrentRoute>,
    mut transition: ResMut<ScreenTransition>,
) {
    wait.0 -= time.delta_seconds();
    if wait.0 < 0. && transition.is_none() {
        route.advance();
        transition.set_if_neq(ScreenTransition::to(route.state()));
    }
}

fn enter(
    mut commands: Commands,
    text: Res<SimpleText>,
    fonts: Res<Fonts>,
) {
    commands.insert_resource(Wait(4.));

    commands
        .spawn(Text2dBundle {
            text: Text::from_section(&text.0, TextStyles::Basic.style(&fonts)),
            text_anchor: Anchor::Center,
            transform: Transform::from_xyz(HALF_WIDTH, HALF_HEIGHT, z_pos::GUI),
            ..default()
        })
        .insert(SimpleTextUI)
    ;
}

fn exit(
    mut commands: Commands,
    to_clean: Query<Entity, With<SimpleTextUI>>,
) {
    for id in to_clean.iter() {
        commands
            .entity(id)
            .despawn_recursive();
    }
}