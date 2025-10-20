//! A credits screen that can be accessed from the title screen.

use crate::prelude::*;

use super::trigger_transition_to_main_menu;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Settings), show_screen);
}

fn show_screen(mut commands: Commands) {
    commands.spawn((
        ui_root("settings"),
        DespawnOnExit(Screen::Settings),
        children![
            header("Settings"),
            label("TODO"),
            button("Back", trigger_transition_to_main_menu)
        ],
    ));
}
