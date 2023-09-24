use bevy::app::App;
use bevy::math::vec2;
use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::GameState;
use crate::graphics::{ScreenTransition, StarsSpeed, TextStyles};
use crate::logic::{Items, ShipStatus};
use crate::logic::route::{CurrentRoute, Route};
use crate::logic::upgrades::Upgrades;
use crate::music::{PlaySFXEvent, SFX};
use crate::screens::{Fonts, Textures};
use crate::screens::shop::Select;
use crate::util::{HALF_WIDTH, z_pos};

pub struct UpgradePlugin;

#[derive(Component)]
struct UpgradeUI;

impl Plugin for UpgradePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update.run_if(in_state(GameState::Upgrade)))
            .add_systems(OnEnter(GameState::Upgrade), enter)
            .add_systems(OnExit(GameState::Upgrade), exit)
        ;
    }
}

fn update(
    keys: Res<Input<KeyCode>>,
    mut route: ResMut<CurrentRoute>,
    mut status: ResMut<ShipStatus>,
    mut select: ResMut<Select<Upgrades>>,
    mut transition: ResMut<ScreenTransition>,
    mut dot: Query<&mut Transform, With<SelectionDot>>,
    mut sfx: EventWriter<PlaySFXEvent>,
) {
    // Select previous / next option
    if keys.just_pressed(KeyCode::Up) {
        select.selected = (select.items.len() + select.selected - 1) % select.items.len();
    } else if keys.just_pressed(KeyCode::Down) {
        select.selected = (select.selected + 1) % select.items.len();
    }

    let (pos, upgrade) = select.items[select.selected];

    let Ok(mut dot_pos) = dot.get_single_mut() else { return; };
    dot_pos.translation.x = pos.x - 2.;
    dot_pos.translation.y = pos.y - 1.;

    if transition.is_none() && keys.just_pressed(KeyCode::Space) {
        sfx.send(PlaySFXEvent(SFX::Buy));
        status.add(&Items::Upgrade(upgrade));
        route.advance();
        transition.set_if_neq(ScreenTransition::to(route.state()));
    }
}

#[derive(Component)]
struct SelectionDot;

fn enter(
    mut commands: Commands,
    textures: Res<Textures>,
    fonts: Res<Fonts>,
    status: Res<ShipStatus>,
    mut star_field: ResMut<StarsSpeed>,
    route: Res<CurrentRoute>,
) {
    star_field.set_by_level(route.level);

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                anchor: Anchor::BottomLeft,
                ..default()
            },
            texture: textures.upgrade_bg.clone(),
            transform: Transform::from_xyz(0., 0., z_pos::SHOP),
            ..default()
        })
        .insert(UpgradeUI)
    ;

    let upgrades = vec![
        (vec2(28., 101.), Upgrades::new_non_stat_upgrade(&status)),
        (vec2(28., 65.), Upgrades::random_stat_upgrade()),
    ];

    for (pos, u) in upgrades.iter() {
        commands
            .spawn(Text2dBundle {
                text: Text::from_section(u.name(), TextStyles::Basic.style(&fonts)),
                text_anchor: Anchor::BottomLeft,
                transform: Transform::from_xyz(pos.x, pos.y - 4., z_pos::SHOP_TEXT),
                ..default()
            })
            .insert(UpgradeUI)
        ;

        let (l1, l2, l3) = u.description(&status);
        for (text, i) in [(l1, 0), (l2, 1), (l3, 2)] {
            commands
                .spawn(Text2dBundle {
                    text: Text::from_section(text, TextStyles::Basic.style(&fonts)),
                    text_anchor: Anchor::BottomLeft,
                    transform: Transform::from_xyz(36., pos.y - 4. - 10. - 7. * i as f32, z_pos::SHOP_TEXT),
                    ..default()
                })
                .insert(UpgradeUI)
            ;
        }
    }

    commands
        .spawn(SpriteBundle {
            texture: textures.dot.clone(),
            sprite: Sprite {
                anchor: Anchor::BottomRight,
                ..default()
            },
            transform: Transform::from_xyz(0., 0., z_pos::SHOP_TEXT),
            ..default()
        })
        .insert(SelectionDot)
        .insert(UpgradeUI)
    ;

    commands.insert_resource(Select { items: upgrades, selected: 0 });

    commands
        .spawn(Text2dBundle {
            text: Text::from_section(format!("{}-{}", route.act(), (route.level + 1 - (route.act() - 1) * Route::act_len())), TextStyles::Basic.style(&fonts)),
            text_anchor: Anchor::BottomCenter,
            transform: Transform::from_xyz(HALF_WIDTH, 2., z_pos::GUI),
            ..default()
        })
        .insert(UpgradeUI)
    ;
}

fn exit(
    mut commands: Commands,
    to_clean: Query<Entity, With<UpgradeUI>>,
) {
    for id in to_clean.iter() {
        commands
            .entity(id)
            .despawn_recursive();
    }
}