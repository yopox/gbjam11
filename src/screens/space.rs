use bevy::app::App;
use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::entities::{MainShip, Ship, Ships, Shot};
use crate::GameState;
use crate::graphics::{FakeTransform, TextStyles};
use crate::graphics::sizes::Hitbox;
use crate::logic::damage::DamageEvent;
use crate::logic::ShipBundle;
use crate::screens::{Fonts, Textures};
use crate::util::{BORDER, WIDTH, z_pos};
use crate::util::hud::HEALTH_BAR_SIZE;

pub struct SpacePlugin;

#[derive(Component)]
struct SpaceUI;

#[derive(Resource)]
pub struct Credits(pub u16);

#[derive(Component)]
struct LifeBar;

#[derive(Component)]
struct CreditsText;

impl Plugin for SpacePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Credits(0))
            .add_systems(Update, (update, update_gui, update_life)
                .run_if(in_state(GameState::Space)),
            )
            .add_systems(OnEnter(GameState::Space), enter)
            .add_systems(OnExit(GameState::Space), exit)
        ;
    }
}

fn update(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut ship: Query<(&Ship, &Hitbox, &mut FakeTransform)>,
) {
    for (s, hitbox, mut pos) in ship.iter_mut() {
        if !s.friendly { continue; }

        let hitbox_w = hitbox.0.x;
        let movement_x = s.speed * time.delta_seconds();
        let dx = movement_x + hitbox_w / 2. + BORDER;
        if keys.pressed(KeyCode::Left) {
            if pos.translation.x - dx >= 0. { pos.translation.x -= movement_x; }
        }
        if keys.pressed(KeyCode::Right) {
            if pos.translation.x + dx <= WIDTH as f32 { pos.translation.x += movement_x; }
        }
    }
}

fn enter(
    mut commands: Commands,
    textures: Res<Textures>,
    fonts: Res<Fonts>,
) {
    commands
        .spawn(ShipBundle::from(
            textures.ship.clone(),
            Ships::Player,
            vec2(WIDTH as f32 / 2., 24.),
        ))
        .insert(MainShip)
        .insert(SpaceUI)
    ;

    // GUI
    commands
        .spawn(Text2dBundle {
            text: Text::from_section("Life", TextStyles::Basic.style(&fonts)),
            text_anchor: Anchor::BottomLeft,
            transform: Transform::from_xyz(8., 4., z_pos::GUI),
            ..default()
        })
        .insert(SpaceUI)
    ;

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                anchor: Anchor::BottomLeft,
                ..default()
            },
            texture: textures.bar.clone(),
            ..default()
        })
        .insert(LifeBar)
        .insert(FakeTransform::from_xyz_and_scale(
            8., 4., z_pos::GUI,
            HEALTH_BAR_SIZE as f32, 1.,
        ))
        .insert(SpaceUI)
    ;

    commands
        .spawn(Text2dBundle {
            text: Text::from_section("Credits: 999", TextStyles::Basic.style(&fonts)),
            text_anchor: Anchor::BottomRight,
            transform: Transform::from_xyz(WIDTH as f32 - 7., 4., z_pos::GUI),
            ..default()
        })
        .insert(CreditsText)
        .insert(SpaceUI)
    ;

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                anchor: Anchor::BottomRight,
                ..default()
            },
            texture: textures.bar.clone(),
            transform: Transform {
                translation: vec3(WIDTH as f32 - 8., 4., z_pos::GUI),
                scale: vec3(55., 1., 1.),
                ..default()
            },
            ..default()
        })
        .insert(SpaceUI)
    ;
}

fn update_gui(
    credits: Res<Credits>,
    mut text: Query<&mut Text, With<CreditsText>>,
) {
    if credits.is_changed() {
        text.single_mut().sections[0].value = format!("Credits: {:03}", credits.0);
    }
}

fn update_life(
    ships: Query<&Ship, With<MainShip>>,
    mut bar_transform: Query<&mut FakeTransform, With<LifeBar>>,
    mut damaged: EventReader<DamageEvent>,
) {
    for &DamageEvent { ship, fatal } in damaged.iter() {
        if let Ok(ship) = ships.get(ship) {
            bar_transform.single_mut().scale = Some(Vec2::new(
                ship.health as f32 / ship.max_health as f32 * HEALTH_BAR_SIZE as f32,
                1.,
            ))
        }
    }
}

fn exit(
    mut commands: Commands,
    to_clean: Query<Entity, Or<(With<SpaceUI>, With<Ship>, With<Shot>)>>,
) {
    for id in to_clean.iter() {
        commands
            .entity(id)
            .despawn_recursive();
    }
}