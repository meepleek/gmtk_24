//! A credits screen that can be accessed from the title screen.

use crate::prelude::*;

use super::trigger_transition_to_main_menu;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Credits), show_credits_screen);
}

fn show_credits_screen(mut commands: Commands) {
    commands.spawn((ui_root("credits"),
        DespawnOnExit(Screen::Credits),
        children![
            header("Made by"),
            label("Joe Shmoe - Implemented aligator wrestling AI"),
            label("Jane Doe - Made the music for the alien invasion"),

            header("Assets"),
            label("Bevy logo - All rights reserved by the Bevy Foundation. Permission granted for splash screen use when unmodified."),
            label("Ducky sprite - CC0 by Caz Creates Games"),
            label("Button SFX - CC0 by Jaszunio15"),
            label("Music - CC BY 3.0 by Kevin MacLeod"),

            button("Back", trigger_transition_to_main_menu),
        ]));
}
