use crate::prelude::*;

mod hand;
pub use hand::MouseHand;

mod start;
pub use start::MouseHoverState;

mod black_mouse_animation;

/// Marks the mouse start entity spawned at the beginning of gameplay.
#[derive(Component, Reflect)]
pub struct MouseStart;

pub(crate) fn plugin(app: &mut App) {
    hand::plugin(app);
    start::plugin(app);
    black_mouse_animation::plugin(app);
}
