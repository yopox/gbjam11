use bevy::app::App;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use crate::entities::Ship;
use crate::GameState;
use crate::graphics::FakeTransform;
use crate::graphics::sizes::ShipSize;
use crate::screens::Textures;
use crate::util::{BORDER, WIDTH};

pub struct SpacePlugin;

#[derive(Component)]
struct SpaceUI;

impl Plugin for SpacePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update.run_if(in_state(GameState::Space)))
            .add_systems(OnEnter(GameState::Space), enter)
            .add_systems(OnExit(GameState::Space), exit)
        ;
    }
}

fn update(
    keys: Res<Input<KeyCode>>,
    mut ship: Query<(&Ship, &mut FakeTransform)>,
) {
    for (s, mut pos) in ship.iter_mut() {
        if !s.friendly { continue }

        let dx = s.speed + s.size.hitbox().x / 2. + BORDER;
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
) {
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: textures.ship.clone(),
            ..default()
        })
        .insert(FakeTransform::from_xyz(WIDTH as f32 / 2., 24., 1.))
        .insert(Ship::new(true, ShipSize::Hero, 0.5))
    ;
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