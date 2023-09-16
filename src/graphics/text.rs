use bevy::text::TextStyle;
use crate::graphics::Palette;
use crate::screens::Fonts;

pub enum TextStyles {
    Basic,
}

impl TextStyles {
    pub fn style(&self, fonts: &Fonts) -> TextStyle {
        match self {
            TextStyles::Basic => TextStyle {
                font: fonts.rank.clone(),
                font_size: 16.0,
                color: Palette::Greyscale.colors()[2],
            }
        }
    }
}