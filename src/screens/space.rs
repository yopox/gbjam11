use bevy::app::App;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use crate::entities::Ship;
use crate::GameState;
use crate::graphics::FakeTransform;
use crate::screens::Textures;

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

        if keys.pressed(KeyCode::Left) { pos.translation.x -= 0.25; }
        if keys.pressed(KeyCode::Right) { pos.translation.x += 0.25; }
    }
}

fn enter(
    mut commands: Commands,
    textures: Res<Textures>,
) {
    commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                anchor: Anchor::BottomLeft,
                ..default()
            },
            texture_atlas: textures.ship.clone(),
            ..default()
        })
        .insert(FakeTransform::from_xyz(0., 0., 1.))
        .insert(Ship::new(true))
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