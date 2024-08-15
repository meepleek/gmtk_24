//! A loading screen during which game assets are loaded.
//! This reduces stuttering, especially for audio on WASM.

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Loading), show_loading_screen);
}

fn show_loading_screen(mut commands: Commands) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Loading))
        .with_children(|children| {
            children.label("Loading...");
        });
}
