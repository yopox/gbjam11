use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::GameState;
use crate::music::{BGM, PlayBGMEvent};

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_loading_state(
                LoadingState::new(GameState::Loading)
                    .continue_to_state(GameState::Title),
            )
            .add_collection_to_loading_state::<_, Textures>(GameState::Loading)
            .add_collection_to_loading_state::<_, Fonts>(GameState::Loading)
            .add_collection_to_loading_state::<_, Sounds>(GameState::Loading)
            .add_systems(OnExit(GameState::Loading), exit)
        ;
    }
}

fn exit(mut play_bgm: EventWriter<PlayBGMEvent>) { play_bgm.send(PlayBGMEvent(BGM::Title)); }

#[derive(AssetCollection, Resource)]
pub struct Textures {
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 18, rows = 1, padding_x = 0., padding_y = 0.))]
    #[asset(path = "ships.png")]
    pub ship: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 8., tile_size_y = 8., columns = 5, rows = 1, padding_x = 0., padding_y = 0.))]
    #[asset(path = "shots.png")]
    pub shots: Handle<TextureAtlas>,

    #[asset(path = "bar.png")]
    pub bar: Handle<Image>,

    #[asset(path = "hangar.png")]
    pub hangar: Handle<Image>,

    #[asset(path = "legend.png")]
    pub legend: Handle<Image>,

    #[asset(path = "option_bars.png")]
    pub option_bars: Handle<Image>,

    #[asset(path = "shop_bg.png")]
    pub shop_bg: Handle<Image>,

    #[asset(path = "dot.png")]
    pub dot: Handle<Image>,

    #[asset(path = "upgrade_bg.png")]
    pub upgrade_bg: Handle<Image>,

    #[asset(path = "shield.png")]
    pub shield: Handle<Image>,

    #[asset(path = "logo.png")]
    pub logo: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct Fonts {
    #[asset(path = "Rank 6h.ttf")]
    pub rank: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct Sounds {
    #[asset(path = "bgm/1_Title screen.ogg")]
    pub title: Handle<AudioSource>,

    #[asset(path = "bgm/2_Ship selection.ogg")]
    pub hangar: Handle<AudioSource>,

    #[asset(path = "bgm/3_Main game.ogg")]
    pub space: Handle<AudioSource>,

    #[asset(path = "bgm/4_Elite.ogg")]
    pub elite: Handle<AudioSource>,

    #[asset(path = "bgm/5_Boss.ogg")]
    pub boss: Handle<AudioSource>,

    #[asset(path = "bgm/6_Shop.ogg")]
    pub shop: Handle<AudioSource>,

    #[asset(path = "sfx/OS_Repairing.ogg")]
    pub repair: Handle<AudioSource>,

    #[asset(path = "sfx/OS_Buy_upgrade.ogg")]
    pub buy: Handle<AudioSource>,

    #[asset(path = "sfx/OS_Boom.ogg")]
    pub missile: Handle<AudioSource>,

    #[asset(path = "sfx/OS_Shield.ogg")]
    pub shield: Handle<AudioSource>,

    #[asset(path = "sfx/OS_Die.ogg")]
    pub game_over: Handle<AudioSource>,

    #[asset(path = "sfx/OS_EnemyFiring.ogg")]
    pub enemy_fire: Handle<AudioSource>,

    #[asset(path = "sfx/OS_Hit.ogg")]
    pub enemy_hit: Handle<AudioSource>,

    #[asset(path = "sfx/OS_Shoot.ogg")]
    pub ship_fire: Handle<AudioSource>,

    #[asset(path = "sfx/OS_Touch.ogg")]
    pub ship_hit: Handle<AudioSource>,

    #[asset(path = "sfx/OS_Select.ogg")]
    pub select: Handle<AudioSource>,

    #[asset(path = "sfx/OS_Cancel.ogg")]
    pub cancel: Handle<AudioSource>,

    #[asset(path = "sfx/OS_Dash.ogg")]
    pub dash: Handle<AudioSource>,

    #[asset(path = "sfx/OS_Left.ogg")]
    pub left: Handle<AudioSource>,

    #[asset(path = "sfx/OS_Right.ogg")]
    pub right: Handle<AudioSource>,

    #[asset(path = "sfx/OS_Sell.ogg")]
    pub sell: Handle<AudioSource>,

    #[asset(path = "sfx/OS_Error.ogg")]
    pub error: Handle<AudioSource>,

    #[asset(path = "sfx/OS_Leech.ogg")]
    pub leech: Handle<AudioSource>,
}