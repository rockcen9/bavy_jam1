use crate::prelude::*;
mod mouse;
pub use mouse::*;
mod hand_drawing;
pub(crate) use hand_drawing::{LeftHand, RightHand};
mod hand_screen;
pub use hand_screen::{RightHandScreenPosition, ScreenHalf};
mod head;
pub use head::PlayerHeadPosition;
#[cfg(not(feature = "backend"))]
mod mock;

mod noise;

pub(crate) fn plugin(app: &mut App) {
    mouse::plugin(app);
    hand_drawing::plugin(app);
    hand_screen::plugin(app);
    head::plugin(app);
    noise::plugin(app);
    #[cfg(not(feature = "backend"))]
    mock::plugin(app);
    #[cfg(feature = "backend")]
    app.add_systems(OnEnter(GameState::Tutorial), on_enter_idle_enable_yolo)
        .add_systems(OnEnter(GameState::Prepare), on_enter_idle_enable_yolo);
}

#[cfg(feature = "backend")]
fn on_enter_idle_enable_yolo(mut config: ResMut<yolo::YoloConfig>) {
    config.run_inference = true;
}
