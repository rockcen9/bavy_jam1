use crate::prelude::*;

use super::SleepResource;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        check_sleep_depleted
            .in_set(PausableSystems)
            .run_if(in_state(GameState::Start)),
    );
}

fn check_sleep_depleted(
    sleep: Res<SleepResource>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if sleep.value <= SleepResource::min() && *state.get() == GameState::Start {
        next_state.set(GameState::Failed);
    }
}
