use anim_ui::IsDespawning;
use kira_ext::SFXEvent;

use crate::prelude::*;

const EAT_DISTANCE: f32 = 64.0;

#[derive(Resource, Debug, Default, Reflect)]
pub struct EatCount(pub u32);

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<EatCount>();
    app.add_systems(Update, eat_cheese.in_set(PausableSystems));
}

fn eat_cheese(
    mut commands: Commands,
    mouse_hand: Query<&Transform, With<MouseHand>>,
    cheeses: Query<(Entity, &Transform), (With<Cheese>, Without<IsDespawning>)>,
    mut eat_count: ResMut<EatCount>,
) {
    let Ok(hand_transform) = mouse_hand.single() else {
        return;
    };
    let hand_pos = hand_transform.translation.truncate();

    for (cheese_entity, cheese_transform) in &cheeses {
        let cheese_pos = cheese_transform.translation.truncate();
        if hand_pos.distance(cheese_pos) <= EAT_DISTANCE {
            eat_count.0 += 1;
            commands.trigger(SFXEvent::sfx("eating"));
            commands.entity(cheese_entity).insert(AnimDespawn);
        }
    }
}
