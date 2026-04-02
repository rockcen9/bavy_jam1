pub(crate) fn plugin(_app: &mut App) {}

use crate::prelude::*;

// ── Events ────────────────────────────────────────────────────────────────────

/// Send to enable bubble columns on the lake surface.
/// `positions` holds UV-x coordinates (0.0–1.0), up to 8 entries.
#[derive(Event)]
pub(crate) struct StartBubbleEvent {
    pub positions: Vec<f32>,
}

/// Send to disable the bubble effect entirely.
#[derive(Event)]
pub(crate) struct StopBubbleEvent;
