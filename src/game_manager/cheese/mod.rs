use crate::prelude::*;

mod eat;
mod spawn;

pub use eat::EatCount;

#[derive(Component, Debug, Clone, Reflect, Default)]
#[require(SpriteLayer::Cheese)]
pub struct Cheese;

pub(crate) fn plugin(app: &mut App) {
    spawn::plugin(app);
    eat::plugin(app);
}
