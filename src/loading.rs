use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::GameState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_loading_state(
                LoadingState::new(GameState::Loading)
                    .continue_to_state(GameState::Title),
            )
            .add_collection_to_loading_state::<_, Textures>(GameState::Loading);
    }
}

#[derive(AssetCollection, Resource)]
pub struct Textures {
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 1, rows = 1, padding_x = 0., padding_y = 0.))]
    #[asset(path = "ship.png")]
    pub ship: Handle<TextureAtlas>,
}