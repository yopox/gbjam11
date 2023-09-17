use bevy::math::{Vec2, vec2};
use bevy::prelude::Component;

#[derive(Component)]
pub enum Hitbox {
    Hero,
}

impl Hitbox {
    /// Real sprite size without transparent pixels on the border. Must be even!
    pub fn size(&self) -> Vec2 {
        match self {
            Hitbox::Hero => vec2(16., 10.),
        }
    }
}