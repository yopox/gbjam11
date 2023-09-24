use bevy::prelude::{Component, Query, Transform, Vec2, Vec3};

/// Manipulate this component instead of [Transform] to ensure that
/// sprites are never drawn on subpixels.
#[derive(Component)]
pub struct FakeTransform {
    pub translation: Vec3,
    pub scale: Option<Vec2>
}

impl FakeTransform {
    pub fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        Self { translation: Vec3::new(x, y, z), scale: None }
    }

    pub fn from_xyz_and_scale(x: f32, y: f32, z: f32, scale_x: f32, scale_y: f32) -> Self {
        Self { translation: Vec3::new(x, y, z), scale: Some(Vec2::new(scale_x, scale_y)) }
    }
}

pub fn update_positions(
    mut query: Query<(&FakeTransform, &mut Transform)>,
) {
    for (fake_pos, mut pos) in query.iter_mut() {
        pos.translation.x = fake_pos.translation.x.round();
        pos.translation.y = fake_pos.translation.y.round();
        pos.translation.z = fake_pos.translation.z;

        if fake_pos.scale.is_some() {
            // Assume initial size is round and short (scale = 0.5 ignored even if size = 2)
            let scale = fake_pos.scale.unwrap();
            pos.scale.x = scale.x.ceil();
            pos.scale.y = scale.y.ceil();
        }
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