use bevy::prelude::{ClearColor, Color, Commands, DetectChanges, Input, KeyCode, Query, Res, ResMut, Resource};
use crate::graphics::GBShaderSettings;

#[derive(Copy, Clone, PartialEq)]
pub enum Palette {
    Greyscale,
    YellowPurple,
    GameBoy,
}

#[derive(Resource)]
pub struct CurrentPalette(pub Palette);

pub fn update_palette(
    mut commands: Commands,
    mut palette: ResMut<CurrentPalette>,
    mut shader_options: Query<&mut GBShaderSettings>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::P) {
        if palette.0 == Palette::YellowPurple { palette.0 = Palette::GameBoy; }
        else { palette.0 = Palette::YellowPurple; }
    }

    if !palette.is_changed() { return; }

    // Update clear color
    let colors = palette.0.colors();
    commands.insert_resource(ClearColor(colors[0]));

    // Update shader
    let new_options = GBShaderSettings::from_palette(palette.0);
    for mut opt in shader_options.iter_mut() {
        opt.color_0 = new_options.color_0;
        opt.color_1 = new_options.color_1;
        opt.color_2 = new_options.color_2;
        opt.color_3 = new_options.color_3;
    }
}

impl Palette {
    pub fn colors(&self) -> [Color; 4] {
        match self {
            Palette::Greyscale => [
                Color::hex("ffffff").unwrap(),
                Color::hex("aaaaaa").unwrap(),
                Color::hex("666666").unwrap(),
                Color::hex("000000").unwrap(),
            ],
            Palette::YellowPurple => [
                Color::hex("201e33").unwrap(),
                Color::hex("802570").unwrap(),
                Color::hex("ffbc43").unwrap(),
                Color::hex("808a94").unwrap(),
            ],
            Palette::GameBoy => [
                Color::hex("0f380f").unwrap(),
                Color::hex("306230").unwrap(),
                Color::hex("8bac0f").unwrap(),
                Color::hex("9bbc0f").unwrap(),
            ],
        }
    }
}