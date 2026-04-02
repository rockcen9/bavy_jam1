// use crate::game_manager::monster::MonsterState;
use crate::prelude::*;
use kira_ext::BGMEvent;

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), play_prepare);
    app.add_systems(OnEnter(GameState::Prepare), play_prepare);
    app.add_systems(OnEnter(GameState::Start), play_start);
    app.add_systems(OnEnter(GameState::Win), play_end);
}
fn play_prepare(mut commands: Commands) {
    commands.trigger(BGMEvent::new("prepare"));
}
fn play_start(mut commands: Commands) {
    commands.trigger(BGMEvent::new("start"));
}

fn play_end(mut commands: Commands) {
    commands.trigger(BGMEvent::new("end"));
}
