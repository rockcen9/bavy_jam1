use crate::prelude::*;
use rand::RngExt;

const HALF_W: f32 = 1920.0 * 0.6 / 2.0;
const HALF_H: f32 = 1080.0 * 0.6 / 2.0;
const MIN_CHEESE_DIST: f32 = 350.0;
// Exclude the lower 25% of screen height where the cat and obstacles live
const CHEESE_MIN_Y: f32 = -HALF_H * 0.5;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Start), spawn_cheeses);
}

fn spawn_cheeses(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut rng = rand::rng();
    let texture = asset_server.load("textures/cheese.png");
    let mut positions: Vec<Vec2> = Vec::new();

    for _ in 0..5 {
        let pos = loop {
            let x = rng.random_range(-HALF_W..HALF_W);
            let y = rng.random_range(CHEESE_MIN_Y..HALF_H);
            let candidate = Vec2::new(x, y);
            if positions
                .iter()
                .all(|p| p.distance(candidate) >= MIN_CHEESE_DIST)
            {
                break candidate;
            }
        };
        positions.push(pos);
        commands.spawn((
            Cheese,
            Sprite::from_image(texture.clone()),
            Transform::from_xyz(pos.x, pos.y, 0.0),
            SpriteLayer::Cheese,
            AnimSpawnOn,
            DespawnOnExit::<GameState>(GameState::Start),
        ));
    }
}
