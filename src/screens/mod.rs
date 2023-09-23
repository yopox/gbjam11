use bevy::app::App;
use bevy::prelude::*;

pub use hangar::SelectedShip;
pub use loading::Fonts;
pub use loading::Textures;
pub use space::Credits;

use crate::screens::hangar::HangarPlugin;
use crate::screens::loading::LoadingPlugin;
use crate::screens::shop::ShopPlugin;
use crate::screens::space::SpacePlugin;
use crate::screens::title::TitlePlugin;
use crate::screens::upgrade::UpgradePlugin;

mod loading;
mod space;
mod title;
mod hangar;
mod shop;
mod upgrade;

pub struct ScreensPlugin;

impl Plugin for ScreensPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                LoadingPlugin,
                TitlePlugin,
                HangarPlugin,
                SpacePlugin,
                ShopPlugin,
                UpgradePlugin,
            ))
        ;
    }
}