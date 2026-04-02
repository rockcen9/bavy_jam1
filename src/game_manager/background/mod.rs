pub(crate) mod bubble;

use crate::prelude::*;

use bevy::{
    ecs::template::template,
    scene2::{CommandsSceneExt, bsn},
};
pub(crate) use bubble::{StartBubbleEvent, StopBubbleEvent};

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(bubble::plugin);
    app.add_systems(OnEnter(Screen::Title), spawn_background_sprite);
}

fn spawn_background_sprite(mut commands: Commands, asset_server: Res<AssetServer>) {
    let scene = bsn! {
             template(|context| {
                Ok(Sprite {
                    image: context
                        .resource::<AssetServer>()
                        .load("background/night.png").into(),
                    custom_size: Some(Vec2::new(crate::GAME_WIDTH, crate::GAME_HEIGHT)),
                    ..default()
                })
            })
        Transform::from_xyz(0.0, 0.0, -100.0)
        SpriteLayer::Background
        DespawnOnExit::<Screen>(Screen::Gameplay)
    };
    commands.spawn_scene(scene);
}
