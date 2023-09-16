use bevy::prelude::{Component, Query, Transform, Vec3};

/// Manipulate this component instead of [Transform] to ensure that
/// sprites are never drawn on subpixels.
#[derive(Component)]
pub struct FakeTransform {
    pub translation: Vec3,
}

impl FakeTransform {
    pub fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        Self { translation: Vec3::new(x, y, z) }
    }
}

pub fn update_positions(
    mut query: Query<(&FakeTransform, &mut Transform)>,
) {
    for (fake_pos, mut pos) in query.iter_mut() {
        pos.translation.x = fake_pos.translation.x.round();
        pos.translation.y = fake_pos.translation.y.round();
    }
}

pub fn check_position(
    query: Query<&Transform>,
) {
    for pos in &query {
        if pos.translation.x != pos.translation.x.round() {
            panic!("x coordinate must be round!")
        }
        if pos.translation.y != pos.translation.y.round() {
            panic!("y coordinate must be round!")
        }
    }
}