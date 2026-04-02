use crate::prelude::*;

mod sleepy;
pub(crate) use sleepy::SleepResource;

pub(crate) fn plugin(app: &mut App) {
    sleepy::plugin(app);
    // app.add_systems(OnEnter(Screen::Gameplay), spawn_cat);
}

// fn spawn_cat(mut commands: Commands, asset_server: Res<AssetServer>) {
//     // commands.spawn((
//     //     Sprite::from_image(asset_server.load("textures/cat.png")),
//     //     Transform::from_xyz(0.0, 460.0, 0.0),
//     //     SpriteLayer::Cat,
//     //     DespawnOnExit::<Screen>(Screen::Gameplay),
//     // ));
// }
