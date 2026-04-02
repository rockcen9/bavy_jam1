use bevy::{
    ecs::template::template,
    scene2::{CommandsSceneExt, bsn, bsn_list},
};

use crate::prelude::*;

// ── Constants ────────────────────────────────────────────────────────────────

const FADE_DURATION_SECS: f32 = 1.5;
const SHOW_END_FADE_SECS: f32 = 2.5;
/// Delay after fade-to-black before credits fade in.
const SHOW_END_DELAY_SECS: f32 = 1.2;

const CREDITS_TITLE_SIZE: f32 = 80.0;
const CREDITS_SUBTITLE_SIZE: f32 = 30.0;
const CREDITS_GAP: f32 = 18.0;

// ── Markers ──────────────────────────────────────────────────────────────────

#[derive(Component, FromTemplate)]
struct EndOverlay;

#[derive(Component, Clone, Copy, FromTemplate)]
struct EndCreditsTitle;

#[derive(Component, FromTemplate)]
struct EndCreditsSubtitle;

// ── Resources ────────────────────────────────────────────────────────────────

#[derive(Resource)]
struct EndSequence {
    phase: EndPhase,
}

enum EndPhase {
    FadeToBlack { elapsed: f32 },
    ShowEnd { elapsed: f32 },
    Done,
}

// ── Plugin ───────────────────────────────────────────────────────────────────

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Win), spawn_end_sequence)
        .add_systems(Update, tick_end_sequence.run_if(in_state(GameState::Win)));
}

// ── Systems ──────────────────────────────────────────────────────────────────

fn spawn_end_sequence(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    palette: Res<crate::theme::palette::ColorPalette>,
) {
    let overlay = bsn! {
            #EndOverlay
            EndOverlay
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
            }
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0))
            ZIndex(1000)
    };

    let quicksand: &str = "fonts/Quicksand-Regular.ttf";
    let color = palette.blush.with_alpha(0.0);

    let title = bsn!(
        #EndCreditsTitle
        EndCreditsTitle
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(50.0),
            left: Val::Percent(0.0),
            right: Val::Percent(0.0),
            width: Val::Percent(100.0),
        }
        Text::new("Congratulations!")
        TextLayout::new_with_justify(Justify::Center)
        template(move |context| {
            Ok(TextFont {
                font: context
                    .resource::<AssetServer>()
                    .load(quicksand).into(),
                font_size: FontSize::Px(CREDITS_TITLE_SIZE),
                ..default()
            })
        })

        TextColor(color)
        ZIndex(1001)
    );

    let top = 540.0 + CREDITS_TITLE_SIZE + CREDITS_GAP;
    let color = palette.rose_petal.with_alpha(0.0);
    let sub_title = bsn! {
        #EndCreditsSubtitle
        EndCreditsSubtitle
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(top),
            left: Val::Percent(0.0),
            right: Val::Percent(0.0),
            width: Val::Percent(100.0)
        }
        Text::new("It's a short one, but thanks for giving it a try!")
        TextLayout::new_with_justify(Justify::Center)
         template(move |context| {
            Ok(TextFont {
                font: context
                    .resource::<AssetServer>()
                    .load(quicksand).into(),
                font_size: FontSize::Px(CREDITS_SUBTITLE_SIZE),
                ..default()
            })
        })

        TextColor(color)
        ZIndex(1001)
    };

    let list = bsn_list![overlay, title, sub_title];
    commands.spawn_scene_list(list);
    commands.insert_resource(EndSequence {
        phase: EndPhase::FadeToBlack { elapsed: 0.0 },
    });
}

fn tick_end_sequence(
    time: Res<Time>,
    mut sequence: ResMut<EndSequence>,
    mut overlay: Query<&mut BackgroundColor, With<EndOverlay>>,
    mut title_color: Query<&mut TextColor, (With<EndCreditsTitle>, Without<EndCreditsSubtitle>)>,
    mut subtitle_color: Query<&mut TextColor, (With<EndCreditsSubtitle>, Without<EndCreditsTitle>)>,
) {
    let dt = time.delta_secs();

    match &mut sequence.phase {
        EndPhase::FadeToBlack { elapsed } => {
            *elapsed += dt;
            let alpha = (*elapsed / FADE_DURATION_SECS).clamp(0.0, 1.0);
            if let Ok(mut bg) = overlay.single_mut() {
                bg.0 = Color::srgba(0.0, 0.0, 0.0, alpha);
            }
            if *elapsed >= FADE_DURATION_SECS {
                sequence.phase = EndPhase::ShowEnd { elapsed: 0.0 };
            }
        }
        EndPhase::ShowEnd { elapsed } => {
            *elapsed += dt;
            let fade_t = ((*elapsed - SHOW_END_DELAY_SECS) / SHOW_END_FADE_SECS).clamp(0.0, 1.0);
            if let Ok(mut color) = title_color.single_mut() {
                color.set_if_neq(TextColor(color.0.with_alpha(fade_t)));
            }
            if let Ok(mut color) = subtitle_color.single_mut() {
                color.set_if_neq(TextColor(color.0.with_alpha(fade_t)));
            }
            if *elapsed >= SHOW_END_DELAY_SECS + SHOW_END_FADE_SECS {
                sequence.phase = EndPhase::Done;
            }
        }
        EndPhase::Done => {}
    }
}
