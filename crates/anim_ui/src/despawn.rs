use bevy::prelude::*;
use bevy_tweening::{AnimTarget, TweenAnim};

use crate::IsDespawning;

use super::despawn_on::scale_out_tween;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(on_add_anim_despawn);
}

/// Attach to an entity to immediately play a 200 ms scale-out animation and
/// despawn it once the animation finishes.
///
/// # Example
/// ```rust
/// commands.entity(entity).insert(AnimDespawn);
/// ```
#[derive(Component)]
pub struct AnimDespawn;

fn on_add_anim_despawn(trigger: On<Add, AnimDespawn>, mut commands: Commands) {
    let entity = trigger.event_target();
    commands.entity(entity).insert(IsDespawning);
    commands.spawn((
        TweenAnim::new(scale_out_tween()),
        AnimTarget::component::<Transform>(entity),
    ));
}
