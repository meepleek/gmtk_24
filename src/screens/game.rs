//! The screen state for the main game loop.

use crate::{game::level::SpawnLevel, prelude::*};
use leafwing_input_manager::common_conditions::action_just_pressed;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Game), spawn_level)
        .add_systems(OnExit(Screen::Game), stop_music)
        .add_systems(OnEnter(Screen::RestartGame), enter_restart)
        .add_systems(
            Update,
            (
                return_to_main_menu
                    .run_if(in_state(Screen::Game).and_then(action_just_pressed(UiAction::Back))),
                restart_game
                    .run_if(in_state(Screen::Game).and_then(action_just_pressed(UiAction::Reset))),
            ),
        );
}

fn spawn_level(mut commands: Commands) {
    commands.add(SpawnLevel);
    commands.play_music(MusicTrack::Game);
}

fn stop_music(mut commands: Commands) {
    commands.stop_music();
}

fn return_to_main_menu(mut cmd: Commands) {
    cmd.transition_to_screen(Screen::MainMenu);
}

fn restart_game(mut cmd: Commands) {
    cmd.transition_to_screen(Screen::RestartGame);
}

fn enter_restart(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Game);
}
