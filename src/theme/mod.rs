//! Reusable UI widgets & theming.

pub mod interaction;
pub mod palette;
mod widgets;

#[allow(dead_code, unused_imports)]
pub mod prelude {
    pub use super::{
        interaction::{InteractionPalette, Pressed},
        palette as ui_palette,
        widgets::*,
    };
}

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(interaction::plugin);
}
