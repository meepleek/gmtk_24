use crate::prelude::*;

use super::{
    trigger_transition_to_credits, trigger_transition_to_game, trigger_transition_to_settings,
    trigger_transition_to_tutorial,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::MainMenu), show_title_screen);
}

fn show_title_screen(mut cmd: Commands) {
    cmd.spawn((
        ui_root("title"),
        DespawnOnExit(Screen::MainMenu),
        children![
            button("Play", trigger_transition_to_game),
            button("Tutorial", trigger_transition_to_tutorial),
            button("Settings", trigger_transition_to_settings),
            button("Credits", trigger_transition_to_credits),
            #[cfg(not(target_family = "wasm"))]
            button("Exit", exit_app),
        ],
    ));

    cmd.play_music(MusicTrack::MainMenu);
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_trigger: On<Press>, mut app_exit: MessageWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}
