//! A credits screen that can be accessed from the title screen.

use crate::prelude::*;

use super::trigger_transition_to_main_menu;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Settings), show_screen);
}

fn show_screen(mut commands: Commands) {
    commands
        .ui_root()
        // todo: replace by ScreenTransition scoped instead?
        .insert(StateScoped(Screen::Settings))
        .with_children(|children| {
            children.header("Settings");
            children.label("TODO");
            children
                .button("Back")
                .observe(trigger_transition_to_main_menu);
        });
}
