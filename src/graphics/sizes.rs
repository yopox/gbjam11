use bevy::math::{Vec2, vec2};

pub enum ShipSize {
    Hero,
}

impl ShipSize {
    /// Real sprite size without transparent pixels on the border. Must be even!
    pub fn hitbox(&self) -> Vec2 {
        match self {
            ShipSize::Hero => vec2(16., 10.),
        }
    }
}