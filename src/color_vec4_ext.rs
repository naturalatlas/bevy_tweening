use bevy::color::{Color, Lcha};
use bevy::math::Vec4;

pub(crate) trait ColorExt {
    fn to_vec(&self) -> Vec4;
}

impl ColorExt for Color {
    fn to_vec(&self) -> Vec4 {
        let lcha = Lcha::from(*self);
        Vec4::new(lcha.lightness, lcha.chroma, lcha.hue, lcha.alpha)
    }
}

pub(crate) trait Vec4Ext {
    fn to_color(&self) -> Color;
}

impl Vec4Ext for Vec4 {
    fn to_color(&self) -> Color {
        Color::lcha(self.x, self.y, self.z, self.w)
    }
}
