use crate::prelude::*;

mod animation;
mod overlap;
mod spawn;

#[derive(Component, Debug, Clone, Reflect, Default)]
pub struct EscapePoint;

pub(crate) fn plugin(app: &mut App) {
    spawn::plugin(app);
    overlap::plugin(app);
    animation::plugin(app);
}
