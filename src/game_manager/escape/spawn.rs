use crate::prelude::*;

const LEFT_MIDDLE_X: f32 = -700.0;
const LEFT_MIDDLE_Y: f32 = 0.0;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        spawn_escape_point
            .in_set(PausableSystems)
            .run_if(in_state(GameState::Start)),
    );
}

fn spawn_escape_point(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    cheeses: Query<(), With<Cheese>>,
    escape_points: Query<(), With<EscapePoint>>,
    game_state: Res<State<GameState>>,
) {
    if *game_state.get() != GameState::Start {
        return;
    }
    if !cheeses.is_empty() {
        return;
    }
    if !escape_points.is_empty() {
        return;
    }

    let texture = asset_server.load("textures/hat.png");
    commands.spawn((
        EscapePoint,
        Sprite::from_image(texture),
        Transform::from_xyz(LEFT_MIDDLE_X, LEFT_MIDDLE_Y, 0.0),
        SpriteLayer::Item,
        AnimSpawnOn,
        DespawnOnExit::<GameState>(GameState::Start),
    ));
}
