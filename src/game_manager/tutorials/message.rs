use crate::prelude::*;
use anim_ui::IsDespawning;
use bevy_intl::I18n;
use bevy_tweening::{AnimTarget, TweenAnim, lens::TextColorLens, *};
use std::time::Duration;

const DISPLAY_SECS: f32 = 2.0;
const FADE_DURATION_MS: u64 = 500;
const MSG_Z: f32 = 50.0;

#[derive(Component)]
struct HandDetectingMessage;

#[derive(Resource, Default)]
struct HandDetectingMsgState {
    elapsed: f32,
    fading: bool,
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<HandDetectingMsgState>();
    app.add_systems(OnEnter(GameState::Tutorial), spawn_message);
    app.add_systems(Update, tick_message);
}

fn spawn_message(
    mut commands: Commands,
    font_handle: Res<FontHandle>,
    i18n: Res<I18n>,
    mut state: ResMut<HandDetectingMsgState>,
) {
    *state = HandDetectingMsgState::default();
    let font = font_handle.get(&i18n);
    commands.spawn((
        HandDetectingMessage,
        Text2d::new("Hand gestures detecting..."),
        TextFont {
            font,
            font_size: FontSize::Px(64.0),
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, 0.0, MSG_Z),
        DespawnOnExit(Screen::Gameplay),
    ));
}

fn tick_message(
    mut commands: Commands,
    time: Res<Time>,
    mut state: ResMut<HandDetectingMsgState>,
    query: Query<Entity, With<HandDetectingMessage>>,
) {
    if state.fading {
        return;
    }
    state.elapsed += time.delta_secs();
    if state.elapsed < DISPLAY_SECS {
        return;
    }
    state.fading = true;
    let Ok(entity) = query.single() else { return };
    let tween = Tween::new(
        EaseFunction::QuadraticIn,
        Duration::from_millis(FADE_DURATION_MS),
        TextColorLens {
            start: Color::WHITE,
            end: Color::srgba(1.0, 1.0, 1.0, 0.0),
        },
    );
    commands.entity(entity).insert(IsDespawning);
    commands.spawn((
        TweenAnim::new(tween),
        AnimTarget::component::<TextColor>(entity),
    ));
}
