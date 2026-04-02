use crate::{prelude::*, theme};

mod wake;

// Cat is at world (0, 460) → UI (960, 80). Cat left edge = 928px.
// Midpoint between cat left edge and screen left = 464px.
// Bar width 600px, height 32px, left = 464 - 300 = 164px, top = 80 - 16 = 64px.
// All values as % of 1920x1080.
const BAR_WIDTH_PCT: f32 = 600.0 / 1920.0 * 100.0;
const BAR_HEIGHT_PCT: f32 = 32.0 / 1080.0 * 100.0;
const BAR_LEFT_PCT: f32 = 164.0 / 1920.0 * 100.0;
const BAR_TOP_PCT: f32 = 64.0 / 1080.0 * 100.0;
const BAR_BG_COLOR: Color = Color::srgba(0.15, 0.15, 0.15, 0.6);
const BAR_FILL_COLOR: Color = Color::srgb_u8(0xff, 0xec, 0xf0);

#[derive(Resource, Reflect)]
pub struct SleepResource {
    pub value: f32,
}

impl Default for SleepResource {
    fn default() -> Self {
        Self { value: 100.0 }
    }
}

impl SleepResource {
    pub fn min() -> f32 {
        0.0
    }

    pub fn max() -> f32 {
        100.0
    }

    pub fn set(&mut self, value: f32) {
        self.value = value.clamp(Self::min(), Self::max());
    }
}

#[derive(Component, Default, FromTemplate)]
struct SleepBarFill;

pub(crate) fn plugin(app: &mut App) {
    app.init_resource::<SleepResource>();
    app.add_systems(OnEnter(GameState::Prepare), refill_sleep);
    app.add_systems(OnEnter(GameState::Start), spawn_sleep_bar);
    app.add_systems(Update, update_sleep_bar);
    wake::plugin(app);
}

fn refill_sleep(mut sleep: ResMut<SleepResource>) {
    sleep.set(SleepResource::max());
}

fn spawn_sleep_bar(mut commands: Commands) {
    let scene = bsn! {
        #SleepBar
        Node {
                position_type: PositionType::Absolute,
                top: Val::Percent(BAR_TOP_PCT),
                left: Val::Percent(BAR_LEFT_PCT),
                width: Val::Percent(BAR_WIDTH_PCT),
                height: Val::Percent(BAR_HEIGHT_PCT),
            }
            BackgroundColor(BAR_BG_COLOR)
            DespawnOnExit::<GameState>(GameState::Start)
            Children[
                (
                Name::new("SleepBarFill")
                SleepBarFill
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                }
                BackgroundColor(BAR_FILL_COLOR)
            )
            ]
    };
    commands.spawn_scene(scene);
}

fn update_sleep_bar(
    sleep: Res<SleepResource>,
    mut prev: Local<f32>,
    mut fill_query: Query<&mut Node, With<SleepBarFill>>,
) {
    if sleep.value == *prev {
        return;
    }
    *prev = sleep.value;

    let pct = (sleep.value / SleepResource::max()) * 100.0;
    for mut node in &mut fill_query {
        node.width = Val::Percent(pct);
    }
}
