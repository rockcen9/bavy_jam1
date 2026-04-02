use bevy::{
    ecs::template::template,
    scene2::{CommandsSceneExt, bsn},
};

use crate::prelude::*;

/// Marks the mouse entity that follows the player's right hand after pickup.
#[derive(Component, Reflect, Default, FromTemplate)]
pub struct MouseHand;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Start), spawn_mouse_hand);
    app.add_systems(Update, sync_mouse_hand.in_set(PausableSystems));
}

fn spawn_mouse_hand(mut commands: Commands, right_hand: Query<&Transform, With<RightHand>>) {
    let mouse_pos = right_hand
        .single()
        .map(|t| t.translation.truncate())
        .unwrap_or_default();

    let scene = bsn! {
        #MouseHand
        MouseHand
        template(|context| {
            Ok(Sprite {
                image: context
                    .resource::<AssetServer>()
                    .load("textures/mouse.png").into(),
                ..default()
            })
        })
        Transform::from_xyz(mouse_pos.x, mouse_pos.y, 0.0)
        SpriteLayer::MouseHand
        DespawnOnExit::<GameState>(GameState::Start)
    };

    commands.spawn_scene(scene);
}

fn sync_mouse_hand(
    right_hand: Query<&Transform, (With<RightHand>, Without<MouseHand>)>,
    mut mouse_hand: Query<&mut Transform, With<MouseHand>>,
) {
    let Ok(rh_transform) = right_hand.single() else {
        return;
    };
    let Ok(mut mh_transform) = mouse_hand.single_mut() else {
        return;
    };
    let rh_pos = rh_transform.translation.truncate();
    let prev_pos = mh_transform.translation.truncate();
    let delta = rh_pos - prev_pos;

    mh_transform.translation.x = rh_pos.x;
    mh_transform.translation.y = rh_pos.y;

    if delta.length_squared() > 1.0 {
        let angle = delta.y.atan2(delta.x);
        let target = Quat::from_rotation_z(angle + std::f32::consts::PI);
        mh_transform.rotation = mh_transform.rotation.slerp(target, 0.3);
    }
}
