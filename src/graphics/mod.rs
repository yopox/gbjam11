pub mod palette;
pub mod sizes;
mod pixel;

use bevy::app::App;
use bevy::prelude::*;
use crate::graphics::palette::Palette;
pub use pixel::FakeTransform;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ClearColor(Color::from(Palette::BACKGROUND)))
            .add_systems(PostUpdate, pixel::update_positions)
            .add_systems(PostUpdate, pixel::check_position.after(pixel::update_positions))
        ;
    }
}