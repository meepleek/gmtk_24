//! A credits screen that can be accessed from the title screen.

use crate::prelude::*;

use super::trigger_transition_to_game;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Tutorial), show_screen);
}

fn show_screen(mut commands: Commands) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Tutorial))
        .with_children(|children| {
            children.header("How to play");
            children.label("TODO");
            children.button("Play").observe(trigger_transition_to_game);
        });
}
