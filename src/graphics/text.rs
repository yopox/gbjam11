use bevy::text::TextStyle;

use crate::graphics::Palette;
use crate::screens::Fonts;

pub enum TextStyles {
    Basic,
    Gray,
    Accent,
}

impl TextStyles {
    pub fn style(&self, fonts: &Fonts) -> TextStyle {
        match self {
            TextStyles::Basic => TextStyle {
                font: fonts.rank.clone(),
                font_size: 16.0,
                color: Palette::Greyscale.colors()[2],
            },
            TextStyles::Gray => TextStyle {
                font: fonts.rank.clone(),
                font_size: 16.0,
                color: Palette::Greyscale.colors()[3],
            },
            TextStyles::Accent => TextStyle {
                font: fonts.rank.clone(),
                font_size: 16.0,
                color: Palette::Greyscale.colors()[1],
            },
        }
    }
}