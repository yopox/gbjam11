use bevy::app::App;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use crate::GameState;
use crate::loading::Textures;

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

) {

}

fn enter(
    mut commands: Commands,
    textures: Res<Textures>
) {
    commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                anchor: Anchor::BottomLeft,
                ..default()
            },
            texture_atlas: textures.ship.clone(),
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
        })
    ;
    bevy::log::info!("spawning ship");
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