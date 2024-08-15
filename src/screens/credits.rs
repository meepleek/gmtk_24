//! A credits screen that can be accessed from the title screen.

use crate::prelude::*;

use super::trigger_transition_to_main_menu;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Credits), show_credits_screen);
}

fn show_credits_screen(mut commands: Commands) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Credits))
        .with_children(|children| {
            children.header("Made by");
            children.label("Joe Shmoe - Implemented aligator wrestling AI");
            children.label("Jane Doe - Made the music for the alien invasion");

            children.header("Assets");
            children.label("Bevy logo - All rights reserved by the Bevy Foundation. Permission granted for splash screen use when unmodified.");
            children.label("Ducky sprite - CC0 by Caz Creates Games");
            children.label("Button SFX - CC0 by Jaszunio15");
            children.label("Music - CC BY 3.0 by Kevin MacLeod");

            children.button("Back").observe(trigger_transition_to_main_menu);
        });
}
