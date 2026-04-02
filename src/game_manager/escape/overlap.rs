use kira_ext::SFXEvent;

use crate::prelude::*;

const OVERLAP_DISTANCE: f32 = 64.0;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, check_escape_overlap.in_set(PausableSystems));
}

fn check_escape_overlap(
    mouse_hand: Query<&Transform, With<MouseHand>>,
    escape_points: Query<&Transform, With<EscapePoint>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
) {
    let Ok(hand_transform) = mouse_hand.single() else {
        return;
    };
    let hand_pos = hand_transform.translation.truncate();

    for escape_transform in &escape_points {
        let escape_pos = escape_transform.translation.truncate();
        if hand_pos.distance(escape_pos) <= OVERLAP_DISTANCE {
            next_state.set(GameState::Win);
            commands.trigger(SFXEvent::ui("put"));

            return;
        }
    }
}
