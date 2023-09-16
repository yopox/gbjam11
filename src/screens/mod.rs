mod loading;
mod space;
mod title;

use bevy::app::App;
use bevy::prelude::*;
use crate::screens::loading::LoadingPlugin;
use crate::screens::space::SpacePlugin;
use crate::screens::title::TitlePlugin;
pub use loading::Textures;
pub use loading::Fonts;

pub struct ScreensPlugin;

impl Plugin for ScreensPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((LoadingPlugin, TitlePlugin, SpacePlugin))
        ;
    }
}