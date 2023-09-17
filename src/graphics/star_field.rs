use bevy::app::App;
use bevy::prelude::*;
use crate::GameState;

pub struct StarFieldPlugin;

#[derive(Component)]
struct StarFieldUI;

/// Average speed of the star field
#[derive(Resource)]
pub struct StarsSpeed(pub f32);

impl Plugin for StarFieldPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(StarsSpeed(1.0))
            .add_systems(Update, update.run_if(in_state(GameState::Space)))
            .add_systems(OnEnter(GameState::Space), enter)
            .add_systems(OnExit(GameState::Space), exit)
        ;
    }
}

fn update(
    mut commands: Commands,
    speed: Res<StarsSpeed>,
    // TODO: Query stars and update their FakeTransform (~ mut stars: Query<(&Star, &mut FakeTransform)>)
) {
    // TODO: Update stars pos
    // TODO: Spawn stars
}

fn enter(
    mut commands: Commands,
) {
    // TODO: Init star field
}

fn exit(
    mut commands: Commands,
    to_clean: Query<Entity, With<StarFieldUI>>,
) {
    for id in to_clean.iter() {
        commands
            .entity(id)
            .despawn_recursive();
    }
}