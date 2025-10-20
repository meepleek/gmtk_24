//! A credits screen that can be accessed from the title screen.

use crate::prelude::*;

use super::trigger_transition_to_game;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Tutorial), show_screen);
}

fn show_screen(mut commands: Commands) {
    commands.spawn((
        ui_root("tutorial"),
        DespawnOnExit(Screen::Tutorial),
        children![
            header("How to play"),
            label("TODO"),
            button("Play", trigger_transition_to_game),
        ],
    ));
}
