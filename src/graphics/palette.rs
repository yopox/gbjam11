use bevy::prelude::{Color, DetectChanges, Query, ResMut, Resource};
use bevy::utils::HashMap;
use lazy_static::lazy_static;

use crate::graphics::GBShaderSettings;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Palette {
    Greyscale,
    Yopox,
    Nexus2060,
    LaserLab,
    Foliage,
}

#[derive(Resource)]
pub struct CurrentPalette(pub Palette);

pub fn update_palette(
    mut palette: ResMut<CurrentPalette>,
    mut shader_options: Query<&mut GBShaderSettings>,
) {
    if !palette.is_changed() { return; }

    // Update shader
    let new_options = GBShaderSettings::from_palette(palette.0);
    for mut opt in shader_options.iter_mut() {
        opt.color_0 = new_options.color_0;
        opt.color_1 = new_options.color_1;
        opt.color_2 = new_options.color_2;
        opt.color_3 = new_options.color_3;
    }
}

lazy_static! {
    static ref COLORS: HashMap<Palette, [Color; 4]> = HashMap::from([
        (Palette::Greyscale, [
            Color::hex("000000").unwrap(),
            Color::hex("666666").unwrap(),
            Color::hex("aaaaaa").unwrap(),
            Color::hex("ffffff").unwrap(),
        ]),
        (Palette::Yopox, [
            Color::hex("201e33").unwrap(),
            Color::hex("ab1a5d").unwrap(),
            Color::hex("ffbc43").unwrap(),
            Color::hex("808a94").unwrap(),
        ]),
        (Palette::Nexus2060, [
            Color::hex("2a110c").unwrap(),
            Color::hex("f1461b").unwrap(),
            Color::hex("faad1f").unwrap(),
            Color::hex("fdfdfd").unwrap(),
        ]),
        (Palette::LaserLab, [
            Color::hex("271d2c").unwrap(),
            Color::hex("e01f3f").unwrap(),
            Color::hex("cc9477").unwrap(),
            Color::hex("fff8ed").unwrap(),
        ]),
        (Palette::Foliage, [
            Color::hex("2e8344").unwrap(),
            Color::hex("095d42").unwrap(),
            Color::hex("77d38f").unwrap(),
            Color::hex("2f4a36").unwrap(),
        ]),
    ]);
}

impl Palette {
    pub fn colors(&self) -> [Color; 4] {
        COLORS[self]
    }
}