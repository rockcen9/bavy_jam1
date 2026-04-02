use crate::prelude::*;

/// Pixels-per-second speed above which hand movement is considered "noisy".
const NOISE_SPEED_THRESHOLD: f32 = 300.0;

/// How much SleepResource is drained per second when moving at max excess speed.
const MAX_DRAIN_PER_SEC: f32 = 20.0;

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(Update, drain_sleep_on_fast_hand.in_set(PausableSystems));
}

fn drain_sleep_on_fast_hand(
    time: Res<Time>,
    mouse_hand: Query<&Transform, With<MouseHand>>,
    mut prev_pos: Local<Option<Vec2>>,
    mut sleep: ResMut<SleepResource>,
) {
    let Ok(transform) = mouse_hand.single() else {
        *prev_pos = None;
        return;
    };

    let current = transform.translation.truncate();

    if let Some(prev) = *prev_pos {
        let dt = time.delta_secs();
        if dt > 0.0 {
            let speed = current.distance(prev) / dt;
            let excess = (speed - NOISE_SPEED_THRESHOLD).max(0.0);
            if excess > 0.0 {
                let drain = (excess / NOISE_SPEED_THRESHOLD) * MAX_DRAIN_PER_SEC * dt;
                let new_value = sleep.value - drain;
                sleep.set(new_value);
            }
        }
    }

    *prev_pos = Some(current);
}
