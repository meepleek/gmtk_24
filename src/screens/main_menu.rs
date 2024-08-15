use crate::prelude::*;

use super::{
    trigger_transition_to_credits, trigger_transition_to_game, trigger_transition_to_settings,
    trigger_transition_to_tutorial,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::MainMenu), show_title_screen);
}

fn show_title_screen(mut cmd: Commands) {
    cmd.ui_root()
        .insert(StateScoped(Screen::MainMenu))
        .with_children(|children| {
            children.button("Play").observe(trigger_transition_to_game);
            children
                .button("Tutorial")
                .observe(trigger_transition_to_tutorial);
            children
                .button("Settings")
                .observe(trigger_transition_to_settings);
            children
                .button("Credits")
                .observe(trigger_transition_to_credits);

            #[cfg(not(target_family = "wasm"))]
            children.button("Exit").observe(exit_app);
        });

    cmd.play_music(MusicTrack::MainMenu);
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_trigger: Trigger<OnPress>, mut app_exit: EventWriter<AppExit>) {
    app_exit.send(AppExit::Success);
}
