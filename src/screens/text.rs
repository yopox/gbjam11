use bevy::app::App;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use rand::{RngCore, thread_rng};

use crate::GameState;
use crate::graphics::{ScreenTransition, TextStyles};
use crate::logic::{Items, ShipStatus};
use crate::logic::route::CurrentRoute;
use crate::screens::Fonts;
use crate::util::{HALF_HEIGHT, HALF_WIDTH, z_pos};

pub struct SimpleTextPlugin;

#[derive(Component)]
struct SimpleTextUI;

#[derive(Resource)]
pub struct SimpleText(pub String);

impl Plugin for SimpleTextPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SimpleText(String::new()))
            .add_systems(Update, update.run_if(in_state(GameState::SimpleText)))
            .add_systems(OnEnter(GameState::Repair), on_enter_repair)
            .add_systems(OnEnter(GameState::SimpleText), enter)
            .add_systems(OnExit(GameState::SimpleText), exit)
        ;
    }
}

fn on_enter_repair(
    mut text: ResMut<SimpleText>,
    mut ship_status: ResMut<ShipStatus>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let justine = match thread_rng().next_u32() % 10 {
        0 => {
            ship_status.add(&Items::Missile);
            "\n1 missile found!"
        }
        1 => {
            ship_status.add(&Items::Shield);
            "\n1 shield found!"
        }
        _ => ""
    };
    for _ in 0..4 { ship_status.add(&Items::Repair); }
    text.0 = format!("Hull repaired. ({:.0}/{:.0}){}", ship_status.health().0, ship_status.health().1, justine);
    next_state.set(GameState::SimpleText);
}

#[derive(Resource)]
struct Wait(f32);

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
            text: Text::from_section(&text.0, TextStyles::Basic.style(&fonts))
                .with_alignment(TextAlignment::Center)
            ,
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