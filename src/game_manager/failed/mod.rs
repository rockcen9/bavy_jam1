use crate::prelude::*;
use kira_ext::SFXEvent;

// ── Constants ────────────────────────────────────────────────────────────────

const FADE_OUT_SECS: f32 = 1.5;
const FADE_IN_SECS: f32 = 1.5;

// ── Markers ──────────────────────────────────────────────────────────────────

#[derive(Component)]
struct FailedOverlay;

// ── Resources ────────────────────────────────────────────────────────────────

#[derive(Resource)]
struct FailedSequence {
    phase: FailedPhase,
}

enum FailedPhase {
    FadeToBlack { elapsed: f32 },
    FadeIn { elapsed: f32 },
    Done,
}

// ── Plugin ───────────────────────────────────────────────────────────────────

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Failed), (spawn_failed_sequence, play_meow))
        .add_systems(
            Update,
            tick_failed_sequence.run_if(in_state(GameState::Failed)),
        );
}

// ── Systems ──────────────────────────────────────────────────────────────────

fn play_meow(mut commands: Commands) {
    commands.trigger(SFXEvent::sfx("meow"));
}

fn spawn_failed_sequence(mut commands: Commands) {
    commands.spawn((
        Name::new("FailedOverlay"),
        FailedOverlay,
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)),
        ZIndex(1000),
        DespawnOnExit::<GameState>(GameState::Failed),
    ));

    commands.insert_resource(FailedSequence {
        phase: FailedPhase::FadeToBlack { elapsed: 0.0 },
    });
}

fn tick_failed_sequence(
    time: Res<Time>,
    mut sequence: ResMut<FailedSequence>,
    mut overlay: Query<&mut BackgroundColor, With<FailedOverlay>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let dt = time.delta_secs();

    match &mut sequence.phase {
        FailedPhase::FadeToBlack { elapsed } => {
            *elapsed += dt;
            let alpha = (*elapsed / FADE_OUT_SECS).clamp(0.0, 1.0);
            if let Ok(mut bg) = overlay.single_mut() {
                bg.0 = Color::srgba(0.0, 0.0, 0.0, alpha);
            }
            if *elapsed >= FADE_OUT_SECS {
                sequence.phase = FailedPhase::FadeIn { elapsed: 0.0 };
            }
        }
        FailedPhase::FadeIn { elapsed } => {
            *elapsed += dt;
            let alpha = (1.0 - (*elapsed / FADE_IN_SECS)).clamp(0.0, 1.0);
            if let Ok(mut bg) = overlay.single_mut() {
                bg.0 = Color::srgba(0.0, 0.0, 0.0, alpha);
            }
            if *elapsed >= FADE_IN_SECS {
                sequence.phase = FailedPhase::Done;
                next_state.set(GameState::Prepare);
            }
        }
        FailedPhase::Done => {}
    }
}
