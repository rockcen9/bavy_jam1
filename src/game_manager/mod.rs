mod player;
use crate::prelude::*;
pub(crate) use player::*;

mod bgm;

mod background;

mod sprite_layer;
pub use sprite_layer::*;

// mod actor;
// pub use actor::*;

mod end;

mod failed;

mod tutorials;

mod cat;
pub(crate) use cat::SleepResource;

mod cheese;
pub(crate) use cheese::*;

mod escape;
pub(crate) use escape::*;

pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.add_sub_state::<GameState>();
    app.init_state::<Pause>();
    app.configure_sets(Update, PausableSystems.run_if(in_state(Pause(false))));
    player::plugin(app);
    bgm::plugin(app);
    background::plugin(app);
    end::plugin(app);
    failed::plugin(app);
    tutorials::plugin(app);
    cat::plugin(app);
    cheese::plugin(app);
    escape::plugin(app);
}
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum PostPhysicsAppSystems {
    /// Tick timers.
    TickTimers,
    /// Change UI.
    ChangeUi,
    /// Play sounds.
    PlaySounds,
    /// Play animations.
    PlayAnimations,
    /// Do everything else (consider splitting this into further variants).
    Update,
}

#[derive(
    SubStates, Debug, Hash, PartialEq, Eq, Clone, Default, Reflect, strum_macros::EnumIter,
)]
#[source(Screen = Screen::Gameplay)]
pub enum GameState {
    #[default]
    Tutorial,
    Prepare,
    Start,
    UIOpened,
    Succeeding,
    Failed,
    Win,
}

/// Whether or not the game is paused.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
pub(crate) struct Pause(pub(crate) bool);

/// A system set for systems that shouldn't run while the game is paused.
#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub(crate) struct PausableSystems;
