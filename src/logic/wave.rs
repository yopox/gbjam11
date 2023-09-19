use bevy::app::App;
use bevy::prelude::*;

use crate::entities::{Ship, Ships, ShipWeapons};
use crate::GameState;
use crate::graphics::FakeTransform;
use crate::graphics::sizes::Hitbox;
use crate::util::z_pos;

pub struct WavePlugin;

#[derive(Component)]
struct WaveUI;

impl Plugin for WavePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update.run_if(in_state(GameState::Space)))
            .add_systems(OnEnter(GameState::Space), enter)
            .add_systems(OnExit(GameState::Space), exit)
        ;
    }
}

#[derive(Bundle)]
pub struct ShipBundle {
    sprite: SpriteSheetBundle,
    pos: FakeTransform,
    weapons: ShipWeapons,
    hitbox: Hitbox,
    ship: Ship,
}

impl ShipBundle {
    pub fn from(atlas: Handle<TextureAtlas>, model: Ships, pos: Vec2) -> ShipBundle {
        let ship = Ship::from(model);
        Self {
            sprite: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: ship.sprite_index(),
                    ..default()
                },
                texture_atlas: atlas,
                ..default()
            },
            pos: FakeTransform::from_xyz(pos.x, pos.y, z_pos::SHIPS),
            weapons: ShipWeapons::new(&ship, model.weapons()),
            hitbox: model.hitbox(),
            ship,
        }
    }
}

fn update(

) {

}

fn enter(

) {

}

fn exit(
    mut commands: Commands,
    to_clean: Query<Entity, With<WaveUI>>,
) {
    for id in to_clean.iter() {
        commands
            .entity(id)
            .despawn_recursive();
    }
}