use kira_ext::SFXEvent;

use crate::loading_bar::{LOADING_BAR_RING_SIZE_PX, LoadingBarMaterial};
use crate::prelude::*;

use super::{MouseHand, MouseStart};

const MOUSE_X: f32 = 480.0;
const MOUSE_Y: f32 = 200.0;
const HOVER_SIZE: f32 = 64.0;
const HOVER_DURATION_SECS: f32 = 1.0;
const LB_Z: f32 = 45.0; // matches SpriteLayer::LoadingBar

#[derive(Component)]
struct MouseStartLoadingBar;

#[derive(Resource, Default, Reflect)]
pub struct MouseHoverState {
    elapsed: f32,
    active: bool,
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<MouseHoverState>();
    app.add_systems(OnEnter(GameState::Prepare), spawn_mouse);
    app.add_systems(
        Update,
        (tick_mouse_hover, sync_mouse_loading_bar).chain(), // .in_set(PausableSystems),
    );
}

fn spawn_mouse(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut lb_materials: ResMut<Assets<LoadingBarMaterial>>,
) {
    commands.spawn((
        MouseStart,
        Sprite {
            image: asset_server.load("textures/mouse.png"),
            color: Color::BLACK,
            ..default()
        },
        Transform::from_xyz(MOUSE_X, MOUSE_Y, 0.0),
        SpriteLayer::MouseStart,
        DespawnOnExit::<GameState>(GameState::Prepare),
    ));

    commands.spawn((
        Name::new("MouseStartLoadingBar"),
        MouseStartLoadingBar,
        SpriteLayer::LoadingBar,
        Mesh2d(meshes.add(Rectangle::new(
            LOADING_BAR_RING_SIZE_PX,
            LOADING_BAR_RING_SIZE_PX,
        ))),
        MeshMaterial2d(lb_materials.add(LoadingBarMaterial::default())),
        Transform::from_xyz(MOUSE_X, MOUSE_Y, LB_Z),
        Visibility::Hidden,
        DespawnOnExit::<GameState>(GameState::Prepare),
    ));
}

fn tick_mouse_hover(
    mouse_hand_exists: Query<(), With<MouseHand>>,
    mouse_start: Query<&Transform, With<MouseStart>>,
    right_hand: Query<(&Transform, &Visibility), With<RightHand>>,
    mut state: ResMut<MouseHoverState>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if !mouse_hand_exists.is_empty() {
        return;
    }

    let Ok(mouse_transform) = mouse_start.single() else {
        return;
    };
    let mouse_pos = mouse_transform.translation.truncate();
    let half = HOVER_SIZE / 2.0;

    let is_hovering = right_hand.iter().any(|(transform, visibility)| {
        if *visibility == Visibility::Hidden {
            return false;
        }
        let hand_pos = transform.translation.truncate();
        (hand_pos.x - mouse_pos.x).abs() <= half && (hand_pos.y - mouse_pos.y).abs() <= half
    });

    if is_hovering {
        if !state.active {
            state.active = true;
            state.elapsed = 0.0;
        }
        state.elapsed += time.delta_secs();
        if state.elapsed >= HOVER_DURATION_SECS {
            *state = MouseHoverState::default();

            commands.trigger(SFXEvent::ui("put"));
            next_game_state.set(GameState::Start);
        }
    } else {
        state.active = false;
        state.elapsed = 0.0;
    }
}

fn sync_mouse_loading_bar(
    state: Res<MouseHoverState>,
    loading_bar: Query<(&MeshMaterial2d<LoadingBarMaterial>, Entity), With<MouseStartLoadingBar>>,
    mut materials: ResMut<Assets<LoadingBarMaterial>>,
    mut visibility: Query<&mut Visibility>,
) {
    let Ok((mat_handle, entity)) = loading_bar.single() else {
        return;
    };
    let Ok(mut vis) = visibility.get_mut(entity) else {
        return;
    };

    if state.active {
        vis.set_if_neq(Visibility::Visible);
        if let Some(mut mat) = materials.get_mut(&mat_handle.0) {
            mat.params.x = (state.elapsed / HOVER_DURATION_SECS).clamp(0.0, 1.0);
        }
    } else {
        vis.set_if_neq(Visibility::Hidden);
    }
}
