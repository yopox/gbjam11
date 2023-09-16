mod palette;
pub mod sizes;
mod pixel;
mod shader;

use bevy::app::App;
use bevy::prelude::*;
pub use pixel::FakeTransform;
pub use shader::GBShaderSettings;
pub use palette::Palette;
use crate::graphics::palette::CurrentPalette;
use crate::graphics::shader::GBShaderPlugin;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        let palette = Palette::YellowPurple;
        app
            .add_plugins(GBShaderPlugin)
            .insert_resource(CurrentPalette(palette))
            .insert_resource(ClearColor(palette.colors()[0]))
            .add_systems(Update, palette::update_palette)
            .add_systems(PostUpdate, pixel::update_positions)
            .add_systems(PostUpdate, pixel::check_position.after(pixel::update_positions))
        ;
    }
}