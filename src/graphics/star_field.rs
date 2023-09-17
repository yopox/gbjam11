use bevy::app::App;
use bevy::prelude::*;
use rand::{random, Rng};
use crate::GameState;
use crate::graphics::{FakeTransform, Palette};
use crate::util::{HEIGHT, WIDTH, z_pos};

pub struct StarFieldPlugin;

#[derive(Component)]
struct StarFieldUI;

#[derive(Component)]
struct Star {
    pub speed_modifier: f32
}

/// Average speed of the star field
#[derive(Resource)]
pub struct StarsSpeed(pub Vec2);

impl Plugin for StarFieldPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(StarsSpeed(Vec2 { x: 0.0, y: -0.5 }))
            .add_systems(Startup, enter)
            .add_systems(Update, update.run_if(in_state(GameState::Space)))
            .add_systems(OnEnter(GameState::Space), enter)
            .add_systems(OnExit(GameState::Space), exit)
        ;
    }
}

fn enter(
    mut commands: Commands
) {
    let mut rng = rand::thread_rng();

    for _i in 0..50 {
        let sprite = Sprite {
            color: Palette::Greyscale.colors()[3],
            custom_size: Some(Vec2::new(1., 1.)),
            ..default()
        };

        commands
            .spawn(SpriteBundle {
                sprite,
                ..default()
            })
            .insert(FakeTransform::from_xyz(
                rng.gen::<f32>() * WIDTH as f32,
                rng.gen::<f32>() * HEIGHT as f32,
                z_pos::STAR_FIELD,
            ))
            .insert(Star {
                // Between 0.5 and 1.5 regular speed of the field
                speed_modifier: rng.gen::<f32>() + 0.5
            })
            .insert(StarFieldUI);
    }
}

fn update(
    speed: Res<StarsSpeed>,
    mut stars: Query<(&Star, &mut FakeTransform)>,
) {
    for (star, mut transform) in stars.iter_mut() {
        transform.translation.y = transform.translation.y + speed.0.y * star.speed_modifier;

        // Note: one handle a single direction
        if transform.translation.y < 0.0 {
            transform.translation.y += HEIGHT as f32;
            transform.translation.x = random::<f32>() * WIDTH as f32;
        } else {
            transform.translation.x = (transform.translation.x + speed.0.x * star.speed_modifier).rem_euclid(WIDTH as f32);
        }

    }
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