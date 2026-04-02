use extol_sprite_layer::LayerIndex;

use crate::prelude::*;

#[derive(Debug, Clone, Component, Hash, PartialEq, Eq, Reflect, Default, FromTemplate)]
pub enum SpriteLayer {
    #[default]
    Background,
    MouseStart,
    Cat,
    Item,
    Cheese,
    Tutorial,
    RightHand,
    LeftHand,
    MouseHand,
    LoadingBar,
    VFX,
}

impl LayerIndex for SpriteLayer {
    fn as_z_coordinate(&self) -> f32 {
        match *self {
            Self::Background => -100.0,
            Self::MouseStart => -99.0,
            Self::Cat => 2.0,
            Self::Item => 30.0,
            Self::Cheese => 31.0,
            Self::Tutorial => 33.0,
            Self::RightHand | Self::LeftHand => 40.0,
            Self::MouseHand => 41.0,
            Self::LoadingBar => 45.0,
            Self::VFX => 1000.0,
        }
    }
}
