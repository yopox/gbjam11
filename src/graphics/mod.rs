mod palette;
pub mod sizes;
mod pixel;
mod shader;
mod text;

use bevy::app::App;
use bevy::prelude::*;
pub use palette::Palette;
pub use palette::CurrentPalette;
pub use pixel::FakeTransform;
use shader::GBShaderPlugin;
pub use shader::GBShaderSettings;
pub use text::TextStyles;

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