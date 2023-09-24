use bevy::app::App;
use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::GameState;
use crate::graphics::{ScreenTransition, StarsSpeed, TextStyles};
use crate::logic::damage::KillCount;
use crate::logic::route::CurrentRoute;
use crate::logic::ShipStatus;
use crate::music::{PlaySFXEvent, SFX};
use crate::screens::Fonts;
use crate::util::{HALF_HEIGHT, HALF_WIDTH, z_pos};

pub struct GameOverPlugin;

#[derive(Component)]
struct GameOverUI;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update.run_if(in_state(GameState::GameOver)))
            .add_systems(OnEnter(GameState::GameOver), enter)
            .add_systems(OnExit(GameState::GameOver), exit)
        ;
    }
}

fn update(
    keys: Res<Input<KeyCode>>,
    mut transition: ResMut<ScreenTransition>,
    mut sfx: EventWriter<PlaySFXEvent>,
) {
    if keys.just_pressed(KeyCode::Space) && transition.is_none() {
        sfx.send(PlaySFXEvent(SFX::Select));
        transition.set_if_neq(ScreenTransition::to(GameState::Title));
    }
}

enum ScreenItem {
    Text(String),
    Space(f32),
}

fn enter(
    mut commands: Commands,
    mut ship_status: ResMut<ShipStatus>,
    mut stars_speed: ResMut<StarsSpeed>,
    route: Res<CurrentRoute>,
    fonts: Res<Fonts>,
) {
    stars_speed.set_by_level(0);

    commands.remove_resource::<KillCount>();

    let mut texts = vec!(
        ScreenItem::Text(if route.win() { "Congratulations!" } else { "Game Over :(" }.to_string()),
        ScreenItem::Space(8.),
        ScreenItem::Text(format!("Hull: {:.0}/{:.0}", ship_status.health().0, ship_status.health().1)),
        ScreenItem::Text(format!("Speed: x{:.2}", ship_status.speed_multiplier())),
        ScreenItem::Text(format!("Damage: x{:.2}", ship_status.damage_multiplier())),
        ScreenItem::Text(format!("Shot speed: x{:.2}", ship_status.shot_speed_multiplier())),
        ScreenItem::Text(format!("Shot frequency: x{:.2}", ship_status.shot_frequency_multiplier())),
        ScreenItem::Space(8.),
    );

    ship_status.non_stat_upgrades().iter().for_each(|u| texts.push(ScreenItem::Text(u.name().to_string())));

    texts.push(ScreenItem::Space(8.));
    texts.push(ScreenItem::Text("Press A to return".to_string()));
    texts.push(ScreenItem::Text("to the hangar.".to_string()));

    let total_y = texts.iter().map(|e| match e {
        ScreenItem::Text(_) => 7.,
        ScreenItem::Space(y) => *y,
    }).sum::<f32>();

    let mut text_y = HALF_HEIGHT + (total_y / 2.).floor() - 9.;

    for item in texts.iter() {
        match item {
            ScreenItem::Text(t) => {
                commands
                    .spawn(Text2dBundle {
                        text: Text::from_section(t, TextStyles::Basic.style(&fonts))
                            .with_alignment(TextAlignment::Center)
                        ,
                        text_anchor: Anchor::BottomCenter,
                        transform: Transform::from_xyz(HALF_WIDTH, text_y, z_pos::GUI),
                        ..default()
                    })
                    .insert(GameOverUI)
                ;
                text_y -= 7.;
            }
            ScreenItem::Space(y) => {
                text_y -= *y;
            }
        }
    }
}

fn exit(
    mut commands: Commands,
    to_clean: Query<Entity, With<GameOverUI>>,
) {
    for id in to_clean.iter() {
        commands
            .entity(id)
            .despawn_recursive();
    }
}