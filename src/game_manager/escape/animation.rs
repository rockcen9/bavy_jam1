use crate::prelude::*;
use bevy_tweening::{lens::TransformScaleLens, *};
use std::time::Duration;

const PULSE_SCALE: f32 = 1.35;
const PULSE_DURATION_MS: u64 = 600;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, start_pulse_on_added);
}

fn start_pulse_on_added(query: Query<Entity, Added<EscapePoint>>, mut commands: Commands) {
    for entity in &query {
        let tween = Tween::new(
            EaseFunction::SineInOut,
            Duration::from_millis(PULSE_DURATION_MS),
            TransformScaleLens {
                start: Vec3::ONE,
                end: Vec3::splat(PULSE_SCALE),
            },
        )
        .with_repeat(RepeatCount::Infinite, RepeatStrategy::MirroredRepeat);

        commands.spawn((TweenAnim::new(tween), AnimTarget::component::<Transform>(entity)));
    }
}
