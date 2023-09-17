use bevy::app::App;
use bevy::math::vec3;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use crate::entities::{Ship, Ships};
use crate::GameState;
use crate::graphics::{FakeTransform, TextStyles};
use crate::graphics::sizes::Hitbox;
use crate::screens::{Fonts, Textures};
use crate::util::{BORDER, WIDTH};

pub struct SpacePlugin;

#[derive(Component)]
struct SpaceUI;

#[derive(Resource)]
pub struct Credits(pub u16);

#[derive(Component)]
struct CreditsText;

impl Plugin for SpacePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Credits(0))
            .add_systems(Update, (update, update_gui)
                .run_if(in_state(GameState::Space))
            )
            .add_systems(OnEnter(GameState::Space), enter)
            .add_systems(OnExit(GameState::Space), exit)
        ;
    }
}

fn update(
    keys: Res<Input<KeyCode>>,
    mut ship: Query<(&Ship, &Hitbox, &mut FakeTransform)>,
) {
    for (s, hitbox, mut pos) in ship.iter_mut() {
        if !s.friendly { continue }

        let hitbox_w = hitbox.size().x;
        let dx = s.speed + hitbox_w / 2. + BORDER;
        if keys.pressed(KeyCode::Left) {
            if pos.translation.x - dx >= 0. { pos.translation.x -= s.speed; }
        }
        if keys.pressed(KeyCode::Right) {
            if pos.translation.x + dx <= WIDTH as f32 { pos.translation.x += s.speed; }
        }
    }
}

fn enter(
    mut commands: Commands,
    textures: Res<Textures>,
    fonts: Res<Fonts>,
) {
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: textures.ship.clone(),
            ..default()
        })
        .insert(FakeTransform::from_xyz(WIDTH as f32 / 2., 24., 1.))
        .insert(Ship::from(Ships::Player))
        .insert(Hitbox::Hero)
        .insert(SpaceUI)
    ;

    // GUI
    commands
        .spawn(Text2dBundle {
            text: Text::from_section("Life", TextStyles::Basic.style(&fonts)),
            text_anchor: Anchor::BottomLeft,
            transform: Transform::from_xyz(8., 4., 1.),
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
            transform: Transform {
                translation: vec3(8., 4., 1.),
                scale: vec3(32., 1., 1.),
                ..default()
            },
            ..default()
        })
        .insert(SpaceUI)
    ;

    commands
        .spawn(Text2dBundle {
            text: Text::from_section("Credits: 999", TextStyles::Basic.style(&fonts)),
            text_anchor: Anchor::BottomRight,
            transform: Transform::from_xyz(WIDTH as f32 - 7., 4., 1.),
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
                translation: vec3(WIDTH as f32 - 8., 4., 1.),
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

fn exit(
    mut commands: Commands,
    to_clean: Query<Entity, With<SpaceUI>>,
) {
    for id in to_clean.iter() {
        commands
            .entity(id)
            .despawn_recursive();
    }
}