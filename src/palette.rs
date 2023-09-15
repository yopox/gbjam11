use bevy::prelude::Color;

#[derive(Copy, Clone)]
pub enum Palette {
    BACKGROUND,
    MUTED,
    SHIP,
    ENEMY,
}

impl From<Palette> for Color {
    fn from(value: Palette) -> Self {
        match value {
            Palette::BACKGROUND => Color::hex("201e33"),
            Palette::MUTED => Color::hex("808a94"),
            Palette::SHIP => Color::hex("ffbc43"),
            Palette::ENEMY => Color::hex("802570"),
        }.expect("Couldn't parse color")
    }
}